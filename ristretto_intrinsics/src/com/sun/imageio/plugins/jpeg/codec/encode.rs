//! JPEG marker and entropy encoding with Java-selected tables and scan scripts.
#![expect(
    clippy::indexing_slicing,
    reason = "scan/component indices and fixed-size coefficient arrays are validated at construction"
)]

use super::decode::{conversion, convert};
use super::{Bank, CodecResult, Config, message, validate_huffman};
use libjpeg_turbo_rs::HuffmanTableDef;
use libjpeg_turbo_rs::common::quant_table::ZIGZAG_ORDER;
use libjpeg_turbo_rs::encode::fdct::fdct_islow;
use libjpeg_turbo_rs::encode::huff_opt::gen_optimal_table;
use libjpeg_turbo_rs::encode::huffman_encode::{HuffTable, build_huff_table};
use libjpeg_turbo_rs::encode::marker_writer;
use libjpeg_turbo_rs::encode::progressive::{ProgressiveScan, simple_progression_for};

#[derive(Debug)]
struct Component {
    id: u8,
    h: usize,
    v: usize,
    q: usize,
    table: usize,
    blocks_x: usize,
    blocks_y: usize,
    data_x: usize,
    data_y: usize,
    blocks: Vec<[i16; 64]>,
}

#[derive(Debug)]
pub(super) struct Write {
    config: Config,
    width: usize,
    height: usize,
    input_components: usize,
    components: Vec<Component>,
    scans: Vec<ProgressiveScan>,
    progressive: bool,
    pixels: Vec<u8>,
    max_h: usize,
    max_v: usize,
}

impl Write {
    pub fn new(config: Config, bank: &Bank, script: &[i32]) -> CodecResult<Self> {
        if !(1..=65500).contains(&config.width)
            || !(1..=65500).contains(&config.height)
            || !(1..=4).contains(&config.components)
            || !(0..=65535).contains(&config.restart)
            || usize::try_from(config.num_scans)
                .ok()
                .and_then(|n| n.checked_mul(9))
                != Some(script.len())
        {
            return Err("Invalid JPEG dimensions, components, restart interval or scans".into());
        }
        let width = usize::try_from(config.width).map_err(message)?;
        let height = usize::try_from(config.height).map_err(message)?;
        let input_components = usize::try_from(config.components).map_err(message)?;
        let n = conversion(config.input_space, config.output_space, input_components)?;
        width
            .checked_mul(height)
            .and_then(|size| size.checked_mul(n.max(input_components)))
            .ok_or("JPEG image size overflow")?;
        let mut components: Vec<Component> = Vec::new();
        for i in 0..n {
            let id = u8::try_from(config.ids[i]).map_err(message)?;
            let h = usize::try_from(config.h[i]).map_err(message)?;
            let v = usize::try_from(config.v[i]).map_err(message)?;
            let q = usize::try_from(config.qsel[i]).map_err(message)?;
            if !(1..=4).contains(&h)
                || !(1..=4).contains(&v)
                || q > 3
                || components.iter().any(|c| c.id == id)
            {
                return Err("Invalid JPEG component settings".into());
            }
            let quant = bank.q[q]
                .as_ref()
                .ok_or("Missing JPEG quantization table")?;
            if quant.contains(&0) {
                return Err("Zero JPEG quantization divisor".into());
            }
            let table = usize::from(matches!(config.output_space, 3 | 5) && matches!(i, 1 | 2));
            components.push(Component {
                id,
                h,
                v,
                q,
                table,
                blocks_x: 0,
                blocks_y: 0,
                data_x: 0,
                data_y: 0,
                blocks: Vec::new(),
            });
        }
        let max_h = components.iter().map(|c| c.h).max().unwrap_or(1);
        let max_v = components.iter().map(|c| c.v).max().unwrap_or(1);
        for c in &mut components {
            if !max_h.is_multiple_of(c.h) || !max_v.is_multiple_of(c.v) {
                return Err("Fractional JPEG sampling is unsupported".into());
            }
            c.blocks_x = width.div_ceil(max_h * 8) * c.h;
            c.blocks_y = height.div_ceil(max_v * 8) * c.v;
            c.data_x = (width * c.h).div_ceil(max_h * 8);
            c.data_y = (height * c.v).div_ceil(max_v * 8);
        }

        let scans = scan_script(&config, script, n)?;
        let progressive = scans.first().is_some_and(|s| s.ss != 0 || s.se != 63);

        validate_scans(&scans, &components, progressive)?;

        Ok(Self {
            config,
            width,
            height,
            input_components,
            components,
            scans,
            progressive,
            pixels: Vec::new(),
            max_h,
            max_v,
        })
    }

