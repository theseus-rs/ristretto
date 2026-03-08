/// A lightweight zero-copy byte reader for big-endian binary data.
#[derive(Debug)]
pub struct ByteReader<'a> {
    data: &'a [u8],
    pos: usize,
}

/// Shared static error for unexpected end of input.
const EOF_ERR: crate::Error = crate::Error::UnexpectedEof;

impl<'a> ByteReader<'a> {
    /// Create a new `ByteReader` from a byte slice.
    #[inline]
    #[must_use]
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    /// Returns the current position in the byte stream.
    #[inline]
    #[must_use]
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Sets the current position in the byte stream.
    #[inline]
    pub fn set_position(&mut self, pos: usize) {
        self.pos = pos;
    }

    /// Returns the underlying byte slice.
    #[inline]
    #[must_use]
    pub fn data(&self) -> &'a [u8] {
        self.data
    }

    /// Returns the number of remaining bytes.
    #[inline]
    #[must_use]
    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }

    /// Returns a slice of the remaining unread data.
    #[inline]
    #[must_use]
    pub fn remaining_slice(&self) -> &'a [u8] {
        &self.data[self.pos..]
    }

    /// Read `N` bytes from the current position, advancing the position.
    /// Returns a reference to an array of exactly `N` bytes.
    #[inline]
    fn read_array<const N: usize>(&mut self) -> crate::Result<&[u8; N]> {
        let end = self.pos + N;
        if end > self.data.len() {
            return Err(EOF_ERR);
        }
        // SAFETY: We just verified that pos..end is within bounds and the slice has exactly N bytes
        #[expect(unsafe_code)]
        let array = unsafe { &*self.data.as_ptr().add(self.pos).cast::<[u8; N]>() };
        self.pos = end;
        Ok(array)
    }

    /// Read a single byte (u8).
    ///
    /// # Errors
    /// Returns `UnexpectedEof` if no bytes remain.
    #[inline]
    pub fn read_u8(&mut self) -> crate::Result<u8> {
        if self.pos >= self.data.len() {
            return Err(EOF_ERR);
        }
        // SAFETY: We just verified pos < data.len()
        #[expect(unsafe_code)]
        let value = unsafe { *self.data.get_unchecked(self.pos) };
        self.pos += 1;
        Ok(value)
    }

    /// Read a single signed byte (i8).
    ///
    /// # Errors
    /// Returns `UnexpectedEof` if no bytes remain.
    #[inline]
    pub fn read_i8(&mut self) -> crate::Result<i8> {
        let arr = self.read_array::<1>()?;
        Ok(i8::from_be_bytes(*arr))
    }

    /// Read a big-endian u16.
    ///
    /// # Errors
    /// Returns `UnexpectedEof` if fewer than 2 bytes remain.
    #[inline]
    pub fn read_u16(&mut self) -> crate::Result<u16> {
        Ok(u16::from_be_bytes(*self.read_array::<2>()?))
    }

    /// Read a big-endian i16.
    ///
    /// # Errors
    /// Returns `UnexpectedEof` if fewer than 2 bytes remain.
    #[inline]
    pub fn read_i16(&mut self) -> crate::Result<i16> {
        Ok(i16::from_be_bytes(*self.read_array::<2>()?))
    }

    /// Read a big-endian u32.
    ///
    /// # Errors
    /// Returns `UnexpectedEof` if fewer than 4 bytes remain.
    #[inline]
    pub fn read_u32(&mut self) -> crate::Result<u32> {
        Ok(u32::from_be_bytes(*self.read_array::<4>()?))
    }

    /// Read a big-endian i32.
    ///
    /// # Errors
    /// Returns `UnexpectedEof` if fewer than 4 bytes remain.
    #[inline]
    pub fn read_i32(&mut self) -> crate::Result<i32> {
        Ok(i32::from_be_bytes(*self.read_array::<4>()?))
    }

    /// Read a big-endian i64.
    ///
    /// # Errors
    /// Returns `UnexpectedEof` if fewer than 8 bytes remain.
    #[inline]
    pub fn read_i64(&mut self) -> crate::Result<i64> {
        Ok(i64::from_be_bytes(*self.read_array::<8>()?))
    }

    /// Read a big-endian f32.
    ///
    /// # Errors
    /// Returns `UnexpectedEof` if fewer than 4 bytes remain.
    #[inline]
    pub fn read_f32(&mut self) -> crate::Result<f32> {
        Ok(f32::from_be_bytes(*self.read_array::<4>()?))
    }

    /// Read a big-endian f64.
    ///
    /// # Errors
    /// Returns `UnexpectedEof` if fewer than 8 bytes remain.
    #[inline]
    pub fn read_f64(&mut self) -> crate::Result<f64> {
        Ok(f64::from_be_bytes(*self.read_array::<8>()?))
    }

    /// Read exactly `len` bytes as a sub-slice without copying.
    ///
    /// # Errors
    /// Returns `UnexpectedEof` if fewer than `len` bytes remain.
    #[inline]
    pub fn read_bytes(&mut self, len: usize) -> crate::Result<&'a [u8]> {
        let end = self.pos + len;
        if end > self.data.len() {
            return Err(EOF_ERR);
        }
        // SAFETY: We just verified end <= data.len()
        #[expect(unsafe_code)]
        let slice = unsafe { self.data.get_unchecked(self.pos..end) };
        self.pos = end;
        Ok(slice)
    }

    /// Read into a mutable buffer (like `std::io::Read::read_exact`).
    ///
    /// # Errors
    /// Returns `UnexpectedEof` if fewer than `buf.len()` bytes remain.
    #[inline]
    pub fn read_exact(&mut self, buf: &mut [u8]) -> crate::Result<()> {
        let slice = self.read_bytes(buf.len())?;
        buf.copy_from_slice(slice);
        Ok(())
    }

    /// Skip exactly `len` bytes without returning a slice.
    ///
    /// # Errors
    /// Returns `UnexpectedEof` if fewer than `len` bytes remain.
    #[inline]
    pub fn skip(&mut self, len: usize) -> crate::Result<()> {
        let end = self.pos + len;
        if end > self.data.len() {
            return Err(EOF_ERR);
        }
        self.pos = end;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[test]
    fn test_new_empty() {
        let reader = ByteReader::new(&[]);
        assert_eq!(reader.position(), 0);
        assert_eq!(reader.remaining(), 0);
        assert_eq!(reader.data(), &[] as &[u8]);
        assert_eq!(reader.remaining_slice(), &[] as &[u8]);
    }

    #[test]
    fn test_new_with_data() {
        let data = [1, 2, 3, 4, 5];
        let reader = ByteReader::new(&data);
        assert_eq!(reader.position(), 0);
        assert_eq!(reader.remaining(), 5);
        assert_eq!(reader.data(), &data);
        assert_eq!(reader.remaining_slice(), &data);
    }

    #[test]
    fn test_set_position() {
        let data = [10, 20, 30];
        let mut reader = ByteReader::new(&data);
        reader.set_position(2);
        assert_eq!(reader.position(), 2);
        assert_eq!(reader.remaining(), 1);
        assert_eq!(reader.remaining_slice(), &[30]);
    }

    #[test]
    fn test_set_position_to_end() {
        let data = [10, 20, 30];
        let mut reader = ByteReader::new(&data);
        reader.set_position(3);
        assert_eq!(reader.position(), 3);
        assert_eq!(reader.remaining(), 0);
        assert_eq!(reader.remaining_slice(), &[] as &[u8]);
    }

    #[test]
    fn test_remaining_decreases_after_reads() {
        let data = [0u8; 8];
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.remaining(), 8);
        let _ = reader.read_u8();
        assert_eq!(reader.remaining(), 7);
        let _ = reader.read_u16();
        assert_eq!(reader.remaining(), 5);
        let _ = reader.read_u32();
        assert_eq!(reader.remaining(), 1);
    }

    #[test]
    fn test_read_u8() -> crate::Result<()> {
        let data = [0x00, 0x7F, 0x80, 0xFF];
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_u8()?, 0x00);
        assert_eq!(reader.read_u8()?, 0x7F);
        assert_eq!(reader.read_u8()?, 0x80);
        assert_eq!(reader.read_u8()?, 0xFF);
        assert_eq!(reader.position(), 4);
        Ok(())
    }

    #[test]
    fn test_read_u8_eof() {
        let mut reader = ByteReader::new(&[]);
        assert_eq!(reader.read_u8(), Err(Error::UnexpectedEof));
    }

    #[test]
    fn test_read_u8_eof_after_exhaustion() {
        let data = [42];
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_u8().unwrap(), 42);
        assert_eq!(reader.read_u8(), Err(Error::UnexpectedEof));
    }

    #[test]
    fn test_read_i8() -> crate::Result<()> {
        let data = [0x00, 0x7F, 0x80, 0xFF];
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_i8()?, 0);
        assert_eq!(reader.read_i8()?, 127);
        assert_eq!(reader.read_i8()?, -128);
        assert_eq!(reader.read_i8()?, -1);
        Ok(())
    }

    #[test]
    fn test_read_i8_eof() {
        let mut reader = ByteReader::new(&[]);
        assert_eq!(reader.read_i8(), Err(Error::UnexpectedEof));
    }

    #[test]
    fn test_read_u16() -> crate::Result<()> {
        // 0x0102 = 258 in big-endian
        let data = [0x01, 0x02];
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_u16()?, 0x0102);
        Ok(())
    }

    #[test]
    fn test_read_u16_min_max() -> crate::Result<()> {
        let data = [0x00, 0x00, 0xFF, 0xFF];
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_u16()?, 0);
        assert_eq!(reader.read_u16()?, u16::MAX);
        Ok(())
    }

    #[test]
    fn test_read_u16_eof_empty() {
        let mut reader = ByteReader::new(&[]);
        assert_eq!(reader.read_u16(), Err(Error::UnexpectedEof));
    }

    #[test]
    fn test_read_u16_eof_one_byte() {
        let mut reader = ByteReader::new(&[0x01]);
        assert_eq!(reader.read_u16(), Err(Error::UnexpectedEof));
    }

    #[test]
    fn test_read_i16() -> crate::Result<()> {
        // 0x7FFF = 32767, 0x8000 = -32768
        let data = [0x7F, 0xFF, 0x80, 0x00, 0xFF, 0xFF, 0x00, 0x00];
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_i16()?, i16::MAX);
        assert_eq!(reader.read_i16()?, i16::MIN);
        assert_eq!(reader.read_i16()?, -1);
        assert_eq!(reader.read_i16()?, 0);
        Ok(())
    }

    #[test]
    fn test_read_i16_eof() {
        let mut reader = ByteReader::new(&[0x01]);
        assert_eq!(reader.read_i16(), Err(Error::UnexpectedEof));
    }

    #[test]
    fn test_read_u32() -> crate::Result<()> {
        let data = [0x00, 0x00, 0x01, 0x00]; // 256
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_u32()?, 256);
        Ok(())
    }

    #[test]
    fn test_read_u32_min_max() -> crate::Result<()> {
        let data = [0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF];
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_u32()?, 0);
        assert_eq!(reader.read_u32()?, u32::MAX);
        Ok(())
    }

    #[test]
    fn test_read_u32_eof() {
        let mut reader = ByteReader::new(&[0x01, 0x02, 0x03]);
        assert_eq!(reader.read_u32(), Err(Error::UnexpectedEof));
    }

    #[test]
    fn test_read_i32() -> crate::Result<()> {
        let data = [
            0x7F, 0xFF, 0xFF, 0xFF, // i32::MAX
            0x80, 0x00, 0x00, 0x00, // i32::MIN
            0xFF, 0xFF, 0xFF, 0xFF, // -1
            0x00, 0x00, 0x00, 0x00, // 0
        ];
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_i32()?, i32::MAX);
        assert_eq!(reader.read_i32()?, i32::MIN);
        assert_eq!(reader.read_i32()?, -1);
        assert_eq!(reader.read_i32()?, 0);
        Ok(())
    }

    #[test]
    fn test_read_i32_eof() {
        let mut reader = ByteReader::new(&[0x01, 0x02]);
        assert_eq!(reader.read_i32(), Err(Error::UnexpectedEof));
    }

    #[test]
    fn test_read_i64() -> crate::Result<()> {
        let data = [
            0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // i64::MAX
            0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // i64::MIN
        ];
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_i64()?, i64::MAX);
        assert_eq!(reader.read_i64()?, i64::MIN);
        Ok(())
    }

    #[test]
    fn test_read_i64_eof() {
        let mut reader = ByteReader::new(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]);
        assert_eq!(reader.read_i64(), Err(Error::UnexpectedEof));
    }

    #[test]
    fn test_read_f32() -> crate::Result<()> {
        let val: f32 = 1.5;
        let bytes = val.to_be_bytes();
        let mut reader = ByteReader::new(&bytes);
        let result = reader.read_f32()?;
        assert!((result - val).abs() < f32::EPSILON);
        Ok(())
    }

    #[test]
    fn test_read_f32_special_values() -> crate::Result<()> {
        let mut data = Vec::new();
        data.extend_from_slice(&0.0_f32.to_be_bytes());
        data.extend_from_slice(&f32::INFINITY.to_be_bytes());
        data.extend_from_slice(&f32::NEG_INFINITY.to_be_bytes());
        data.extend_from_slice(&f32::NAN.to_be_bytes());

        let mut reader = ByteReader::new(&data);
        assert!((reader.read_f32()? - 0.0).abs() < f32::EPSILON);
        let inf = reader.read_f32()?;
        assert!(inf.is_infinite() && inf.is_sign_positive());
        let neg_inf = reader.read_f32()?;
        assert!(neg_inf.is_infinite() && neg_inf.is_sign_negative());
        assert!(reader.read_f32()?.is_nan());
        Ok(())
    }

    #[test]
    fn test_read_f32_eof() {
        let mut reader = ByteReader::new(&[0x01, 0x02, 0x03]);
        assert!(reader.read_f32().is_err());
    }

    #[test]
    fn test_read_f64() -> crate::Result<()> {
        let val: f64 = std::f64::consts::PI;
        let bytes = val.to_be_bytes();
        let mut reader = ByteReader::new(&bytes);
        let result = reader.read_f64()?;
        assert!((result - val).abs() < f64::EPSILON);
        Ok(())
    }

    #[test]
    fn test_read_f64_special_values() -> crate::Result<()> {
        let mut data = Vec::new();
        data.extend_from_slice(&0.0_f64.to_be_bytes());
        data.extend_from_slice(&f64::INFINITY.to_be_bytes());
        data.extend_from_slice(&f64::NEG_INFINITY.to_be_bytes());
        data.extend_from_slice(&f64::NAN.to_be_bytes());

        let mut reader = ByteReader::new(&data);
        assert!((reader.read_f64()? - 0.0).abs() < f64::EPSILON);
        let inf = reader.read_f64()?;
        assert!(inf.is_infinite() && inf.is_sign_positive());
        let neg_inf = reader.read_f64()?;
        assert!(neg_inf.is_infinite() && neg_inf.is_sign_negative());
        assert!(reader.read_f64()?.is_nan());
        Ok(())
    }

    #[test]
    fn test_read_f64_eof() {
        let mut reader = ByteReader::new(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]);
        assert!(reader.read_f64().is_err());
    }

    #[test]
    fn test_read_bytes() -> crate::Result<()> {
        let data = [10, 20, 30, 40, 50];
        let mut reader = ByteReader::new(&data);
        let slice = reader.read_bytes(3)?;
        assert_eq!(slice, &[10, 20, 30]);
        assert_eq!(reader.position(), 3);
        assert_eq!(reader.remaining(), 2);
        Ok(())
    }

    #[test]
    fn test_read_bytes_zero_length() -> crate::Result<()> {
        let data = [1, 2, 3];
        let mut reader = ByteReader::new(&data);
        let slice = reader.read_bytes(0)?;
        assert_eq!(slice, &[] as &[u8]);
        assert_eq!(reader.position(), 0);
        Ok(())
    }

    #[test]
    fn test_read_bytes_entire_buffer() -> crate::Result<()> {
        let data = [1, 2, 3];
        let mut reader = ByteReader::new(&data);
        let slice = reader.read_bytes(3)?;
        assert_eq!(slice, &[1, 2, 3]);
        assert_eq!(reader.remaining(), 0);
        Ok(())
    }

    #[test]
    fn test_read_bytes_eof() {
        let data = [1, 2, 3];
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_bytes(4), Err(Error::UnexpectedEof));
    }

    #[test]
    fn test_read_bytes_eof_empty() {
        let mut reader = ByteReader::new(&[]);
        assert_eq!(reader.read_bytes(1), Err(Error::UnexpectedEof));
    }

    #[test]
    fn test_read_exact() -> crate::Result<()> {
        let data = [0xAA, 0xBB, 0xCC, 0xDD];
        let mut reader = ByteReader::new(&data);
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        assert_eq!(buf, [0xAA, 0xBB, 0xCC, 0xDD]);
        assert_eq!(reader.position(), 4);
        Ok(())
    }

    #[test]
    fn test_read_exact_partial() -> crate::Result<()> {
        let data = [1, 2, 3, 4, 5];
        let mut reader = ByteReader::new(&data);
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        assert_eq!(buf, [1, 2]);
        reader.read_exact(&mut buf)?;
        assert_eq!(buf, [3, 4]);
        assert_eq!(reader.remaining(), 1);
        Ok(())
    }

    #[test]
    fn test_read_exact_zero_length() -> crate::Result<()> {
        let data = [1, 2, 3];
        let mut reader = ByteReader::new(&data);
        let mut buf = [0u8; 0];
        reader.read_exact(&mut buf)?;
        assert_eq!(reader.position(), 0);
        Ok(())
    }

    #[test]
    fn test_read_exact_eof() {
        let data = [1, 2];
        let mut reader = ByteReader::new(&data);
        let mut buf = [0u8; 3];
        assert_eq!(reader.read_exact(&mut buf), Err(Error::UnexpectedEof));
    }

    #[test]
    fn test_skip() -> crate::Result<()> {
        let data = [1, 2, 3, 4, 5];
        let mut reader = ByteReader::new(&data);
        reader.skip(3)?;
        assert_eq!(reader.position(), 3);
        assert_eq!(reader.read_u8()?, 4);
        Ok(())
    }

    #[test]
    fn test_skip_zero() -> crate::Result<()> {
        let data = [1, 2, 3];
        let mut reader = ByteReader::new(&data);
        reader.skip(0)?;
        assert_eq!(reader.position(), 0);
        Ok(())
    }

    #[test]
    fn test_skip_entire_buffer() -> crate::Result<()> {
        let data = [1, 2, 3];
        let mut reader = ByteReader::new(&data);
        reader.skip(3)?;
        assert_eq!(reader.remaining(), 0);
        Ok(())
    }

    #[test]
    fn test_skip_eof() {
        let data = [1, 2, 3];
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.skip(4), Err(Error::UnexpectedEof));
    }

    #[test]
    fn test_skip_eof_empty() {
        let mut reader = ByteReader::new(&[]);
        assert_eq!(reader.skip(1), Err(Error::UnexpectedEof));
    }

    #[test]
    fn test_sequential_mixed_reads() -> crate::Result<()> {
        let mut data = Vec::new();
        data.push(0x42); // u8
        data.extend_from_slice(&0x1234_u16.to_be_bytes()); // u16
        data.extend_from_slice(&0xDEAD_BEEF_u32.to_be_bytes()); // u32
        data.extend_from_slice(&(-1_i64).to_be_bytes()); // i64
        data.extend_from_slice(&std::f64::consts::PI.to_be_bytes()); // f64

        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_u8()?, 0x42);
        assert_eq!(reader.read_u16()?, 0x1234);
        assert_eq!(reader.read_u32()?, 0xDEAD_BEEF);
        assert_eq!(reader.read_i64()?, -1);
        let pi = reader.read_f64()?;
        assert!((pi - std::f64::consts::PI).abs() < f64::EPSILON);
        assert_eq!(reader.remaining(), 0);
        Ok(())
    }

    #[test]
    fn test_read_then_skip_then_read() -> crate::Result<()> {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05];
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_u8()?, 0x01);
        reader.skip(2)?;
        assert_eq!(reader.read_u8()?, 0x04);
        assert_eq!(reader.read_u8()?, 0x05);
        assert_eq!(reader.remaining(), 0);
        Ok(())
    }

    #[test]
    fn test_position_reset_and_reread() -> crate::Result<()> {
        let data = [0xCA, 0xFE, 0xBA, 0xBE];
        let mut reader = ByteReader::new(&data);
        let first = reader.read_u32()?;
        assert_eq!(first, 0xCAFE_BABE);
        reader.set_position(0);
        let second = reader.read_u32()?;
        assert_eq!(second, 0xCAFE_BABE);
        Ok(())
    }

    #[test]
    fn test_read_bytes_zero_copy() -> crate::Result<()> {
        let data = [1, 2, 3, 4, 5];
        let mut reader = ByteReader::new(&data);
        let slice = reader.read_bytes(5)?;
        // Verify it's a reference into the original data (zero-copy)
        assert!(std::ptr::eq(slice.as_ptr(), data.as_ptr()));
        Ok(())
    }

    #[test]
    fn test_remaining_with_set_position_beyond_end() {
        let data = [1, 2, 3];
        let mut reader = ByteReader::new(&data);
        reader.set_position(10);
        // saturating_sub should prevent underflow
        assert_eq!(reader.remaining(), 0);
    }

    #[test]
    fn test_debug_format() {
        let data = [1, 2, 3];
        let reader = ByteReader::new(&data);
        let debug = format!("{reader:?}");
        assert!(debug.contains("ByteReader"));
    }

    #[test]
    fn test_single_byte_boundary() -> crate::Result<()> {
        let data = [0xFF];
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.remaining(), 1);
        assert_eq!(reader.read_u8()?, 0xFF);
        assert_eq!(reader.remaining(), 0);
        assert_eq!(reader.read_u8(), Err(Error::UnexpectedEof));
        Ok(())
    }

    #[test]
    fn test_exact_size_reads() -> crate::Result<()> {
        // Exactly 2 bytes for u16
        let data = [0x00, 0x01];
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_u16()?, 1);
        assert_eq!(reader.remaining(), 0);

        // Exactly 4 bytes for u32
        let data = 42_u32.to_be_bytes();
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_u32()?, 42);
        assert_eq!(reader.remaining(), 0);

        // Exactly 8 bytes for i64
        let data = 123_456_789_i64.to_be_bytes();
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_i64()?, 123_456_789);
        assert_eq!(reader.remaining(), 0);

        Ok(())
    }

    #[test]
    fn test_eof_does_not_advance_position() {
        let data = [0x01];
        let mut reader = ByteReader::new(&data);
        let _ = reader.read_u8(); // succeeds, pos=1
        let pos_before = reader.position();
        let result = reader.read_u16();
        assert!(result.is_err());
        // Position should not change on error
        assert_eq!(reader.position(), pos_before);
    }

    #[test]
    fn test_skip_eof_does_not_advance_position() {
        let data = [0x01, 0x02];
        let mut reader = ByteReader::new(&data);
        let _ = reader.read_u8();
        let pos_before = reader.position();
        let result = reader.skip(5);
        assert!(result.is_err());
        assert_eq!(reader.position(), pos_before);
    }

    #[test]
    fn test_read_bytes_eof_does_not_advance_position() {
        let data = [0x01, 0x02];
        let mut reader = ByteReader::new(&data);
        let _ = reader.read_u8();
        let pos_before = reader.position();
        let result = reader.read_bytes(5);
        assert!(result.is_err());
        assert_eq!(reader.position(), pos_before);
    }

    #[test]
    fn test_read_exact_eof_does_not_advance_position() {
        let data = [0x01, 0x02];
        let mut reader = ByteReader::new(&data);
        let _ = reader.read_u8();
        let pos_before = reader.position();
        let mut buf = [0u8; 5];
        let result = reader.read_exact(&mut buf);
        assert!(result.is_err());
        assert_eq!(reader.position(), pos_before);
    }

    #[test]
    fn test_read_i8_eof_after_exhaustion() {
        let data = [0x42];
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_i8().unwrap(), 0x42);
        assert_eq!(reader.read_i8(), Err(Error::UnexpectedEof));
    }

    #[test]
    fn test_read_i8_sequential_all_bytes() -> crate::Result<()> {
        let data: Vec<u8> = (0..=255).collect();
        let mut reader = ByteReader::new(&data);
        for i in 0..=255u8 {
            let val = reader.read_i8()?;
            assert_eq!(val, i8::from_be_bytes([i]));
        }
        assert_eq!(reader.remaining(), 0);
        Ok(())
    }

    #[test]
    fn test_read_u8_sequential_all_bytes() -> crate::Result<()> {
        let data: Vec<u8> = (0..=255).collect();
        let mut reader = ByteReader::new(&data);
        for i in 0..=255u8 {
            assert_eq!(reader.read_u8()?, i);
        }
        assert_eq!(reader.remaining(), 0);
        Ok(())
    }

    #[test]
    fn test_read_i16_eof_empty() {
        let mut reader = ByteReader::new(&[]);
        assert_eq!(reader.read_i16(), Err(Error::UnexpectedEof));
    }

    #[test]
    fn test_read_i16_known_values() -> crate::Result<()> {
        // 0x0001 = 1, 0xFFFF = -1
        let data = [0x00, 0x01, 0xFF, 0xFE];
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_i16()?, 1);
        assert_eq!(reader.read_i16()?, -2);
        Ok(())
    }

    #[test]
    fn test_read_u32_eof_empty() {
        let mut reader = ByteReader::new(&[]);
        assert_eq!(reader.read_u32(), Err(Error::UnexpectedEof));
    }

    #[test]
    fn test_read_u32_eof_partial_two_bytes() {
        let mut reader = ByteReader::new(&[0x01, 0x02]);
        assert_eq!(reader.read_u32(), Err(Error::UnexpectedEof));
    }

    #[test]
    fn test_read_u32_eof_partial_one_byte() {
        let mut reader = ByteReader::new(&[0x01]);
        assert_eq!(reader.read_u32(), Err(Error::UnexpectedEof));
    }

    #[test]
    fn test_read_i32_eof_empty() {
        let mut reader = ByteReader::new(&[]);
        assert_eq!(reader.read_i32(), Err(Error::UnexpectedEof));
    }

    #[test]
    fn test_read_i32_known_values() -> crate::Result<()> {
        let data = [0x00, 0x00, 0x00, 0x01, 0xFF, 0xFF, 0xFF, 0xFE];
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_i32()?, 1);
        assert_eq!(reader.read_i32()?, -2);
        Ok(())
    }

    #[test]
    fn test_read_i64_eof_empty() {
        let mut reader = ByteReader::new(&[]);
        assert_eq!(reader.read_i64(), Err(Error::UnexpectedEof));
    }

    #[test]
    fn test_read_i64_zero() -> crate::Result<()> {
        let data = 0_i64.to_be_bytes();
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_i64()?, 0);
        Ok(())
    }

    #[test]
    fn test_read_i64_negative_one() -> crate::Result<()> {
        let data = (-1_i64).to_be_bytes();
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_i64()?, -1);
        Ok(())
    }

    #[test]
    fn test_read_i64_eof_partial_4_bytes() {
        let mut reader = ByteReader::new(&[0x01, 0x02, 0x03, 0x04]);
        assert_eq!(reader.read_i64(), Err(Error::UnexpectedEof));
    }

    #[test]
    fn test_read_f32_eof_empty() {
        let mut reader = ByteReader::new(&[]);
        assert!(reader.read_f32().is_err());
    }

    #[test]
    fn test_read_f32_negative_zero() -> crate::Result<()> {
        let data = (-0.0_f32).to_be_bytes();
        let mut reader = ByteReader::new(&data);
        let val = reader.read_f32()?;
        assert!(val.is_sign_negative());
        assert!((val - 0.0).abs() < f32::EPSILON);
        Ok(())
    }

    #[test]
    fn test_read_f32_min_max() -> crate::Result<()> {
        let mut data = Vec::new();
        data.extend_from_slice(&f32::MIN.to_be_bytes());
        data.extend_from_slice(&f32::MAX.to_be_bytes());
        data.extend_from_slice(&f32::MIN_POSITIVE.to_be_bytes());

        let mut reader = ByteReader::new(&data);
        assert!((reader.read_f32()? - f32::MIN).abs() < f32::EPSILON);
        assert!((reader.read_f32()? - f32::MAX).abs() < f32::EPSILON);
        assert!((reader.read_f32()? - f32::MIN_POSITIVE).abs() < f32::EPSILON);
        Ok(())
    }

    #[test]
    fn test_read_f32_eof_one_byte() {
        let mut reader = ByteReader::new(&[0x01]);
        assert!(reader.read_f32().is_err());
    }

    #[test]
    fn test_read_f32_eof_two_bytes() {
        let mut reader = ByteReader::new(&[0x01, 0x02]);
        assert!(reader.read_f32().is_err());
    }

    #[test]
    fn test_read_f64_eof_empty() {
        let mut reader = ByteReader::new(&[]);
        assert!(reader.read_f64().is_err());
    }

    #[test]
    fn test_read_f64_negative_zero() -> crate::Result<()> {
        let data = (-0.0_f64).to_be_bytes();
        let mut reader = ByteReader::new(&data);
        let val = reader.read_f64()?;
        assert!(val.is_sign_negative());
        assert!((val - 0.0).abs() < f64::EPSILON);
        Ok(())
    }

    #[test]
    fn test_read_f64_min_max() -> crate::Result<()> {
        let mut data = Vec::new();
        data.extend_from_slice(&f64::MIN.to_be_bytes());
        data.extend_from_slice(&f64::MAX.to_be_bytes());
        data.extend_from_slice(&f64::MIN_POSITIVE.to_be_bytes());

        let mut reader = ByteReader::new(&data);
        assert!((reader.read_f64()? - f64::MIN).abs() < f64::EPSILON);
        assert!((reader.read_f64()? - f64::MAX).abs() < f64::EPSILON);
        assert!((reader.read_f64()? - f64::MIN_POSITIVE).abs() < f64::EPSILON);
        Ok(())
    }

    #[test]
    fn test_read_f64_eof_four_bytes() {
        let mut reader = ByteReader::new(&[0x01, 0x02, 0x03, 0x04]);
        assert!(reader.read_f64().is_err());
    }

    #[test]
    fn test_read_bytes_zero_length_empty_reader() -> crate::Result<()> {
        let mut reader = ByteReader::new(&[]);
        let slice = reader.read_bytes(0)?;
        assert_eq!(slice, &[] as &[u8]);
        assert_eq!(reader.position(), 0);
        Ok(())
    }

    #[test]
    fn test_read_bytes_multiple_consecutive() -> crate::Result<()> {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_bytes(3)?, &[1, 2, 3]);
        assert_eq!(reader.read_bytes(4)?, &[4, 5, 6, 7]);
        assert_eq!(reader.read_bytes(3)?, &[8, 9, 10]);
        assert_eq!(reader.remaining(), 0);
        Ok(())
    }

    #[test]
    fn test_read_bytes_after_partial_read_eof() {
        let data = [1, 2, 3, 4, 5];
        let mut reader = ByteReader::new(&data);
        let _ = reader.read_bytes(3);
        assert_eq!(reader.read_bytes(3), Err(Error::UnexpectedEof));
    }

    #[test]
    fn test_read_exact_zero_length_empty_reader() -> crate::Result<()> {
        let mut reader = ByteReader::new(&[]);
        let mut buf = [0u8; 0];
        reader.read_exact(&mut buf)?;
        assert_eq!(reader.position(), 0);
        Ok(())
    }

    #[test]
    fn test_read_exact_fills_buffer_correctly() -> crate::Result<()> {
        let data = [0x11, 0x22, 0x33, 0x44, 0x55];
        let mut reader = ByteReader::new(&data);
        let mut buf = [0u8; 5];
        reader.read_exact(&mut buf)?;
        assert_eq!(buf, data);
        Ok(())
    }

    #[test]
    fn test_read_exact_eof_empty_reader() {
        let mut reader = ByteReader::new(&[]);
        let mut buf = [0u8; 1];
        assert_eq!(reader.read_exact(&mut buf), Err(Error::UnexpectedEof));
    }

    #[test]
    fn test_skip_zero_empty_reader() -> crate::Result<()> {
        let mut reader = ByteReader::new(&[]);
        reader.skip(0)?;
        assert_eq!(reader.position(), 0);
        assert_eq!(reader.remaining(), 0);
        Ok(())
    }

    #[test]
    fn test_skip_then_read_remaining() -> crate::Result<()> {
        let data = [0xAA, 0xBB, 0xCC, 0xDD];
        let mut reader = ByteReader::new(&data);
        reader.skip(2)?;
        let remaining = reader.read_bytes(2)?;
        assert_eq!(remaining, &[0xCC, 0xDD]);
        Ok(())
    }

    #[test]
    fn test_skip_multiple() -> crate::Result<()> {
        let data = [1, 2, 3, 4, 5, 6, 7, 8];
        let mut reader = ByteReader::new(&data);
        reader.skip(2)?;
        assert_eq!(reader.position(), 2);
        reader.skip(3)?;
        assert_eq!(reader.position(), 5);
        reader.skip(3)?;
        assert_eq!(reader.position(), 8);
        assert_eq!(reader.remaining(), 0);
        Ok(())
    }

    #[test]
    fn test_set_position_back_and_forth() -> crate::Result<()> {
        let data = [0x11, 0x22, 0x33, 0x44];
        let mut reader = ByteReader::new(&data);
        reader.set_position(2);
        assert_eq!(reader.read_u8()?, 0x33);
        reader.set_position(0);
        assert_eq!(reader.read_u8()?, 0x11);
        reader.set_position(3);
        assert_eq!(reader.read_u8()?, 0x44);
        Ok(())
    }

    #[test]
    fn test_set_position_to_middle() -> crate::Result<()> {
        let data = [0x00, 0x00, 0xCA, 0xFE];
        let mut reader = ByteReader::new(&data);
        reader.set_position(2);
        assert_eq!(reader.read_u16()?, 0xCAFE);
        Ok(())
    }

    // --- data() accessor ---

    #[test]
    fn test_data_returns_full_slice_regardless_of_position() {
        let data = [1, 2, 3, 4, 5];
        let mut reader = ByteReader::new(&data);
        let _ = reader.read_u8();
        let _ = reader.read_u8();
        // data() always returns the full slice
        assert_eq!(reader.data(), &data);
        assert_eq!(reader.data().len(), 5);
    }

    #[test]
    fn test_remaining_slice_after_partial_read() -> crate::Result<()> {
        let data = [10, 20, 30, 40, 50];
        let mut reader = ByteReader::new(&data);
        reader.read_u8()?;
        reader.read_u8()?;
        assert_eq!(reader.remaining_slice(), &[30, 40, 50]);
        Ok(())
    }

    #[test]
    fn test_remaining_slice_at_end() -> crate::Result<()> {
        let data = [1, 2];
        let mut reader = ByteReader::new(&data);
        reader.read_u16()?;
        assert_eq!(reader.remaining_slice(), &[] as &[u8]);
        Ok(())
    }

    #[test]
    fn test_u16_big_endian_byte_order() -> crate::Result<()> {
        // Most-significant byte first
        let data = [0xAB, 0xCD];
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_u16()?, 0xABCD);
        Ok(())
    }

    #[test]
    fn test_u32_big_endian_byte_order() -> crate::Result<()> {
        let data = [0x12, 0x34, 0x56, 0x78];
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_u32()?, 0x1234_5678);
        Ok(())
    }

    #[test]
    fn test_i32_big_endian_byte_order() -> crate::Result<()> {
        let data = [0xFE, 0xDC, 0xBA, 0x98];
        let mut reader = ByteReader::new(&data);
        assert_eq!(
            reader.read_i32()?,
            i32::from_be_bytes([0xFE, 0xDC, 0xBA, 0x98])
        );
        Ok(())
    }

    #[test]
    fn test_i64_big_endian_byte_order() -> crate::Result<()> {
        let data = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_i64()?, 0x0123_4567_89AB_CDEF_i64);
        Ok(())
    }

    #[test]
    fn test_multiple_eof_attempts_u8() {
        let mut reader = ByteReader::new(&[]);
        for _ in 0..5 {
            assert_eq!(reader.read_u8(), Err(Error::UnexpectedEof));
            assert_eq!(reader.position(), 0);
        }
    }

    #[test]
    fn test_multiple_eof_attempts_different_types() {
        let mut reader = ByteReader::new(&[]);
        assert_eq!(reader.read_u8(), Err(Error::UnexpectedEof));
        assert_eq!(reader.read_u16(), Err(Error::UnexpectedEof));
        assert_eq!(reader.read_u32(), Err(Error::UnexpectedEof));
        assert_eq!(reader.read_i32(), Err(Error::UnexpectedEof));
        assert_eq!(reader.read_i64(), Err(Error::UnexpectedEof));
        assert!(reader.read_f32().is_err());
        assert!(reader.read_f64().is_err());
        assert_eq!(reader.read_i8(), Err(Error::UnexpectedEof));
        assert_eq!(reader.read_i16(), Err(Error::UnexpectedEof));
        // Position must remain 0 throughout
        assert_eq!(reader.position(), 0);
    }

    #[test]
    fn test_interleaved_signed_unsigned_reads() -> crate::Result<()> {
        let mut data = Vec::new();
        data.extend_from_slice(&0x7FFF_u16.to_be_bytes()); // u16: 32767
        data.extend_from_slice(&(-1_i16).to_be_bytes()); // i16: -1
        data.extend_from_slice(&0xDEAD_BEEF_u32.to_be_bytes()); // u32
        data.extend_from_slice(&(-42_i32).to_be_bytes()); // i32: -42

        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_u16()?, 0x7FFF);
        assert_eq!(reader.read_i16()?, -1);
        assert_eq!(reader.read_u32()?, 0xDEAD_BEEF);
        assert_eq!(reader.read_i32()?, -42);
        assert_eq!(reader.remaining(), 0);
        Ok(())
    }

    #[test]
    fn test_all_types_round_trip() -> crate::Result<()> {
        let mut data = Vec::new();
        data.push(0xAB); // u8
        data.push(0x80); // i8 as byte
        data.extend_from_slice(&0x1234_u16.to_be_bytes());
        data.extend_from_slice(&(-4321_i16).to_be_bytes());
        data.extend_from_slice(&0xCAFE_BABE_u32.to_be_bytes());
        data.extend_from_slice(&(-123_456_i32).to_be_bytes());
        data.extend_from_slice(&0x0102_0304_0506_0708_i64.to_be_bytes());
        data.extend_from_slice(&1.5_f32.to_be_bytes());
        data.extend_from_slice(&std::f64::consts::E.to_be_bytes());

        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.read_u8()?, 0xAB);
        assert_eq!(reader.read_i8()?, -128);
        assert_eq!(reader.read_u16()?, 0x1234);
        assert_eq!(reader.read_i16()?, -4321);
        assert_eq!(reader.read_u32()?, 0xCAFE_BABE);
        assert_eq!(reader.read_i32()?, -123_456);
        assert_eq!(reader.read_i64()?, 0x0102_0304_0506_0708_i64);
        let f = reader.read_f32()?;
        assert!((f - 1.5_f32).abs() < f32::EPSILON);
        let e = reader.read_f64()?;
        assert!((e - std::f64::consts::E).abs() < f64::EPSILON);
        assert_eq!(reader.remaining(), 0);
        Ok(())
    }

    #[test]
    fn test_read_bytes_large_slice() -> crate::Result<()> {
        let data: Vec<u8> = (0..1024)
            .map(|i: u32| u8::try_from(i % 256).unwrap())
            .collect();
        let mut reader = ByteReader::new(&data);
        let slice = reader.read_bytes(1024)?;
        assert_eq!(slice.len(), 1024);
        assert_eq!(slice[0], 0);
        assert_eq!(slice[255], 255);
        assert_eq!(slice[256], 0);
        assert_eq!(reader.remaining(), 0);
        Ok(())
    }

    #[test]
    fn test_skip_large() -> crate::Result<()> {
        let data = vec![0u8; 10_000];
        let mut reader = ByteReader::new(&data);
        reader.skip(9_999)?;
        assert_eq!(reader.remaining(), 1);
        assert_eq!(reader.read_u8()?, 0);
        Ok(())
    }

    #[test]
    fn test_read_exact_large() -> crate::Result<()> {
        let data: Vec<u8> = (0..512)
            .map(|i: u32| u8::try_from(i % 256).unwrap())
            .collect();
        let mut reader = ByteReader::new(&data);
        let mut buf = vec![0u8; 512];
        reader.read_exact(&mut buf)?;
        assert_eq!(buf, data);
        assert_eq!(reader.remaining(), 0);
        Ok(())
    }

    #[test]
    fn test_read_u8_eof_does_not_advance_position() {
        let data = [0x01, 0x02];
        let mut reader = ByteReader::new(&data);
        let _ = reader.read_u8();
        let _ = reader.read_u8();
        let pos_before = reader.position();
        let result = reader.read_u8();
        assert!(result.is_err());
        assert_eq!(reader.position(), pos_before);
    }

    #[test]
    fn test_read_i8_eof_does_not_advance_position() {
        let data = [0x01];
        let mut reader = ByteReader::new(&data);
        let _ = reader.read_i8();
        let pos_before = reader.position();
        let result = reader.read_i8();
        assert!(result.is_err());
        assert_eq!(reader.position(), pos_before);
    }

    #[test]
    fn test_read_i16_eof_does_not_advance_position() {
        let data = [0x01, 0x02, 0x03];
        let mut reader = ByteReader::new(&data);
        let _ = reader.read_i16(); // consumes 2
        let pos_before = reader.position();
        let result = reader.read_i16(); // only 1 byte left
        assert!(result.is_err());
        assert_eq!(reader.position(), pos_before);
    }

    #[test]
    fn test_read_u16_eof_does_not_advance_position() {
        let data = [0x01, 0x02, 0x03];
        let mut reader = ByteReader::new(&data);
        let _ = reader.read_u16();
        let pos_before = reader.position();
        let result = reader.read_u16();
        assert!(result.is_err());
        assert_eq!(reader.position(), pos_before);
    }

    #[test]
    fn test_read_u32_eof_does_not_advance_position() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05];
        let mut reader = ByteReader::new(&data);
        let _ = reader.read_u32(); // consumes 4
        let pos_before = reader.position();
        let result = reader.read_u32(); // only 1 byte left
        assert!(result.is_err());
        assert_eq!(reader.position(), pos_before);
    }

    #[test]
    fn test_read_i32_eof_does_not_advance_position() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let mut reader = ByteReader::new(&data);
        let _ = reader.read_i32();
        let pos_before = reader.position();
        let result = reader.read_i32(); // only 2 bytes left
        assert!(result.is_err());
        assert_eq!(reader.position(), pos_before);
    }

    #[test]
    fn test_read_i64_eof_does_not_advance_position() {
        let data = [0u8; 10];
        let mut reader = ByteReader::new(&data);
        let _ = reader.read_i64(); // consumes 8
        let pos_before = reader.position();
        let result = reader.read_i64(); // only 2 bytes left
        assert!(result.is_err());
        assert_eq!(reader.position(), pos_before);
    }

    #[test]
    fn test_read_f32_eof_does_not_advance_position() {
        let data = [0u8; 5];
        let mut reader = ByteReader::new(&data);
        let _ = reader.read_f32(); // consumes 4
        let pos_before = reader.position();
        let result = reader.read_f32(); // only 1 byte left
        assert!(result.is_err());
        assert_eq!(reader.position(), pos_before);
    }

    #[test]
    fn test_read_f64_eof_does_not_advance_position() {
        let data = [0u8; 12];
        let mut reader = ByteReader::new(&data);
        let _ = reader.read_f64(); // consumes 8
        let pos_before = reader.position();
        let result = reader.read_f64(); // only 4 bytes left
        assert!(result.is_err());
        assert_eq!(reader.position(), pos_before);
    }

    #[test]
    fn test_read_bytes_zero_copy_at_offset() -> crate::Result<()> {
        let data = [10, 20, 30, 40, 50];
        let mut reader = ByteReader::new(&data);
        reader.skip(2)?;
        let slice = reader.read_bytes(3)?;
        // The returned slice should point into the original data at offset 2
        assert!(std::ptr::eq(slice.as_ptr(), data[2..].as_ptr()));
        assert_eq!(slice, &[30, 40, 50]);
        Ok(())
    }

    #[test]
    fn test_java_classfile_magic_number() -> crate::Result<()> {
        // Java classfile starts with 0xCAFEBABE
        let data = [0xCA, 0xFE, 0xBA, 0xBE, 0x00, 0x00, 0x00, 0x34];
        let mut reader = ByteReader::new(&data);
        let magic = reader.read_u32()?;
        assert_eq!(magic, 0xCAFE_BABE);
        let minor = reader.read_u16()?;
        assert_eq!(minor, 0);
        let major = reader.read_u16()?;
        assert_eq!(major, 52); // Java 8
        assert_eq!(reader.remaining(), 0);
        Ok(())
    }

    #[test]
    fn test_position_tracking_through_various_reads() -> crate::Result<()> {
        let mut data = Vec::new();
        data.push(0x01); // 1 byte
        data.extend_from_slice(&0x0203_u16.to_be_bytes()); // 2 bytes
        data.extend_from_slice(&0x0405_0607_u32.to_be_bytes()); // 4 bytes
        data.push(0x08); // 1 byte

        let mut reader = ByteReader::new(&data);
        assert_eq!(reader.position(), 0);
        reader.read_u8()?;
        assert_eq!(reader.position(), 1);
        reader.read_u16()?;
        assert_eq!(reader.position(), 3);
        reader.read_u32()?;
        assert_eq!(reader.position(), 7);
        reader.read_u8()?;
        assert_eq!(reader.position(), 8);
        assert_eq!(reader.remaining(), 0);
        Ok(())
    }

    #[test]
    fn test_read_after_set_position_to_zero() -> crate::Result<()> {
        let data = [0xAA, 0xBB, 0xCC];
        let mut reader = ByteReader::new(&data);
        reader.read_u8()?;
        reader.read_u8()?;
        reader.set_position(0);
        assert_eq!(reader.read_u8()?, 0xAA);
        Ok(())
    }

    #[test]
    fn test_eof_error_display() {
        let err = Error::UnexpectedEof;
        assert_eq!(format!("{err}"), "Unexpected end of input");
    }
}
