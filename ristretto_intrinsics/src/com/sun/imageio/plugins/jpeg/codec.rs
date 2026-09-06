//! Rust JPEG state and table persistence for the `ImageIO` callback protocol.
#![expect(
    clippy::indexing_slicing,
    reason = "marker lengths and table indices are validated before indexing"
)]

use libjpeg_turbo_rs::HuffmanTableDef;
use libjpeg_turbo_rs::common::quant_table::ZIGZAG_ORDER;
use libjpeg_turbo_rs::encode::{marker_writer, tables as standard};

mod decode;
mod encode;

type CodecResult<T> = Result<T, String>;

#[derive(Clone, Debug)]
pub(super) struct Tables {
    pub nq: i32,
    pub ndc: i32,
    pub nac: i32,
    pub q: [[u32; 64]; 4],
    pub dc_bits: [[u8; 17]; 4],
    pub dc_values: [[u8; 256]; 4],
    pub ac_bits: [[u8; 17]; 4],
    pub ac_values: [[u8; 256]; 4],
}

impl Default for Tables {
    fn default() -> Self {
        Self {
            nq: 0,
            ndc: 0,
            nac: 0,
            q: [[0; 64]; 4],
            dc_bits: [[0; 17]; 4],
            dc_values: [[0; 256]; 4],
            ac_bits: [[0; 17]; 4],
            ac_values: [[0; 256]; 4],
        }
    }
}

#[derive(Clone, Debug, Default)]
pub(super) struct Config {
    pub width: i32,
    pub height: i32,
    pub components: i32,
    pub input_space: i32,
    pub output_space: i32,
    pub optimize: i32,
    pub progressive: i32,
    pub restart: i32,
    pub write_q: i32,
    pub write_h: i32,
    pub num_scans: i32,
    pub ids: [i32; 4],
    pub h: [i32; 4],
    pub v: [i32; 4],
    pub qsel: [i32; 4],
}

#[derive(Clone, Debug, Default)]
struct Bank {
    q: [Option<[u16; 64]>; 4],
    dc: [Option<HuffmanTableDef>; 4],
    ac: [Option<HuffmanTableDef>; 4],
}

impl Bank {
    fn install(&mut self, tables: &Tables) -> CodecResult<()> {
        if ![tables.nq, tables.ndc, tables.nac]
            .into_iter()
            .all(|n| (0..=4).contains(&n))
        {
            return Err("Invalid JPEG table count".into());
        }
        for i in 0..usize::try_from(tables.nq).map_err(message)? {
            let mut q = [0; 64];
            for (dst, &src) in q.iter_mut().zip(&tables.q[i]) {
                *dst = u16::try_from(src).map_err(message)?;
            }
            self.q[i] = Some(q);
        }
        for (count, bits, values, dest) in [
            (tables.ndc, &tables.dc_bits, &tables.dc_values, &mut self.dc),
            (tables.nac, &tables.ac_bits, &tables.ac_values, &mut self.ac),
        ] {
            for i in 0..usize::try_from(count).map_err(message)? {
                let n: usize = bits[i].iter().skip(1).map(|&n| usize::from(n)).sum();
                if n > 256 {
                    return Err("Invalid JPEG Huffman table length".into());
                }
                let table = HuffmanTableDef {
                    bits: bits[i],
                    values: values[i][..n].to_vec(),
                };
                validate_huffman(&table)?;
                dest[i] = Some(table);
            }
        }
        Ok(())
    }

    fn defaults(&mut self) {
        for (i, bits, values, ac_bits, ac_values) in [
            (
                0,
                standard::DC_LUMINANCE_BITS,
                standard::DC_LUMINANCE_VALUES,
                standard::AC_LUMINANCE_BITS,
                standard::AC_LUMINANCE_VALUES,
            ),
            (
                1,
                standard::DC_CHROMINANCE_BITS,
                standard::DC_CHROMINANCE_VALUES,
                standard::AC_CHROMINANCE_BITS,
                standard::AC_CHROMINANCE_VALUES,
            ),
        ] {
            self.dc[i] = Some(HuffmanTableDef {
                bits,
                values: values.to_vec(),
            });
            self.ac[i] = Some(HuffmanTableDef {
                bits: ac_bits,
                values: ac_values.to_vec(),
            });
        }
    }