    pub fn row(&mut self, samples: &[u8]) -> CodecResult<()> {
        let stride = self.width * self.input_components;
        if samples.len() < stride {
            return Err("JPEG input row is too short".into());
        }

        let out_stride = self.width * self.components.len();
        if self.pixels.len() / out_stride >= self.height {
            return Err("Too many JPEG input rows".into());
        }

        self.pixels.try_reserve(out_stride).map_err(message)?;

        for pixel in samples[..stride].chunks_exact(self.input_components) {
            let mut values = [0; 4];
            values[..self.input_components].copy_from_slice(pixel);
            let values = convert(values, self.config.input_space, self.config.output_space);
            self.pixels
                .extend_from_slice(&values[..self.components.len()]);
        }
        Ok(())
    }

    pub fn finish(mut self, bank: &mut Bank) -> CodecResult<Vec<u8>> {
        if self.pixels.len() != self.width * self.height * self.components.len() {
            return Err("Not enough JPEG scanlines".into());
        }

        self.transform(bank)?;
        self.pixels = Vec::new();
        let mut out = Vec::new();

        if self.config.write_q != 0 {
            let mut used = [false; 4];
            for c in &self.components {
                if !std::mem::replace(&mut used[c.q], true) {
                    marker_writer::write_dqt(
                        &mut out,
                        u8::try_from(c.q).map_err(message)?,
                        bank.q[c.q]
                            .as_ref()
                            .ok_or("Missing JPEG quantization table")?,
                    );
                }
            }
        }

        let component_info: Vec<_> = self
            .components
            .iter()
            .map(|c| {
                Ok((
                    c.id,
                    u8::try_from(c.h).map_err(message)?,
                    u8::try_from(c.v).map_err(message)?,
                    u8::try_from(c.q).map_err(message)?,
                ))
            })
            .collect::<CodecResult<_>>()?;
        let width = u16::try_from(self.width).map_err(message)?;
        let height = u16::try_from(self.height).map_err(message)?;
        let extended = self
            .components
            .iter()
            .any(|c| bank.q[c.q].is_some_and(|q| q.iter().any(|&v| v > 255)));
        if self.progressive {
            marker_writer::write_sof2(&mut out, width, height, &component_info);
        } else if extended {
            marker_writer::write_sof1(&mut out, width, height, &component_info);
        } else {
            marker_writer::write_sof0(&mut out, width, height, &component_info);
        }
        if self.config.restart > 0 {
            marker_writer::write_dri(
                &mut out,
                u16::try_from(self.config.restart).map_err(message)?,
            );
        }
        for scan in &self.scans {
            self.encode_scan(scan, bank, &mut out)?;
        }
        marker_writer::write_eoi(&mut out);
        Ok(out)
    }

