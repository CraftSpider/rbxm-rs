//! Implementations of `Read` and `Write` for a `no_std` context

use crate::serde::{Error, Result};

/// A `no_std` minimal implementation of [`std::io::Read`]
pub trait Read {
    /// Read an exact buffer size
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()>;
}

#[cfg(feature = "std")]
impl<T: std::io::Read> Read for T {
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        <Self as std::io::Read>::read_exact(self, buf).map_err(Error::IoError)
    }
}

#[cfg(not(feature = "std"))]
impl Read for &[u8] {
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        if buf.len() < self.len() {
            for i in 0..buf.len() {
                buf[i] = self[i];
            }
            *self = &self[buf.len()..];
            Ok(())
        } else {
            Err(Error::IoError("Input too small to fill buffer"))
        }
    }
}

/// A `no_std` minimal implementation of [`std::io::Write`]
pub trait Write {
    /// Write an exact buffer size
    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
}

#[cfg(feature = "std")]
impl<T: std::io::Write> Write for T {
    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        <Self as std::io::Write>::write_all(self, buf).map_err(super::Error::IoError)
    }
}

#[cfg(not(feature = "std"))]
impl Write for Vec<u8> {
    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.extend_from_slice(buf);
        Ok(())
    }
}

#[cfg(not(feature = "std"))]
impl<T: Write> Write for &mut T {
    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        <T as Write>::write_all(*self, buf)
    }
}
