//! Implementations for the binary encoding of various types in RBXM files.
//! Exposed to the user for low-level modification of RBXM data

mod chomp;
mod print;

pub use chomp::{Chomp, ChompInterleaved, ChompInterleavedTransform, ChompTransform};
pub use print::{Print, PrintInterleaved, PrintInterleavedTransform, PrintTransform};

// Decode/Encode the special formats used to store some values

/// Convert a u32 into an i32, using the special Roblox encoding
#[must_use]
pub const fn decode_i32(mut raw: u32) -> i32 {
    let sign = raw & 1;
    raw >>= 1;
    let mut out = i32::from_ne_bytes(raw.to_ne_bytes());
    if sign == 1 {
        out += 1;
        out = -out;
    }
    out
}

/// Convert an i32 into a u32, using the special Roblox encoding
#[must_use]
pub const fn encode_i32(mut val: i32) -> u32 {
    let sign = (val < 0) as u32;
    if val < 0 {
        val = -val;
        val -= 1;
    }
    let mut out = u32::from_ne_bytes(val.to_ne_bytes());
    out <<= 1;
    out |= sign;
    out
}

/// Convert a u32 to an f32, using the special Roblox encoding
#[must_use]
pub fn decode_f32(mut raw: u32) -> f32 {
    let sign = raw & 1;
    raw >>= 1;
    raw ^= sign * (1 << 31);
    f32::from_ne_bytes(raw.to_ne_bytes())
}

/// Convert an f32 into a u32, using the special Roblox encoding
#[must_use]
pub fn encode_f32(val: f32) -> u32 {
    let mut raw = u32::from_ne_bytes(val.to_ne_bytes());

    let sign = (raw & (1 << 31) > 0) as u32;
    raw <<= 1;
    raw |= sign;
    raw
}

/// Decode a slice of integers as 'cumulative'. Each value should be the sum of all previous
/// values, effectively.
pub fn decode_cumulative(mut slice: &mut [i32]) {
    for _ in 1..slice.len() {
        let (first, second) = slice
            .split_first_mut()
            .expect("Can't happen: we range from 1 to len");
        second[0] += *first;
        slice = second;
    }
}

/// Encode a slice of integers as 'cumulative'. Each value is replaced with its difference from the
/// previous value.
pub fn encode_cumulative(mut slice: &mut [i32]) {
    for _ in (1..slice.len()).rev() {
        let (last, previous) = slice
            .split_last_mut()
            .expect("Can't happen: we range from len to 1");
        *last -= previous.last().expect("Can't happen");
        slice = previous;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_i32() {
        assert_eq!(decode_i32(0b0000_0000_0000_0000_0000_0000_0000_0000), 0);
        assert_eq!(decode_i32(0b0000_0000_0000_0000_0000_0000_0000_0001), -1);
        assert_eq!(decode_i32(0b0000_0000_0000_0000_0000_0000_0000_0100), 2);
        assert_eq!(decode_i32(0b0000_0000_0000_0000_1000_0000_0000_0000), 16384);
        assert_eq!(
            decode_i32(0b0000_0000_0000_0000_1000_0000_0000_0001),
            -16385
        );
    }

    #[test]
    fn test_encode_i32() {
        assert_eq!(encode_i32(0), 0b0000_0000_0000_0000_0000_0000_0000_0000);
        assert_eq!(encode_i32(-1), 0b0000_0000_0000_0000_0000_0000_0000_0001);
        assert_eq!(encode_i32(2), 0b0000_0000_0000_0000_0000_0000_0000_0100);
        assert_eq!(encode_i32(16384), 0b0000_0000_0000_0000_1000_0000_0000_0000);
        assert_eq!(
            encode_i32(-16385),
            0b0000_0000_0000_0000_1000_0000_0000_0001
        );
    }

    #[test]
    fn test_decode_f32() {
        assert_eq!(decode_f32(0b0000_0000_0000_0000_0000_0000_0000_0000), 0f32);
        assert_eq!(decode_f32(0b0111_1111_0000_0000_0000_0000_0000_0001), -1f32);
        assert_eq!(decode_f32(0b1000_0000_0000_0000_0000_0000_0000_0000), 2f32);
    }

    #[test]
    fn test_encode_f32() {
        assert_eq!(encode_f32(0f32), 0b0000_0000_0000_0000_0000_0000_0000_0000);
        assert_eq!(encode_f32(-1f32), 0b0111_1111_0000_0000_0000_0000_0000_0001);
        assert_eq!(encode_f32(2f32), 0b1000_0000_0000_0000_0000_0000_0000_0000);
    }

    #[test]
    fn test_decode_cumulative() {
        let mut array = [0, 1, 1, 5];
        decode_cumulative(&mut array);
        assert_eq!(array, [0, 1, 2, 7]);
    }

    #[test]
    fn test_encode_cumulative() {
        let mut array = [0, 1, 2, 7];
        encode_cumulative(&mut array);
        assert_eq!(array, [0, 1, 1, 5]);
    }
}
