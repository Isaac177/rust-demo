use crate::http::{response::ok};
use crate::http::error::ApiResult;
use crate::http::response::ApiJson;

pub async fn live() -> ApiResult<ApiJson<&'static str>>{
    Ok(ok("Your app is live"))
}

pub async fn ready() -> ApiResult<ApiJson<&'static str>> {
    Ok(ok("Ready"))
}
