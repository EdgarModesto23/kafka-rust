use crate::{Decode, Encode};
use encode_derive::{Decode, Encode};

use super::{BaseRequest, BaseResponse};

#[derive(Debug, Encode, Decode)]
pub struct ApiVersionsRequest {
    pub base: BaseRequest,
}

pub struct ApiVersionsResponse {
    pub base: BaseResponse,
}
