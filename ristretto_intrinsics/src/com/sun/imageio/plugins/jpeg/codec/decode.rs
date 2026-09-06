//! Scan-bounded decoding and Java's JPEG sample/color conventions.
#![expect(
    clippy::indexing_slicing,
    reason = "dimensions, component counts and plane extents are validated before pixel loops"
)]

use super::{Bank, CodecResult, Header, message, segment};
use libjpeg_turbo_rs::{Decoder, RawImage};

#[derive(Debug)]
pub(super) struct Read {
    bytes: Vec<u8>,
    ends: Vec<usize>,
    sampling: Vec<(usize, usize)>,
    width: usize,
    height: usize,
    input_color: i32,
    output_color: i32,
    pub components: usize,
    pass: usize,
    row: usize,
    pixels: Vec<u8>,
}

impl Read {
    pub fn new(bytes: &[u8], header: &Header, bank: &Bank, color: i32) -> CodecResult<Self> {
        let components = conversion(header.color, color, header.components.len())?;
        header
            .width
            .checked_mul(header.height)
            .and_then(|size| size.checked_mul(components))
            .ok_or("JPEG image size overflow")?;
        let mut data = vec![255, 216];
        bank.emit(&mut data, true, true);
        data.extend_from_slice(bytes.get(2..).ok_or("Missing JPEG SOI")?);
        let mut pos = 2;
        let mut ends = Vec::new();
        let mut in_scan = false;
        while let Some(part) = segment(&data, &mut pos)? {
            if in_scan {
                ends.push(part.start);
                in_scan = false;
            }
            if part.code == 0xda {
                in_scan = true;
            }
            if part.code == 0xd9 {
                break;
            }
        }
        if in_scan {
            ends.push(data.len());
        }
        if ends.is_empty() {
            return Err("JPEG image contains no scans".into());
        }
        let sampling = header.components.iter().map(|c| (c.1, c.2)).collect();
        Ok(Self {
            bytes: data,
            ends,
            sampling,
            width: header.width,
            height: header.height,
            input_color: header.color,
            output_color: color,
            components,
            pass: 0,
            row: 0,
            pixels: Vec::new(),
        })
    }

    pub fn start_pass(&mut self, pass: i32) -> CodecResult<i32> {
        self.pass = usize::try_from(pass.max(0))
            .map_err(message)?
            .min(self.ends.len() - 1);
        self.row = 0;
        let mut data = self.bytes[..self.ends[self.pass]].to_vec();
        // A prefix ending at a scan boundary is a valid partial reconstruction.
        // No later scan can change pixels delivered for this output pass.
        data.extend_from_slice(&[255, 217]);
        let raw = Decoder::new(&data)
            .map_err(message)?
            .with_lenient(true)
            .decode_raw()
            .map_err(message)?;
        self.pixels = self.reconstruct(&raw)?;
        i32::try_from(self.pass).map_err(message)
    }

    pub fn row(&mut self, output: &mut [u8]) -> CodecResult<()> {
        let stride = self
            .width
            .checked_mul(self.components)
            .ok_or("JPEG row overflow")?;
        if output.len() < stride {
            return Err("JPEG row buffer too short".into());
        }
        if self.pixels.is_empty() {
            self.start_pass(0)?;
        }
        if self.row >= self.height {
            return Err("No JPEG scanlines remaining".into());
        }
        output[..stride].copy_from_slice(&self.pixels[self.row * stride..(self.row + 1) * stride]);
        self.row += 1;
        Ok(())
    }

    pub fn finish_pass(&self) -> CodecResult<Option<i32>> {
        if self.row != self.height {
            return Err("JPEG output pass is incomplete".into());
        }
        if self.pass + 1 < self.ends.len() {
            Ok(Some(i32::try_from(self.pass + 1).map_err(message)?))
        } else {
            Ok(None)
        }
    }

