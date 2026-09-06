//! `OpenJDK` `ImageIO` JPEG native callbacks and per-VM codec state.
//!
//! The callback protocol follows [OpenJDK's ImageIO JPEG bridge](https://github.com/openjdk/jdk/blob/jdk-25-ga/src/java.desktop/share/native/libjavajpeg/imageioJPEG.c).
//! JPEG coding and callbacks are implemented in Rust. Java owns metadata and
//! raster conversion.

mod codec;
pub mod jpegimagereader;
pub mod jpegimagewriter;
mod support;