    fn emit(&self, out: &mut Vec<u8>, q: bool, h: bool) {
        for id in 0u8..4 {
            let i = usize::from(id);
            if q && let Some(table) = &self.q[i] {
                marker_writer::write_dqt(out, id, table);
            }
            if h {
                for (class, tables) in [(0, &self.dc), (1, &self.ac)] {
                    if let Some(table) = &tables[i] {
                        marker_writer::write_dht(out, class, id, &table.bits, &table.values);
                    }
                }
            }
        }
    }

    fn parse(&mut self, marker: u8, mut data: &[u8]) -> CodecResult<()> {
        while !data.is_empty() {
            let info = data[0];
            let id = usize::from(info & 15);
            if id > 3 || info >> 4 > 1 {
                return Err("Invalid JPEG table selector".into());
            }
            data = &data[1..];
            if marker == 0xdb {
                let bytes = if info >> 4 == 0 { 64 } else { 128 };
                if data.len() < bytes {
                    return Err("Truncated JPEG quantization table".into());
                }
                let mut q = [0; 64];
                for (k, &natural) in ZIGZAG_ORDER.iter().enumerate() {
                    q[natural] = if bytes == 64 {
                        u16::from(data[k])
                    } else {
                        u16::from_be_bytes([data[k * 2], data[k * 2 + 1]])
                    };
                }
                self.q[id] = Some(q);
                data = &data[bytes..];
            } else {
                if data.len() < 16 {
                    return Err("Truncated JPEG Huffman table".into());
                }
                let mut bits = [0; 17];
                bits[1..].copy_from_slice(&data[..16]);
                let n: usize = bits.iter().map(|&b| usize::from(b)).sum();
                if n > 256 || data.len() < 16 + n {
                    return Err("Invalid JPEG Huffman table".into());
                }
                let table = HuffmanTableDef {
                    bits,
                    values: data[16..16 + n].to_vec(),
                };
                validate_huffman(&table)?;
                if info >> 4 == 0 {
                    self.dc[id] = Some(table);
                } else {
                    self.ac[id] = Some(table);
                }
                data = &data[16 + n..];
            }
        }
        Ok(())
    }
}

fn validate_huffman(table: &HuffmanTableDef) -> CodecResult<()> {
    let mut slots = 1i32;
    let mut count = 0;
    for &n in table.bits.iter().skip(1) {
        slots = slots * 2 - i32::from(n);
        count += usize::from(n);
        // JPEG reserves the all-ones code for entropy segment padding.
        if slots <= 0 {
            return Err("Invalid JPEG Huffman code lengths".into());
        }
    }
    if count == 0 || count > 256 || count != table.values.len() {
        return Err("Invalid JPEG Huffman symbol count".into());
    }
    let mut seen = [false; 256];
    for &symbol in &table.values {
        if std::mem::replace(&mut seen[usize::from(symbol)], true) {
            return Err("Duplicate JPEG Huffman symbol".into());
        }
    }
    Ok(())
}

fn message(error: impl std::fmt::Display) -> String {
    error.to_string()
}

/// A marker's payload excludes the marker and its length field.
#[derive(Debug)]
struct Segment<'a> {
    code: u8,
    start: usize,
    end: usize,
    data: &'a [u8],
}

/// Entropy byte stuffing and restart markers are skipped, so this also works
/// between successive scans. A header-only caller stops immediately at SOS.
fn segment<'a>(bytes: &'a [u8], pos: &mut usize) -> CodecResult<Option<Segment<'a>>> {
    while *pos < bytes.len() {
        if bytes[*pos] != 255 {
            *pos += 1;
            continue;
        }
        let start = *pos;
        while *pos < bytes.len() && bytes[*pos] == 255 {
            *pos += 1;
        }
        let Some(&code) = bytes.get(*pos) else {
            return Ok(None);
        };
        *pos += 1;
        if code == 0 || (0xd0..=0xd7).contains(&code) {
            continue;
        }
        if matches!(code, 0xd8 | 0xd9 | 1) {
            return Ok(Some(Segment {
                code,
                start,
                end: *pos,
                data: &[],
            }));
        }
        let pair = bytes
            .get(*pos..*pos + 2)
            .ok_or("Truncated JPEG marker length")?;
        let len = usize::from(u16::from_be_bytes([pair[0], pair[1]]));
        if len < 2 {
            return Err("Invalid JPEG marker length".into());
        }
        let data = bytes
            .get(*pos + 2..*pos + len)
            .ok_or("Truncated JPEG marker")?;
        *pos += len;
        return Ok(Some(Segment {
            code,
            start,
            end: *pos,
            data,
        }));
    }
    Ok(None)
}

