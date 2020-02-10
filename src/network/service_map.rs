use std::sync::{Weak, Arc};
use std::hash::Hash;
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicI64, Ordering};

use futures::future::FutureExt;
use futures::lock::Mutex;
use chashmap::CHashMap;
use async_trait::async_trait;
use tokio::time::interval;

use super::service::{Service, ServiceBase, RecvFuture};

struct ServiceItem<T> {
    // Milliseconds since the service map "start_time"
    last_activity: AtomicI64,
    service: Mutex<T>,
}

struct ServiceMapInner<K, T> {
    factory: Box<dyn Fn(K) -> T + Send + Sync + 'static>,
    services: CHashMap<K, Arc<ServiceItem<T>>>,
    start_time: Instant,
}

impl<K, T> ServiceMapInner<K, T> {
    fn cull_service(&self, key: K, item: &Arc<ServiceItem<T>>)
    where
        K: PartialEq + Hash,
    {
        self.services.alter(key, |maybe_item| {
            maybe_item.filter(|x| !Arc::ptr_eq(x, item))
        });
    }

    fn now_ms(&self) -> i64 {
        self.start_time.elapsed().as_millis() as i64
    }
    fn get_item(&self, key: &K) -> Arc<ServiceItem<T>>
    where
        K: PartialEq + Hash + Clone,
    {
        // Find or initialize the correct service
        let mut item = None;
        self.services.alter(key.clone(), |maybe_item| {
            item = Some(maybe_item.unwrap_or_else(|| {
                Arc::new(ServiceItem {
                    service: Mutex::new((self.factory)(key.clone())),
                    last_activity: AtomicI64::new(self.now_ms()),
                })
            }));

            item.clone()
        });
        item.expect("Should be initialized")
    }
}

#[derive(Clone)]
pub struct ServiceMap<K, T>(Arc<ServiceMapInner<K, T>>);

// Drop services which have been idle for too long
async fn cull_services<K, T>(inner: Weak<ServiceMapInner<K, T>>) {
    let mut interval = interval(Duration::from_secs(1));
    loop {
        interval.tick().await;

        if let Some(inner) = inner.upgrade() {
            let cutoff = inner.now_ms() - 5000;
            inner.services.retain(|_, v| {
                Arc::strong_count(v) > 1 || v.last_activity.load(Ordering::SeqCst) > cutoff
            })
        } else {
            break;
        }
    }
}

impl<K, T> ServiceMap<K, T>
where
    T: Send + 'static,
    K: Send + Sync + 'static,
{
    pub fn new<F>(factory: F) -> Self
    where
        F: Fn(K) -> T + Send + Sync + 'static,
    {
        Self::new_boxed(Box::new(factory))
    }
    pub fn new_boxed(factory: Box<dyn Fn(K) -> T + Send + Sync + 'static>) -> Self {
        let inner = Arc::new(ServiceMapInner {
            factory,
            services: CHashMap::new(),
            start_time: Instant::now(),
        });

        // Start background culling task
        tokio::spawn(cull_services(Arc::downgrade(&inner)));

        Self(inner)
    }
    pub fn poke(&mut self, key: &K)
    where
        K: PartialEq + Hash + Clone,
    {
        self.0.get_item(key);
    }
}

impl<K, T> ServiceBase for ServiceMap<K, T> {}

#[async_trait]
impl<Req, K, T> Service<(K, Req)> for ServiceMap<K, T>
where
    T: Service<Req> + Send + 'static,
    Req: Send + 'static,
    K: PartialEq + Hash + Clone + Send + Sync + 'static,
{
    type Response = T::Response;
    type Error = T::Error;

    async fn send(&'async_trait mut self, (key, req): (K, Req)) -> Result<RecvFuture<Self::Response, Self::Error>, Self::Error> {
        let inner = &*self.0;

        let item = inner.get_item(&key);

        // Try to send the message to the service
        let recv = match {
            let mut service = item.service.lock().await;
            service.send(req).await
        } {
            Ok(recv) => recv,
            Err(e) => {
                // Sending failed; cull this service immediately
                inner.cull_service(key, &item);
                return Err(e);
            }
        };

        // Sending succeeded, wait for the response
        let inner = self.0.clone();
        Ok(async move {
            match recv.await {
                Ok(res) => {
                    item.last_activity.store(inner.now_ms(), Ordering::SeqCst);
                    Ok(res)
                },
                Err(e) => {
                    // Receiving failed; cull this service immediately
                    inner.cull_service(key, &item);
                    Err(e)
                }
            }
        }.boxed())
    }
}