    fn transform(&mut self, bank: &Bank) -> CodecResult<()> {
        let n = self.components.len();
        for (ci, c) in self.components.iter_mut().enumerate() {
            let quant = bank.q[c.q]
                .as_ref()
                .ok_or("Missing JPEG quantization table")?;
            let hr = self.max_h / c.h;
            let vr = self.max_v / c.v;
            c.blocks
                .try_reserve_exact(c.blocks_x * c.blocks_y)
                .map_err(message)?;
            let samples = ComponentSamples {
                pixels: &self.pixels,
                width: self.width,
                height: self.height,
                components: n,
                component: ci,
                hr,
                vr,
            };
            transform_component(c, &samples, quant)?;
        }
        Ok(())
    }

    fn encode_scan(
        &self,
        scan: &ProgressiveScan,
        bank: &mut Bank,
        out: &mut Vec<u8>,
    ) -> CodecResult<()> {
        let mut frequencies = [[[0u32; 257]; 4]; 2];
        self.visit(scan, &mut |token| {
            if let Token::Symbol(class, table, symbol) = token {
                frequencies[class][table][usize::from(symbol)] += 1;
            }
            Ok(())
        })?;
        let optimize = self.config.optimize != 0 || self.progressive;
        let mut codes: [[Option<HuffTable>; 4]; 2] =
            std::array::from_fn(|_| std::array::from_fn(|_| None));
        for class in 0..2 {
            for table in 0..4 {
                if frequencies[class][table].iter().all(|&f| f == 0) {
                    continue;
                }
                let slot = if class == 0 {
                    &mut bank.dc[table]
                } else {
                    &mut bank.ac[table]
                };
                if optimize {
                    let (bits, values) = gen_optimal_table(&frequencies[class][table]);
                    *slot = Some(HuffmanTableDef { bits, values });
                }
                let definition = slot.as_ref().ok_or("Missing JPEG Huffman table")?;
                validate_huffman(definition)?;
                codes[class][table] = Some(build_huff_table(&definition.bits, &definition.values));
                if self.config.write_h != 0 || optimize {
                    marker_writer::write_dht(
                        out,
                        u8::try_from(class).map_err(message)?,
                        u8::try_from(table).map_err(message)?,
                        &definition.bits,
                        &definition.values,
                    );
                }
            }
        }

        let scan_components: Vec<_> = scan
            .component_indices
            .iter()
            .map(|&i| {
                let c = &self.components[i];
                let table = u8::try_from(c.table).map_err(message)?;
                Ok((
                    c.id,
                    if scan.ss == 0 && scan.ah == 0 {
                        table
                    } else {
                        0
                    },
                    if scan.se > 0 { table } else { 0 },
                ))
            })
            .collect::<CodecResult<_>>()?;
        write_scan_header(out, &scan_components, scan)?;
        let mut bits = Bits::default();
        self.visit(scan, &mut |token| {
            match token {
                Token::Symbol(class, table, symbol) => {
                    let code = codes[class][table]
                        .as_ref()
                        .ok_or("Missing JPEG Huffman table")?;
                    let index = usize::from(symbol);
                    let length = code.ehufsi[index];
                    if length == 0 {
                        return Err("JPEG Huffman table cannot encode a coefficient".into());
                    }
                    bits.put(code.ehufco[index], length);
                }
                Token::Bits(value, size) => bits.put(value, size),
                Token::Restart(index) => {
                    bits.flush();
                    bits.bytes.extend_from_slice(&[255, 0xd0 + index]);
                }
            }
            Ok(())
        })?;
        bits.flush();
        out.extend(bits.bytes);
        Ok(())
    }

    fn visit(
        &self,
        scan: &ProgressiveScan,
        emit: &mut impl FnMut(Token) -> CodecResult<()>,
    ) -> CodecResult<()> {
        let single = scan.component_indices.len() == 1;
        let first = &self.components[scan.component_indices[0]];
        let (mx, my) = if single {
            (first.data_x, first.data_y)
        } else {
            (
                self.width.div_ceil(self.max_h * 8),
                self.height.div_ceil(self.max_v * 8),
            )
        };
        let mut dc = [0i16; 4];
        let restart = usize::try_from(self.config.restart).map_err(message)?;
        let mut restart_index = 0;
        for y in 0..my {
            for x in 0..mx {
                let mcu = y * mx + x;
                if restart != 0 && mcu != 0 && mcu.is_multiple_of(restart) {
                    emit(Token::Restart(restart_index))?;
                    restart_index = (restart_index + 1) % 8;
                    dc.fill(0);
                }
                for &ci in &scan.component_indices {
                    let c = &self.components[ci];
                    visit_component(c, scan, (x, y), single, &mut dc[ci], emit)?;
                }
            }
        }
        Ok(())
    }
}