#[derive(Debug, Default)]
struct Header {
    width: usize,
    height: usize,
    color: i32,
    components: Vec<(u8, usize, usize, usize)>,
    progressive: bool,
    multiple: bool,
}

impl Header {
    fn parse(part: &Segment<'_>) -> CodecResult<Self> {
        if !matches!(part.code, 0xc0..=0xc2) {
            return Err("Unsupported JPEG coding process".into());
        }
        let data = part.data;
        if data.len() < 6 || data[0] != 8 {
            return Err("Unsupported JPEG sample precision".into());
        }
        let n = usize::from(data[5]);
        if !(1..=4).contains(&n) || data.len() != 6 + 3 * n {
            return Err("Invalid JPEG component count".into());
        }
        let height = usize::from(u16::from_be_bytes([data[1], data[2]]));
        let width = usize::from(u16::from_be_bytes([data[3], data[4]]));
        if width == 0 || height == 0 {
            return Err("Empty JPEG image".into());
        }

        let mut components = Vec::new();
        for c in data[6..].as_chunks::<3>().0 {
            let (h, v, q) = (
                usize::from(c[1] >> 4),
                usize::from(c[1] & 15),
                usize::from(c[2]),
            );
            if !(1..=4).contains(&h)
                || !(1..=4).contains(&v)
                || q > 3
                || components.iter().any(|&(id, _, _, _)| id == c[0])
            {
                return Err("Invalid JPEG component specification".into());
            }
            components.push((c[0], h, v, q));
        }
        Ok(Self {
            width,
            height,
            color: 0,
            components,
            progressive: part.code == 0xc2,
            multiple: part.code == 0xc2,
        })
    }

    fn scan(&mut self, data: &[u8]) -> CodecResult<()> {
        let count = usize::from(*data.first().ok_or("Empty JPEG scan")?);
        if count == 0 || count > self.components.len() || data.len() != 4 + 2 * count {
            return Err("Invalid JPEG scan component count or length".into());
        }
        let mut seen = [false; 4];
        for selector in data[1..=2 * count].as_chunks::<2>().0 {
            let index = self
                .components
                .iter()
                .position(|c| c.0 == selector[0])
                .ok_or("Unknown JPEG scan component")?;
            if std::mem::replace(&mut seen[index], true) {
                return Err("Duplicate JPEG scan component".into());
            }
        }
        self.multiple |= count < self.components.len();
        Ok(())
    }
    fn info(&self) -> CodecResult<[i32; 6]> {
        Ok([
            i32::try_from(self.width).map_err(message)?,
            i32::try_from(self.height).map_err(message)?,
            self.color,
            self.default_color(),
            i32::try_from(self.components.len()).map_err(message)?,
            i32::from(self.multiple),
        ])
    }
    fn default_color(&self) -> i32 {
        match self.color {
            3 => 2,
            5 => 4,
            color => color,
        }
    }
}

#[derive(Debug)]
pub(super) struct Codec {
    writer: bool,
    bank: Bank,
    input: Vec<u8>,
    header: Option<Header>,
    reader: Option<decode::Read>,
    encoder: Option<encode::Write>,
    output: Vec<u8>,
    warning: Option<String>,
}