    fn reconstruct(&self, raw: &RawImage) -> CodecResult<Vec<u8>> {
        if raw.planes.len() != self.sampling.len()
            || raw.plane_widths.len() != self.sampling.len()
            || raw.plane_heights.len() != self.sampling.len()
        {
            return Err("Invalid decoded JPEG planes".into());
        }
        let max_h = self.sampling.iter().map(|c| c.0).max().unwrap_or(1);
        let max_v = self.sampling.iter().map(|c| c.1).max().unwrap_or(1);
        for (i, &(h, v)) in self.sampling.iter().enumerate() {
            if !max_h.is_multiple_of(h) || !max_v.is_multiple_of(v) {
                return Err("Fractional JPEG sampling is unsupported".into());
            }
            let required_w = (self.width * h).div_ceil(max_h);
            let required_h = (self.height * v).div_ceil(max_v);
            if raw.plane_widths[i] < required_w
                || raw.plane_heights[i] < required_h
                || raw.planes[i].len() < raw.plane_widths[i] * raw.plane_heights[i]
            {
                return Err("Truncated decoded JPEG plane".into());
            }
        }
        let size = self
            .width
            .checked_mul(self.height)
            .and_then(|n| n.checked_mul(self.components))
            .ok_or("JPEG image size overflow")?;
        let mut pixels = Vec::new();
        pixels.try_reserve_exact(size).map_err(message)?;
        for y in 0..self.height {
            for x in 0..self.width {
                let mut samples = [0; 4];
                for (i, &(h, v)) in self.sampling.iter().enumerate() {
                    let hr = max_h / h;
                    let vr = max_v / v;
                    let w = (self.width * h).div_ceil(max_h);
                    let height = (self.height * v).div_ceil(max_v);
                    samples[i] =
                        upsample(&raw.planes[i], raw.plane_widths[i], w, height, x, y, hr, vr)?;
                }
                let converted = convert(samples, self.input_color, self.output_color);
                pixels.extend_from_slice(&converted[..self.components]);
            }
        }
        Ok(pixels)
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "plane geometry and output coordinates define the sampling operation"
)]
fn upsample(
    plane: &[u8],
    stride: usize,
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    hr: usize,
    vr: usize,
) -> CodecResult<u8> {
    let cx = x / hr;
    let cy = y / vr;
    let value = |x, y| u32::from(plane[y * stride + x]);
    let current = value(cx, cy);
    // The IJG triangle filters apply only to h2v1 and h2v2, with
    // downsampled_width > 2. Other integral ratios use sample replication.
    if hr != 2 || !matches!(vr, 1 | 2) || width <= 2 {
        return u8::try_from(current).map_err(message);
    }
    let nx = if x.is_multiple_of(2) {
        cx.saturating_sub(1)
    } else {
        (cx + 1).min(width - 1)
    };
    if vr == 1 {
        let bias = if x.is_multiple_of(2) { 1 } else { 2 };
        return u8::try_from((3 * current + value(nx, cy) + bias) >> 2).map_err(message);
    }
    let ny = if y.is_multiple_of(2) {
        cy.saturating_sub(1)
    } else {
        (cy + 1).min(height - 1)
    };
    let sum = 3 * current + value(cx, ny);
    let neighbor = 3 * value(nx, cy) + value(nx, ny);
    let bias = if x.is_multiple_of(2) { 8 } else { 7 };
    u8::try_from((3 * sum + neighbor + bias) >> 4).map_err(message)
}

pub(super) fn conversion(input: i32, output: i32, components: usize) -> CodecResult<usize> {
    let expected = match input {
        0 => components,
        1 => 1,
        2 | 3 => 3,
        4 | 5 => 4,
        _ => 0,
    };
    if expected != components || expected == 0 {
        return Err("Invalid JPEG input color space".into());
    }
    match (input, output) {
        (i, o) if i == o || o == 0 => Ok(components),
        (1..=3, 1) => Ok(1),
        (1..=3, 2 | 3) => Ok(3),
        (4 | 5, 4 | 5) => Ok(4),
        _ => Err("Unsupported JPEG color conversion".into()),
    }
}

#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "fixed-point JPEG color transforms are bounded to 8-bit samples"
)]
pub(super) fn convert(mut samples: [u8; 4], input: i32, output: i32) -> [u8; 4] {
    if input == output || output == 0 {
        return samples;
    }

    if input == 1 {
        samples[1] = samples[0];
        samples[2] = samples[0];
    }

    if input == 3 || input == 5 {
        if output == 1 {
            return samples;
        }
        let (r, g, b) =
            libjpeg_turbo_rs::decode::color::ycbcr_to_rgb_pixel(samples[0], samples[1], samples[2]);
        samples[..3].copy_from_slice(&[r, g, b]);
        if input == 5 {
            for value in &mut samples[..3] {
                *value = 255 - *value;
            }
        }
        return samples;
    }

    if output == 5 {
        for value in &mut samples[..3] {
            *value = 255 - *value;
        }
    }

    if output == 1 || output == 3 || output == 5 {
        let [r, g, b, _] = samples.map(i32::from);
        samples[0] = ((19595 * r + 38470 * g + 7471 * b + 32768) >> 16) as u8;
        samples[1] = ((-11059 * r - 21709 * g + 32768 * b + 8_421_375) >> 16) as u8;
        samples[2] = ((32768 * r - 27439 * g - 5329 * b + 8_421_375) >> 16) as u8;
    }
    samples
}
