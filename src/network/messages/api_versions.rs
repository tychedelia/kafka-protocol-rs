use franz_derive::{Encodable, Decodable};
use franz_protocol::{
    Request, Downgrade, NoDowngrade, Upgrade, NoUpgrade, MaybeDowngrade, MaybeUpgrade,
    DowngradeError, RequestType
};

#[derive(Debug)]
pub struct ApiVersions;

impl RequestType for ApiVersions {
    const KEY: i16 = 18;
    const MAX_VERSION: i16 = 1;
}

#[derive(Debug, Clone, Encodable, Decodable)]
pub struct ApiVersions0Request {}

#[derive(Debug, Clone, Encodable, Decodable)]
pub struct ApiVersions1Request {}

#[derive(Debug, Clone, Encodable, Decodable)]
pub struct ApiVersions0Response {
    #[franz(Int16)]
    pub error_code: i16,
    #[franz(Array(Struct))]
    pub api_keys: Option<Vec<ApiKey0>>,
}

#[derive(Debug, Clone, Encodable, Decodable)]
pub struct ApiVersions1Response {
    #[franz(Int16)]
    pub error_code: i16,
    #[franz(Array(Struct))]
    pub api_keys: Option<Vec<ApiKey0>>,
    #[franz(Int32)]
    pub throttle_time_ms: i32,
}

#[derive(Debug, Clone, Encodable, Decodable)]
pub struct ApiKey0 {
    #[franz(Int16)]
    pub api_key: i16,
    #[franz(Int16)]
    pub min_version: i16,
    #[franz(Int16)]
    pub max_version: i16,
}

impl Request for ApiVersions0Request {
    type Type = ApiVersions;
    const VERSION: i16 = 0;
    type Response = ApiVersions0Response;
    type Downgrade = NoDowngrade;
    type Upgrade = Upgrade;
}

impl MaybeUpgrade<ApiVersions0Request> for Upgrade {
    type Request = ApiVersions1Request;
    type Response = ApiVersions1Response;

    fn upgrade(_req: ApiVersions0Request) -> ApiVersions1Request {
        ApiVersions1Request {}
    }
    fn downgrade_response(other: ApiVersions1Response) -> Result<ApiVersions0Response, DowngradeError> {
        Ok(ApiVersions0Response {
            error_code: other.error_code,
            api_keys: other.api_keys,
        })
    }
}

impl Request for ApiVersions1Request {
    type Type = ApiVersions;
    const VERSION: i16 = 1;
    type Response = ApiVersions1Response;
    type Downgrade = Downgrade;
    type Upgrade = NoUpgrade;
}

impl MaybeDowngrade<ApiVersions1Request> for Downgrade {
    type Request = ApiVersions0Request;
    type Response = ApiVersions0Response;

    fn downgrade(_req: ApiVersions1Request) -> Result<ApiVersions0Request, DowngradeError> {
        Ok(ApiVersions0Request {})
    }
    fn upgrade_response(other: ApiVersions0Response) -> ApiVersions1Response {
        ApiVersions1Response {
            error_code: other.error_code,
            api_keys: other.api_keys,
            throttle_time_ms: 0,
        }
    }
}