impl Codec {
    #[expect(
        clippy::unnecessary_wraps,
        reason = "fallible constructor interface shared with Java allocation error handling"
    )]
    pub fn new(writer: bool) -> CodecResult<Self> {
        Ok(Self {
            writer,
            bank: Bank::default(),
            input: Vec::new(),
            header: None,
            reader: None,
            encoder: None,
            output: Vec::new(),
            warning: None,
        })
    }

    fn kind(&self, writer: bool) -> CodecResult<()> {
        if writer == self.writer {
            Ok(())
        } else {
            Err("Wrong JPEG codec kind".into())
        }
    }

    pub fn reset(&mut self) {
        self.input.clear();
        self.header = None;
        self.reader = None;
        self.encoder = None;
        self.output.clear();
        self.warning = None;
        // Quantization/Huffman tables survive jpeg_abort, for abbreviated images.
    }

    pub fn warning(&mut self) -> Option<String> {
        self.warning.take()
    }

    pub fn header(&mut self, bytes: &[u8]) -> CodecResult<([i32; 6], bool)> {
        self.kind(false)?;
        self.reset();
        if !bytes.starts_with(&[255, 216]) {
            return Err("Not a JPEG stream".into());
        }

        let mut pos = 2;
        let mut header = None;
        let mut adobe = None;
        let mut jfif = false;
        let mut exif = false;

        loop {
            let before = pos;
            let part = match segment(bytes, &mut pos) {
                Ok(Some(part)) => part,
                Ok(None) => {
                    self.warning = Some("Premature end of JPEG stream".into());
                    break;
                }
                Err(error) if header.is_none() && error.starts_with("Truncated") => {
                    self.warning = Some(error);
                    break;
                }
                Err(error) => return Err(error),
            };
            if part.start > before {
                self.warning = Some("Extraneous JPEG data before marker".into());
            }
            match part.code {
                0xdb | 0xc4 => self.bank.parse(part.code, part.data)?,
                0xe0 => jfif |= part.data.starts_with(b"JFIF\0"),
                0xe1 => exif |= part.data.starts_with(b"Exif\0"),
                0xee if part.data.starts_with(b"Adobe") && part.data.len() >= 12 => {
                    adobe = Some(part.data[11]);
                }
                0xc0..=0xc3 | 0xc5..=0xcb | 0xcd..=0xcf => {
                    if header.is_some() {
                        return Err("Duplicate JPEG frame".into());
                    }
                    header = Some(Header::parse(&part)?);
                }
                0xda => {
                    let h = header.as_mut().ok_or("JPEG scan without frame")?;
                    h.scan(part.data)?;
                    break;
                }
                0xd9 => break,
                0xd8 => return Err("Duplicate JPEG SOI".into()),
                _ => {}
            }
        }

        let Some(mut h) = header else {
            return Ok(([0; 6], true));
        };
        h.color = match h.components.len() {
            1 => 1,
            3 => match adobe {
                Some(0) => 2,
                Some(1) => 3,
                Some(_) => 0,
                None if jfif => 3,
                None => {
                    let ids: Vec<_> = h.components.iter().map(|c| c.0).collect();
                    if ids == b"RGB"
                        || (!exif
                            && ids != [1, 2, 3]
                            && h.components
                                .iter()
                                .all(|c| (c.1, c.2) == (h.components[0].1, h.components[0].2)))
                    {
                        2
                    } else {
                        3
                    }
                }
            },
            4 => match adobe {
                Some(2) => 5,
                Some(0) | None => {
                    let c = &h.components;
                    if (c[1].1 > c[0].1 && c[2].1 > c[0].1) || (c[1].2 > c[0].2 && c[2].2 > c[0].2)
                    {
                        5
                    } else {
                        4
                    }
                }
                Some(_) => 0,
            },
            _ => 0,
        };
        let info = h.info()?;
        self.header = Some(h);
        self.input = bytes.to_vec();
        Ok((info, false))
    }

    pub fn start_read(&mut self, color: i32, tables: &Tables) -> CodecResult<[i32; 3]> {
        self.kind(false)?;
        let mut supplied = Bank::default();
        supplied.install(tables)?;
        if self.bank.q[0].is_none() {
            self.bank.q = supplied.q;
        }
        if self.bank.dc[0].is_none() {
            self.bank.dc = supplied.dc;
        }
        if self.bank.ac[0].is_none() {
            self.bank.ac = supplied.ac;
        }
        let header = self.header.as_ref().ok_or("JPEG header not read")?;
        let reader = decode::Read::new(&self.input, header, &self.bank, color)?;
        let info = [
            i32::try_from(header.width).map_err(message)?,
            i32::try_from(header.height).map_err(message)?,
            i32::try_from(reader.components).map_err(message)?,
        ];
        self.reader = Some(reader);
        Ok(info)
    }

    pub fn start_pass(&mut self, pass: i32) -> CodecResult<i32> {
        self.kind(false)?;
        self.reader
            .as_mut()
            .ok_or("JPEG read not started")?
            .start_pass(pass)
    }

    pub fn read_row(&mut self, row: &mut [u8]) -> CodecResult<()> {
        self.kind(false)?;
        self.reader
            .as_mut()
            .ok_or("JPEG read not started")?
            .row(row)
    }

    pub fn finish_pass(&mut self) -> CodecResult<Option<i32>> {
        self.kind(false)?;
        self.reader
            .as_ref()
            .ok_or("JPEG read not started")?
            .finish_pass()
    }

    pub fn start_write(
        &mut self,
        config: Config,
        tables: &Tables,
        scans: &[i32],
    ) -> CodecResult<()> {
        self.kind(true)?;
        self.reset();
        self.bank.defaults();
        self.bank.install(tables)?;
        self.encoder = Some(encode::Write::new(config, &self.bank, scans)?);
        // Java writes APP/COM metadata after SOI, before the codec's tables/SOF.
        self.output.extend_from_slice(&[255, 216]);
        Ok(())
    }

    pub fn write_row(&mut self, row: &[u8]) -> CodecResult<()> {
        self.kind(true)?;
        self.encoder
            .as_mut()
            .ok_or("JPEG write not started")?
            .row(row)
    }

    pub fn finish_write(&mut self) -> CodecResult<()> {
        self.kind(true)?;
        let writer = self.encoder.take().ok_or("JPEG write not started")?;
        self.output.extend(writer.finish(&mut self.bank)?);
        Ok(())
    }

    pub fn write_tables(&mut self, tables: &Tables) -> CodecResult<()> {
        self.kind(true)?;
        let mut selected = Bank::default();
        selected.install(tables)?;
        self.bank.install(tables)?;
        self.output.extend_from_slice(&[255, 216]);
        selected.emit(&mut self.output, true, true);
        self.output.extend_from_slice(&[255, 217]);
        Ok(())
    }

    pub fn output(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encode(progressive: bool) -> CodecResult<Vec<u8>> {
        let mut encoder = Codec::new(true)?;
        let tables = Tables {
            nq: 1,
            q: [[16; 64]; 4],
            ..Tables::default()
        };
        let config = Config {
            width: 9,
            height: 7,
            components: 1,
            input_space: 1,
            output_space: 1,
            optimize: 1,
            progressive: i32::from(progressive),
            write_q: 1,
            write_h: 1,
            ids: [1, 0, 0, 0],
            h: [1; 4],
            v: [1; 4],
            restart: 1,
            ..Config::default()
        };
        encoder.start_write(config, &tables, &[])?;
        let mut data = encoder.output();
        for _ in 0..7 {
            encoder.write_row(&[128; 9])?;
            data.extend(encoder.output());
        }
        encoder.finish_write()?;
        data.extend(encoder.output());
        Ok(data)
    }

    #[test]
    fn baseline_and_progressive_pixels() -> CodecResult<()> {
        for progressive in [false, true] {
            let data = encode(progressive)?;
            assert!(data.starts_with(&[255, 216]));
            assert!(data.ends_with(&[255, 217]));
            let mut decoder = Codec::new(false)?;
            let (info, only_tables) = decoder.header(&data)?;
            assert!(!only_tables);
            assert_eq!(info, [9, 7, 1, 1, 1, i32::from(progressive)]);
            assert_eq!(decoder.start_read(1, &Tables::default())?, [9, 7, 1]);
            let mut pass = 0;
            let mut passes = 0;
            loop {
                decoder.start_pass(pass)?;
                let mut row = [0; 9];
                for _ in 0..7 {
                    decoder.read_row(&mut row)?;
                    assert_eq!(row, [128; 9]);
                }
                passes += 1;
                let Some(next) = decoder.finish_pass()? else {
                    break;
                };
                pass = next;
            }
            assert_eq!(passes > 1, progressive);
        }
        Ok(())
    }

    #[test]
    fn explicit_progressive_scan_script() -> CodecResult<()> {
        let mut encoder = Codec::new(true)?;
        let tables = Tables {
            nq: 1,
            q: [[16; 64]; 4],
            ..Tables::default()
        };
        let config = Config {
            width: 8,
            height: 8,
            components: 1,
            input_space: 1,
            output_space: 1,
            optimize: 1,
            progressive: 1,
            num_scans: 2,
            write_q: 1,
            write_h: 1,
            ids: [1, 0, 0, 0],
            h: [1; 4],
            v: [1; 4],
            ..Config::default()
        };
        // DC followed by AC, with no successive approximation.
        let scans = [1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 63, 0, 0];
        encoder.start_write(config, &tables, &scans)?;
        for _ in 0..8 {
            encoder.write_row(&[128; 8])?;
        }
        encoder.finish_write()?;
        let mut decoder = Codec::new(false)?;
        decoder.header(&encoder.output())?;
        decoder.start_read(1, &Tables::default())?;
        decoder.start_pass(0)?;
        let mut row = [0; 8];
        for _ in 0..8 {
            decoder.read_row(&mut row)?;
        }
        assert_eq!(decoder.finish_pass()?, Some(1));
        decoder.start_pass(1)?;
        for _ in 0..8 {
            decoder.read_row(&mut row)?;
            assert_eq!(row, [128; 8]);
        }
        assert_eq!(decoder.finish_pass()?, None);
        Ok(())
    }

    #[test]
    fn progressive_pass_beyond_end_is_clamped() -> CodecResult<()> {
        let data = encode(true)?;
        let mut decoder = Codec::new(false)?;
        decoder.header(&data)?;
        decoder.start_read(1, &Tables::default())?;
        let actual = decoder.start_pass(i32::MAX)?;
        assert!((0..100).contains(&actual));
        for _ in 0..7 {
            decoder.read_row(&mut [0; 9])?;
        }
        assert_eq!(decoder.finish_pass()?, None);
        Ok(())
    }

    #[test]
    fn rgb_cmyk_and_component_settings() -> CodecResult<()> {
        for (color, components, ids, samples) in [
            (2, 3, [82, 71, 66, 0], vec![32, 64, 128]),
            (4, 4, [67, 77, 89, 75], vec![32, 64, 128, 160]),
        ] {
            let mut encoder = Codec::new(true)?;
            let tables = Tables {
                nq: 1,
                q: [[16; 64]; 4],
                ..Tables::default()
            };
            let config = Config {
                width: 8,
                height: 8,
                components,
                input_space: color,
                output_space: color,
                optimize: 1,
                write_q: 1,
                write_h: 1,
                ids,
                h: [1; 4],
                v: [1; 4],
                ..Config::default()
            };
            let row = samples.repeat(8);
            encoder.start_write(config, &tables, &[])?;
            for _ in 0..8 {
                encoder.write_row(&row)?;
            }
            encoder.finish_write()?;
            let data = encoder.output();
            let mut decoder = Codec::new(false)?;
            assert_eq!(
                decoder.header(&data)?.0,
                [8, 8, color, color, components, 0]
            );
            decoder.start_read(color, &Tables::default())?;
            let mut output = vec![0; row.len()];
            for _ in 0..8 {
                decoder.read_row(&mut output)?;
                assert_eq!(output, row);
            }
        }
        Ok(())
    }

    #[test]
    fn errors_leave_codec_reusable() -> CodecResult<()> {
        let mut decoder = Codec::new(false)?;
        assert!(decoder.header(b"invalid JPEG").is_err());
        decoder.reset();
        let data = encode(false)?;
        decoder.header(&data)?;
        decoder.start_read(1, &Tables::default())?;
        assert!(decoder.read_row(&mut [0; 1]).is_err());
        decoder.reset();
        decoder.header(&data)?;
        decoder.start_read(1, &Tables::default())?;
        decoder.read_row(&mut [0; 9])?;
        assert!(decoder.finish_write().is_err());
        Ok(())
    }

    #[test]
    fn table_stream_and_truncated_header() -> CodecResult<()> {
        let tables = Tables {
            nq: 1,
            q: [[23; 64]; 4],
            ..Tables::default()
        };
        let mut encoder = Codec::new(true)?;
        encoder.write_tables(&tables)?;
        let data = encoder.output();
        let mut decoder = Codec::new(false)?;
        assert!(decoder.header(&data)?.1);
        // A truncated tables-only stream reports a warning.
        decoder.header(&[255, 216, 255, 219, 0, 67, 0])?;
        assert!(decoder.warning().is_some());
        assert!(encoder.write_tables(&Tables { nq: 5, ..tables }).is_err());
        Ok(())
    }
    #[test]
    fn progressive_refinement_with_unusual_components() -> CodecResult<()> {
        for (input, color, components) in [(0, 0, 2), (2, 2, 3), (2, 3, 3), (4, 4, 4), (4, 5, 4)] {
            let mut reconstructions = Vec::new();
            for progressive in [false, true] {
                let mut encoder = Codec::new(true)?;
                let tables = Tables {
                    nq: 4,
                    q: [[5; 64], [7; 64], [13; 64], [19; 64]],
                    ..Tables::default()
                };
                let sampled = matches!(color, 3 | 5);
                let config = Config {
                    width: 31,
                    height: 19,
                    components,
                    input_space: input,
                    output_space: color,
                    optimize: 1,
                    progressive: i32::from(progressive),
                    restart: 3,
                    write_q: 1,
                    write_h: 1,
                    ids: if color == 2 {
                        [82, 71, 66, 0]
                    } else {
                        [1, 2, 3, 4]
                    },
                    h: if sampled { [2, 2, 1, 1] } else { [1; 4] },
                    v: [1; 4],
                    qsel: [3, 2, 1, 0],
                    ..Config::default()
                };
                encoder.start_write(config, &tables, &[])?;
                for y in 0..19 {
                    let mut row = Vec::new();
                    for x in 0..31 {
                        for k in 0..components {
                            row.push(
                                u8::try_from((x * 13 + y * 19 + x * y % 47 + k * 29) & 255)
                                    .map_err(message)?,
                            );
                        }
                    }
                    encoder.write_row(&row)?;
                }
                encoder.finish_write()?;
                let mut bytes = encoder.output();
                if color == 5 {
                    let mut adobe = Vec::new();
                    marker_writer::write_app14_adobe(&mut adobe, 2);
                    bytes.splice(2..2, adobe);
                }
                let mut decoder = Codec::new(false)?;
                assert_eq!(decoder.header(&bytes)?.0[2], color);
                decoder.start_read(color, &Tables::default())?;
                decoder.start_pass(i32::MAX)?;
                let mut image = Vec::new();
                for _ in 0..19 {
                    let mut row = vec![0; 31 * usize::try_from(components).map_err(message)?];
                    decoder.read_row(&mut row)?;
                    image.extend(row);
                }
                assert_eq!(decoder.finish_pass()?, None);
                reconstructions.push(image);
            }
            assert_eq!(
                reconstructions[0], reconstructions[1],
                "color space {color}"
            );
        }
        Ok(())
    }

    #[test]
    fn malformed_tables_and_scan_scripts_are_errors() -> CodecResult<()> {
        let mut tables = Tables {
            ndc: 1,
            ..Tables::default()
        };
        tables.dc_bits[0][1] = 3;
        let mut encoder = Codec::new(true)?;
        assert!(encoder.write_tables(&tables).is_err());
        tables.dc_bits[0][1] = 1;
        tables.dc_bits[0][2] = 1;
        assert!(encoder.write_tables(&tables).is_err()); // duplicate symbols
        tables = Tables {
            nq: 1,
            q: [[16; 64]; 4],
            ..Tables::default()
        };
        let config = Config {
            width: 8,
            height: 8,
            components: 1,
            input_space: 1,
            output_space: 1,
            progressive: 1,
            num_scans: 1,
            h: [1; 4],
            v: [1; 4],
            ..Config::default()
        };
        for script in [
            [1, 4, 0, 0, 0, 0, 0, 0, 0],  // component outside the frame
            [1, 0, 0, 0, 0, 1, 63, 0, 0], // AC before DC
            [1, 0, 0, 0, 0, 0, 0, 2, 0],  // invalid successive approximation
        ] {
            assert!(
                encoder
                    .start_write(config.clone(), &tables, &script)
                    .is_err()
            );
        }
        Ok(())
    }
}
