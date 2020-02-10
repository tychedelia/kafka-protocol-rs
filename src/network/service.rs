use std::fmt;
use std::time::Duration;
use std::marker::PhantomData;

use tokio::sync::{oneshot, mpsc};
use tokio::time::{Elapsed, timeout};
use futures::stream::{Stream, StreamExt};
use futures::sink::{Sink, SinkExt};
use futures::future::{self, Future, FutureExt, BoxFuture, TryFutureExt, RemoteHandle};
use async_trait::async_trait;

pub type RecvFuture<Res, Err> = BoxFuture<'static, Result<Res, Err>>;
pub type SendFuture<'a, Res, Err> = BoxFuture<'a, Result<RecvFuture<Res, Err>, Err>>;
pub type BoxService<'a, Req, Res, Err> = Box<dyn Service<Req, Response=Res, Error=Err> + Send + 'a>;

pub trait ServiceBase {}

#[async_trait]
pub trait Service<Req: Send + 'static>: ServiceBase {
    type Response: Send + 'static;
    type Error: Send + 'static;

    fn send<'a>(&'a mut self, request: Req) -> SendFuture<'a, Self::Response, Self::Error> where Self: 'a;
}

impl<'b, Req, Res, Err> ServiceBase for BoxService<'b, Req, Res, Err> {}

impl<'b, Req, Res, Err> Service<Req> for BoxService<'b, Req, Res, Err>
where
    Req: Send + 'static,
    Res: Send + 'static,
    Err: Send + 'static,
{
    type Response = Res;
    type Error = Err;

    fn send<'a>(&'a mut self, request: Req) -> SendFuture<'a, Self::Response, Self::Error> where Self: 'a {
        (**self).send(request)
    }
}

pub struct Pipeline<St: Stream, Si, Err> {
    sender: mpsc::Sender<oneshot::Sender<St::Item>>,
    sink: Si,
    phantom: PhantomData<fn() -> Err>,
}

impl<St: Stream, Si, Err> fmt::Debug for Pipeline<St, Si, Err> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Pipeline { .. }")
    }
}

impl<St: Stream, Si, Err> Pipeline<St, Si, Err>
where
    St: Send + 'static,
    St::Item: Send + 'static,
{
    pub fn new(stream: St, sink: Si, max_concurrency: usize) -> Self {
        let (sender, receiver) = mpsc::channel::<oneshot::Sender<St::Item>>(max_concurrency);
        println!("Starting pipeline...");

        // Process responses in a new task
        tokio::spawn(stream.zip(receiver).for_each(|(res, dst)| {
            dst.send(res).ok();
            future::ready(())
        }).map(|_| println!("Pipeline stopped.")));

        Self { sender, sink, phantom: PhantomData }
    }
}

#[derive(Debug)]
pub struct Shutdown;

impl<St: Stream, Si, Err> ServiceBase for Pipeline<St, Si, Err> {}

#[async_trait]
impl<'a, St, Si, Req, Res, StErr, Err> Service<Req> for Pipeline<St, Si, Err>
where
    St: Stream<Item=Result<Res, StErr>>,
    Si: Sink<Req> + Unpin + Send,
    Req: Send + 'static,
    Res: Send + 'static,
    Err: From<StErr> + From<Si::Error> + From<Shutdown> + Send + 'static,
    Si::Error: Send + 'static,
    StErr: Send + 'static,
{
    type Response = Res;
    type Error = Err;

    async fn send(&'async_trait mut self, request: Req) -> Result<RecvFuture<Res, Err>, Err> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sink.send(request).await?;
        self.sender.send(response_sender).await
            .map_err(|_| Shutdown)?;

        Ok(async move {
            Ok(response_receiver.await
                .map_err(|_| Shutdown)??)
        }.boxed())
    }
}

#[async_trait]
pub trait ServiceExt: ServiceBase {
    async fn call<Req>(&'async_trait mut self, request: Req) -> Result<Self::Response, Self::Error>
    where
        Req: Send + 'static,
        Self: Service<Req>,
    {
        self.send(request).await?.await
    }
    fn timeout(self, duration: Duration) -> TimeoutService<Self>;
    fn map_recv<F>(self, f: F) -> MapRecv<Self, F>;
}

impl<T: ServiceBase + Send> ServiceExt for T {
    fn timeout(self, duration: Duration) -> TimeoutService<Self> {
        TimeoutService {
            inner: self,
            duration,
        }
    }
    fn map_recv<F>(self, f: F) -> MapRecv<Self, F> {
        MapRecv {
            inner: self,
            f,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MapRecv<T: ?Sized, F> {
    f: F,
    inner: T,
}

impl<T: ?Sized, F> ServiceBase for MapRecv<T, F> {}

#[async_trait]
impl<Req, T, F, Res> Service<Req> for MapRecv<T, F>
where
    Req: Send + 'static,
    Res: Send + 'static,
    T: Service<Req> + ?Sized + Send,
    F: FnMut(RecvFuture<T::Response, T::Error>) -> RecvFuture<Res, T::Error> + Send,
{
    type Response = Res;
    type Error = T::Error;

    async fn send(&'async_trait mut self, request: Req) -> Result<RecvFuture<Self::Response, Self::Error>, Self::Error> {
        let res = self.inner.send(request).await?;
        Ok((self.f)(res))
    }
}

#[derive(Debug, Clone)]
pub struct TimeoutService<T: ?Sized> {
    duration: Duration,
    inner: T,
}

impl<T: ?Sized> ServiceBase for TimeoutService<T> {}

#[async_trait]
impl<Req, T> Service<Req> for  TimeoutService<T>
where
    Req: Send + 'static,
    T: Service<Req> + ?Sized + Send,
    T::Error: From<Elapsed>,
{
    type Response = T::Response;
    type Error = T::Error;

    async fn send(&'async_trait mut self, request: Req) -> Result<RecvFuture<Self::Response, Self::Error>, Self::Error> {
        let res = self.inner.send(request).await?;

        Ok(timeout(self.duration, res).unwrap_or_else(|e| Err(e.into())).boxed())
    }
}

enum EagerServiceInner<T, Err> {
    Pending(RemoteHandle<Result<T, Err>>),
    Ready(T),
}

pub struct EagerService<T, Err>(EagerServiceInner<T, Err>);

impl<T, Err> EagerService<T, Err> {
    pub fn new<F>(f: F) -> Self
    where
        F: Future<Output=Result<T, Err>> + Send + 'static,
        T: Send,
        Err: Send,
    {
        let (task, handle) = f.remote_handle();
        tokio::spawn(task);
        EagerService(EagerServiceInner::Pending(handle))
    }
}

impl<T, Err> ServiceBase for EagerService<T, Err> {}

#[async_trait]
impl<Req, T, Err> Service<Req> for EagerService<T, Err>
where
    Req: Send + 'static,
    Err: Send + 'static,
    T: Service<Req, Error=Err> + Send + 'static,
{
    type Response = T::Response;
    type Error = Err;

    async fn send(&'async_trait mut self, request: Req) -> Result<RecvFuture<Self::Response, Self::Error>, Self::Error> {
        match &mut self.0 {
            EagerServiceInner::Pending(future) => {
                let mut service = future.await?;
                let res = service.send(request).await;
                self.0 = EagerServiceInner::Ready(service);
                res
            },
            EagerServiceInner::Ready(service) => {
                service.send(request).await
            },
        }
    }
}
