use std::{fs::File, io::BufReader, path::Path};

use crate::{Decode, Encode, Size};
use anyhow::Error;
use encode_derive::{Decode, Size};
use serde::Deserialize as Serde_Deserialize;

use super::{BaseRequest, BaseResponse};

#[derive(Debug, Encode, Decode)]
pub struct ApiVersionsRequest {
    pub base: BaseRequest,
}

#[derive(Serde_Deserialize, Debug, Encode, Decode, Size)]
pub struct SupportedVersionsKey {
    pub key: i16,
    pub min: i16,
    pub max: i16,
    pub tagged_fields: u8,
}

#[derive(Debug, Encode, Decode, Size)]
pub struct ApiVersionsResponse {
    pub base: BaseResponse,
    pub error_code: i16,
    pub api_keys: Vec<SupportedVersionsKey>,
    pub throttle_time_ms: i32,
}

fn get_supported_versions<P: AsRef<Path>>(path: P) -> Result<Vec<SupportedVersionsKey>, Error> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    let data: Vec<SupportedVersionsKey> = serde_json::from_reader(reader)?;

    Ok(data)
}

pub fn is_version_supported<P: AsRef<Path>>(
    path: P,
    key: i16,
    version: i16,
) -> Result<bool, Error> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);

    let data: Vec<SupportedVersionsKey> = serde_json::from_reader(reader)?;

    Ok(data
        .iter()
        .filter(|val| val.key == key && (version >= val.min && version <= val.max))
        .last()
        .is_some())
}

impl ApiVersionsRequest {
    pub async fn handle_request(&self) -> Result<ApiVersionsResponse, Error> {
        let base = BaseResponse {
            correlation_id: self.base.correlation_id,
            size: 0,
        };

        let error_code = if is_version_supported(
            "supported_versions.json",
            self.base.api_key,
            self.base.api_versions,
        )
        .unwrap_or(false)
        {
            0
        } else {
            35
        };
        let api_keys = match get_supported_versions("supported_versions.json") {
            Ok(value) => value,
            Err(err) => return Err(err),
        };
        let throttle_time_ms = 0;

        let mut response = ApiVersionsResponse {
            base,
            error_code,
            api_keys,
            throttle_time_ms,
        };

        let res_size = response.size_in_bytes();
        response.base.size = res_size as i32;

        Ok(response)
    }
}
