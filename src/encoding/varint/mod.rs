//! An implementation of varint, which encodes integers to variable-length bytes;
//!

use crate::encoding::zigzag::{ZagZig, ZigZag};
use std::{error::Error, fmt};

/// Maximum length of a varint-encoded 16bit integer
pub const MAX_VARINT_LEN_U16: usize = 3;

/// Maximum length of a varint-encoded 32bit integer
pub const MAX_VARINT_LEN_U32: usize = 5;

/// Maximum length of a varint-encoded 64bit integer
pub const MAX_VARINT_LEN_U64: usize = 10;

const OVERFLOW_U8: u8 = 1 << 7;
const OVERFLOW: u64 = OVERFLOW_U8 as u64;
const MASK: u64 = OVERFLOW - 1;

/// ErrorKind for get decoding
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    /// Size of 7bit nums is greater than 10
    Overflow(usize),
    /// Short of last 7bit num
    ShortOfData(usize),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::Overflow(bytes) => write!(
                f,
                "overflows a 64-bit integer for varint encoding, already read {} bytes",
                bytes,
            ),
            ErrorKind::ShortOfData(bytes) => write!(
                f,
                "not enough data for a 64-bit varint-encoded integer, already {} bytes",
                bytes
            ),
        }
    }
}

impl Error for ErrorKind {}

/// Encodes a u64 value and appends to the buffer
pub fn put_u64(buf: &mut Vec<u8>, mut num: u64) -> usize {
    let mut written = 0;
    while num >= OVERFLOW {
        buf.push(num as u8 | OVERFLOW_U8);
        num >>= 7;
        written += 1;
    }

    buf.push(num as u8);
    written + 1
}

/// Encodes an i64 value and appends to the buffer
pub fn put_i64(buf: &mut Vec<u8>, num: i64) -> usize {
    put_u64(buf, num.zigzag())
}

/// Decodes a u64 from given bytes
pub fn get_u64(data: &[u8]) -> Result<(u64, usize), ErrorKind> {
    let mut read = 0;
    let mut res = 0;
    let mut bit_shift = 0;

    while read < data.len() && read < MAX_VARINT_LEN_U64 {
        let byte = data[read];

        res |= (byte as u64 & MASK) << bit_shift;
        if byte < OVERFLOW_U8 {
            return Ok((res, read + 1));
        }

        bit_shift += 7;
        read += 1;
    }

    if read == MAX_VARINT_LEN_U64 {
        return Err(ErrorKind::Overflow(read));
    }

    Err(ErrorKind::ShortOfData(read))
}

/// Decodes a i64 from given bytes
pub fn get_i64(data: &[u8]) -> Result<(i64, usize), ErrorKind> {
    get_u64(data).map(|(uint, read)| (uint.zagzig(), read))
}

#[cfg(test)]
mod tests;
