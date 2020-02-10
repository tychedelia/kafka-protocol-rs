use franz_derive::{Encodable, Decodable};
use franz_protocol::{
    Request, Downgrade, NoDowngrade, Upgrade, NoUpgrade, MaybeDowngrade, MaybeUpgrade,
    DowngradeError, RequestType
};

#[derive(Debug)]
pub struct Header;

impl RequestType for Header {
    const KEY: i16 = -1;
    const MAX_VERSION: i16 = 11;
}

#[derive(Debug, Clone, Encodable, Decodable)]
pub struct RequestHeader0 {
    #[franz(Int16)]
    pub api_key: i16,
    #[franz(Int16)]
    pub api_version: i16,
    #[franz(Int32)]
    pub correlation_id: i32,
    // The documentation says this field wasn't introduced until 
    // version 1, but the implementation would appear to refute
    // that, and Kafka will error if it is missing.
    #[franz(String)]
    pub client_id: Option<String>,
}

#[derive(Debug, Clone, Encodable, Decodable)]
pub struct RequestHeader1 {
    #[franz(Int16)]
    pub api_key: i16,
    #[franz(Int16)]
    pub api_version: i16,
    #[franz(Int32)]
    pub correlation_id: i32,
    #[franz(String)]
    pub client_id: Option<String>,
}

#[derive(Debug, Clone, Encodable, Decodable)]
pub struct ResponseHeader0 {
    #[franz(Int32)]
    pub correlation_id: i32,
}

#[derive(Debug, Clone, Encodable, Decodable)]
pub struct ResponseHeader1 {
    #[franz(Int32)]
    pub correlation_id: i32,
}

impl Request for RequestHeader0 {
    type Type = Header;
    const VERSION: i16 = 0;
    type Response = ResponseHeader0;
    type Downgrade = NoDowngrade;
    type Upgrade = Upgrade;
}

impl MaybeUpgrade<RequestHeader0> for Upgrade {
    type Request = RequestHeader1;
    type Response = ResponseHeader1;

    fn upgrade(req: RequestHeader0) -> RequestHeader1 {
        RequestHeader1 {
            api_key: req.api_key,
            api_version: req.api_version,
            correlation_id: req.correlation_id,
            client_id: req.client_id,
        }
    }
    fn downgrade_response(other: ResponseHeader1) -> Result<ResponseHeader0, DowngradeError> {
        Ok(ResponseHeader0 {
            correlation_id: other.correlation_id,
        })
    }
}

impl Request for RequestHeader1 {
    type Type = Header;
    const VERSION: i16 = 1;
    type Response = ResponseHeader1;
    type Downgrade = Downgrade;
    type Upgrade = NoUpgrade;
}

impl MaybeDowngrade<RequestHeader1> for Downgrade {
    type Request = RequestHeader0;
    type Response = ResponseHeader0;

    fn downgrade(req: RequestHeader1) -> Result<RequestHeader0, DowngradeError> {
        Ok(RequestHeader0 {
            api_key: req.api_key,
            api_version: req.api_version,
            correlation_id: req.correlation_id,
            client_id: req.client_id,
        })
    }
    fn upgrade_response(other: ResponseHeader0) -> ResponseHeader1 {
        ResponseHeader1 {
            correlation_id: other.correlation_id,
        }
    }
}