fn visit_component(
    component: &Component,
    scan: &ProgressiveScan,
    (x, y): (usize, usize),
    single: bool,
    dc: &mut i16,
    emit: &mut impl FnMut(Token) -> CodecResult<()>,
) -> CodecResult<()> {
    let (h, v) = if single {
        (1, 1)
    } else {
        (component.h, component.v)
    };
    for dy in 0..v {
        for dx in 0..h {
            let block = &component.blocks[(y * v + dy) * component.blocks_x + x * h + dx];
            encode_block(block, scan, component.table, dc, emit)?;
        }
    }
    Ok(())
}

/// Interleaved pixels and sampling geometry for one encoded component.
struct ComponentSamples<'a> {
    pixels: &'a [u8],
    width: usize,
    height: usize,
    components: usize,
    component: usize,
    hr: usize,
    vr: usize,
}

fn transform_component(
    component: &mut Component,
    image: &ComponentSamples<'_>,
    quant: &[u16; 64],
) -> CodecResult<()> {
    for by in 0..component.blocks_y {
        for bx in 0..component.blocks_x {
            component
                .blocks
                .push(transform_block(image, bx, by, quant)?);
        }
    }
    Ok(())
}

fn transform_block(
    image: &ComponentSamples<'_>,
    bx: usize,
    by: usize,
    quant: &[u16; 64],
) -> CodecResult<[i16; 64]> {
    let mut samples = [0i16; 64];
    for (k, sample) in samples.iter_mut().enumerate() {
        let x = (bx * 8 + k % 8) * image.hr;
        let y = (by * 8 + k / 8) * image.vr;
        let sum = sample_sum(image, x, y);
        let count = image.hr * image.vr;
        let bias = if matches!((image.hr, image.vr), (2, 1 | 2)) {
            count / 2 - 1 + (k % 2)
        } else {
            count / 2
        };
        *sample = i16::try_from((sum + bias) / count).map_err(message)? - 128;
    }
    let mut dct = [0; 64];
    fdct_islow(&samples, &mut dct);
    let mut coefficients = [0; 64];
    for ((coefficient, &value), &q) in coefficients.iter_mut().zip(&dct).zip(quant) {
        let divisor = i32::from(q) * 8;
        let magnitude = (value.abs() + divisor / 2) / divisor;
        *coefficient = i16::try_from(magnitude * value.signum()).map_err(message)?;
    }
    Ok(coefficients)
}

fn sample_sum(image: &ComponentSamples<'_>, x: usize, y: usize) -> usize {
    let mut sum = 0;
    for dy in 0..image.vr {
        for dx in 0..image.hr {
            sum += usize::from(
                image.pixels[((y + dy).min(image.height - 1) * image.width
                    + (x + dx).min(image.width - 1))
                    * image.components
                    + image.component],
            );
        }
    }
    sum
}

/// AC tables apply to every nonempty AC band, including bands ending before
/// coefficient 63. Preserve the selected tables in each progressive SOS.
fn write_scan_header(
    out: &mut Vec<u8>,
    components: &[(u8, u8, u8)],
    scan: &ProgressiveScan,
) -> CodecResult<()> {
    let mut data = vec![u8::try_from(components.len()).map_err(message)?];
    for &(id, dc, ac) in components {
        data.extend_from_slice(&[id, (dc << 4) | ac]);
    }
    data.extend_from_slice(&[scan.ss, scan.se, (scan.ah << 4) | scan.al]);
    marker_writer::write_marker(out, 0xda, &data);
    Ok(())
}

