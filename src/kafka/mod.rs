use crate::{Decode, Encode, Size};
use encode_derive::{Decode, Size};

#[derive(Debug, Encode, Decode, Size)]
pub struct BaseRequest {
    pub size: i32,
    pub api_key: i16,
    pub api_versions: i16,
    pub correlation_id: i32,
}

#[derive(Debug, Encode, Decode, Size)]
pub struct BaseRequestV2 {
    pub size: i32,
    pub api_key: i16,
    pub api_versions: i16,
    pub correlation_id: i32,
    pub client_id: String,
    pub tag_buffer: u8,
}

#[derive(Debug, Encode, Decode, Size)]
pub struct BaseResponse {
    pub size: i32,
    pub correlation_id: i32,
}

#[derive(Debug, Encode, Decode, Size)]
pub struct BaseResponseV1 {
    pub base: BaseResponse,
    pub tag_buffer: u8,
}

pub mod apiversions;
pub mod listpartitions;

#[cfg(test)]
mod tests {
    use crate::Decode;

    use super::BaseRequestV2;

    #[test]
    fn test_decode_base_request_v2() {
        let test_request = vec![
            0x00, 0x00, 0x00, 0x20, 0x00, 0x4B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0x00, 0x09,
            0x6B, 0x61, 0x66, 0x6B, 0x61, 0x2D, 0x63, 0x6C, 0x69, 0x00, 0x02, 0x04, 0x66, 0x6F,
            0x6F, 0x00, 0x00, 0x00, 0x00, 0x64, 0xFF, 0x00,
        ];

        let mut offset = 0;

        let base_decoded = BaseRequestV2::decode(&test_request[..], &mut offset);
        println!("offset after decode: {offset:?}");
        println!("{:?}", base_decoded.correlation_id);

        assert_eq!(base_decoded.client_id, "kafka-cli")
    }
}
