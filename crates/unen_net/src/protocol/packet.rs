use bincode::{Decode, Encode};
use crc32fast::Hasher;
use thiserror::Error;

pub const PACKET_MAX_SIZE: usize = 1200;
pub const PROTOCOL_ID: [u8; 8] = *b"UEUDP001";
pub const END_CHECK: u32 = 0xFFFFFFFF;

#[derive(Debug, Error)]
pub enum Error {
    #[error("payload too large")]
    PayloadTooLarge,
    #[error("CRC mismatch")]
    CrcMismatch,
    #[error("invalid packet end")]
    InvalidEnd,
    #[error("encode error: {0}")]
    EncodeError(#[from] bincode::error::EncodeError),
    #[error("decode error: {0}")]
    DecodeError(#[from] bincode::error::DecodeError),
}

#[derive(Debug, Clone)]
pub struct Packet {
    data: Vec<u8>,
}

impl Packet {
    pub fn from_data<T: Encode>(value: T) -> Result<Packet, Error> {
        let mut payload = Vec::new();
        bincode::encode_into_std_write(value, &mut payload, bincode::config::standard())?;

        let overhead = 4 + 4; // CRC + end_check
        if payload.len() + overhead > PACKET_MAX_SIZE {
            return Err(Error::PayloadTooLarge);
        }

        let mut crc_input = Vec::new();
        crc_input.extend_from_slice(&PROTOCOL_ID);
        crc_input.extend(&payload);
        crc_input.extend_from_slice(&END_CHECK.to_le_bytes());

        let mut hasher = Hasher::new();
        hasher.update(&crc_input);
        let crc = hasher.finalize();

        let mut buf = Vec::new();
        buf.extend_from_slice(&crc.to_le_bytes());
        buf.extend(&payload);
        buf.extend_from_slice(&END_CHECK.to_le_bytes());

        Ok(Packet { data: buf })
    }

    pub fn to_data<T: Decode<()>>(&self) -> Result<T, Error> {
        if self.data.len() < 8 {
            return Err(Error::InvalidEnd);
        }

        let crc_bytes = &self.data[..4];
        let payload = &self.data[4..self.data.len() - 4];
        let end_bytes = &self.data[self.data.len() - 4..];

        if u32::from_le_bytes(end_bytes.try_into().unwrap()) != END_CHECK {
            return Err(Error::InvalidEnd);
        }

        let mut crc_input = Vec::new();
        crc_input.extend_from_slice(&PROTOCOL_ID);
        crc_input.extend(payload);
        crc_input.extend_from_slice(&END_CHECK.to_le_bytes());

        let mut hasher = Hasher::new();
        hasher.update(&crc_input);
        let crc_calc = hasher.finalize();
        let crc_orig = u32::from_le_bytes(crc_bytes.try_into().unwrap());

        if crc_calc != crc_orig {
            return Err(Error::CrcMismatch);
        }

        let value =
            bincode::decode_from_slice(payload, bincode::config::standard()).map(|(v, _)| v)?;
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LARGE_PACKET_SIZE: usize = PACKET_MAX_SIZE * 2;

    #[test]
    fn test_packet_roundtrip() {
        let original = "hello world".to_string();
        let packet =
            Packet::from_data(original.clone()).expect("should be able to create packet from data");
        let decoded: String = packet
            .to_data()
            .expect("should be able to decode packet back to original data");
        assert_eq!(original, decoded, "decoded value should match original");
    }

    #[test]
    fn test_payload_too_large() {
        let large = vec![0u8; LARGE_PACKET_SIZE]; // exceeds allowed
        let result = Packet::from_data(large);
        assert!(
            matches!(result, Err(Error::PayloadTooLarge)),
            "should return PayloadTooLarge error when data exceeds maximum size"
        );
    }
}