fn scan_script(config: &Config, script: &[i32], n: usize) -> CodecResult<Vec<ProgressiveScan>> {
    let scans = if config.progressive == 0 {
        vec![ProgressiveScan {
            component_indices: (0..n).collect(),
            ss: 0,
            se: 63,
            ah: 0,
            al: 0,
        }]
    } else if script.is_empty() {
        simple_progression_for(n, config.output_space == 3)
    } else {
        let mut scans = Vec::new();
        for s in script.as_chunks::<9>().0 {
            let count = usize::try_from(s[0]).map_err(message)?;
            if !(1..=4).contains(&count) {
                return Err("Invalid JPEG scan component count".into());
            }
            scans.push(ProgressiveScan {
                component_indices: s[1..=count]
                    .iter()
                    .map(|&i| usize::try_from(i).map_err(message))
                    .collect::<CodecResult<_>>()?,
                ss: u8::try_from(s[5]).map_err(message)?,
                se: u8::try_from(s[6]).map_err(message)?,
                ah: u8::try_from(s[7]).map_err(message)?,
                al: u8::try_from(s[8]).map_err(message)?,
            });
        }
        scans
    };
    Ok(scans)
}

fn validate_scans(
    scans: &[ProgressiveScan],
    components: &[Component],
    progressive: bool,
) -> CodecResult<()> {
    let mut transmitted = [[-1i16; 64]; 4];
    for scan in scans {
        let indices = &scan.component_indices;
        if indices.is_empty()
            || indices.len() > 4
            || indices.iter().any(|&i| i >= components.len())
            || !indices.windows(2).all(|w| w[0] < w[1])
            || scan.se > 63
            || scan.ss > scan.se
            || scan.ah > 13
            || scan.al > 13
        {
            return Err("Invalid JPEG scan script".into());
        }
        if indices.len() > 1
            && indices
                .iter()
                .map(|&i| components[i].h * components[i].v)
                .sum::<usize>()
                > 10
        {
            return Err("Too many JPEG blocks per MCU".into());
        }
        if progressive {
            if (scan.ss == 0 && scan.se != 0)
                || (scan.ss != 0 && indices.len() != 1)
                || (scan.ah != 0 && scan.al + 1 != scan.ah)
            {
                return Err("Invalid progressive JPEG scan".into());
            }
        } else if scan.ss != 0 || scan.se != 63 || scan.ah != 0 || scan.al != 0 {
            return Err("Invalid sequential JPEG scan".into());
        }
        for &ci in indices {
            if scan.ss != 0 && transmitted[ci][0] < 0 {
                return Err("JPEG AC scan precedes DC scan".into());
            }
            for coefficient in &mut transmitted[ci][usize::from(scan.ss)..=usize::from(scan.se)] {
                if (scan.ah == 0 && *coefficient != -1)
                    || (scan.ah != 0 && *coefficient != i16::from(scan.ah))
                {
                    return Err("Invalid JPEG successive approximation".into());
                }
                *coefficient = i16::from(scan.al);
            }
        }
    }
    if transmitted[..components.len()].iter().any(|c| c[0] < 0) {
        return Err("JPEG scan script omits a component".into());
    }
    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum Token {
    Symbol(usize, usize, u8),
    Bits(u16, u8),
    Restart(u8),
}

fn magnitude(
    value: i16,
    class: usize,
    table: usize,
    run: u8,
    emit: &mut impl FnMut(Token) -> CodecResult<()>,
) -> CodecResult<()> {
    let size = u8::try_from(16 - value.unsigned_abs().leading_zeros()).map_err(message)?;
    if (class == 0 && size > 11) || (class == 1 && size > 10) {
        return Err("JPEG coefficient magnitude exceeds 8-bit precision".into());
    }
    emit(Token::Symbol(class, table, (run << 4) | size))?;
    if size > 0 {
        let bits = if value < 0 {
            i32::from(value) - 1 + (1 << size)
        } else {
            i32::from(value)
        };
        emit(Token::Bits(u16::try_from(bits).map_err(message)?, size))?;
    }
    Ok(())
}

fn encode_block(
    block: &[i16; 64],
    scan: &ProgressiveScan,
    table: usize,
    dc: &mut i16,
    emit: &mut impl FnMut(Token) -> CodecResult<()>,
) -> CodecResult<()> {
    if scan.ss == 0 {
        let value = block[0] >> scan.al;
        if scan.ah == 0 {
            magnitude(value - *dc, 0, table, 0, emit)?;
            *dc = value;
        } else {
            emit(Token::Bits(u16::try_from(value & 1).map_err(message)?, 1))?;
        }
        if scan.se == 0 {
            return Ok(());
        }
    }
    let start = usize::from(scan.ss.max(1));
    let end = usize::from(scan.se);
    let mut run = 0u8;
    if scan.ah == 0 {
        for &k in &ZIGZAG_ORDER[start..=end] {
            let value = (block[k].abs() >> scan.al) * block[k].signum();
            if value == 0 {
                run += 1;
                continue;
            }
            while run >= 16 {
                emit(Token::Symbol(1, table, 0xf0))?;
                run -= 16;
            }
            magnitude(value, 1, table, run, emit)?;
            run = 0;
        }
        if run > 0 {
            emit(Token::Symbol(1, table, 0))?;
        }
    } else {
        // Emit one EOB per block. Correction bits follow their Huffman symbol,
        // as specified by T.81 G.1.2.3; they never precede a new coefficient's sign.
        let last_new =
            (start..=end).rfind(|&k| (block[ZIGZAG_ORDER[k]].unsigned_abs() >> scan.al) == 1);
        let mut corrections = Vec::new();
        for (k, &natural) in ZIGZAG_ORDER.iter().enumerate().take(end + 1).skip(start) {
            let value = block[natural];
            let shifted = value.unsigned_abs() >> scan.al;
            if shifted == 0 {
                run += 1;
                if run == 16 && last_new.is_some_and(|last| k < last) {
                    emit(Token::Symbol(1, table, 0xf0))?;
                    for bit in corrections.drain(..) {
                        emit(Token::Bits(bit, 1))?;
                    }
                    run = 0;
                }
            } else if shifted > 1 {
                corrections.push(shifted & 1);
            } else {
                emit(Token::Symbol(1, table, (run << 4) | 1))?;
                emit(Token::Bits(u16::from(value > 0), 1))?;
                for bit in corrections.drain(..) {
                    emit(Token::Bits(bit, 1))?;
                }
                run = 0;
            }
        }
        if run > 0 || !corrections.is_empty() {
            emit(Token::Symbol(1, table, 0))?;
            for bit in corrections {
                emit(Token::Bits(bit, 1))?;
            }
        }
    }
    Ok(())
}

/// Entropy bytes are stuffed after FF; each segment is padded with one bits.
#[derive(Debug, Default)]
struct Bits {
    bytes: Vec<u8>,
    value: u32,
    count: u8,
}
impl Bits {
    fn put(&mut self, value: u16, count: u8) {
        self.value = (self.value << count) | u32::from(value);
        self.count += count;
        while self.count >= 8 {
            self.count -= 8;
            let byte = (self.value >> self.count).to_le_bytes()[0];
            self.bytes.push(byte);
            if byte == 255 {
                self.bytes.push(0);
            }
        }
        self.value &= (1 << self.count) - 1;
    }
    fn flush(&mut self) {
        if self.count > 0 {
            let count = 8 - self.count;
            self.put((1u16 << count) - 1, count);
        }
    }
}
