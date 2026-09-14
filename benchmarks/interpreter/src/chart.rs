//! Plotters produces standalone SVG and PNG charts
#![expect(
    clippy::cast_precision_loss,
    reason = "Small chart row counts fit floating-point coordinates"
)]

use crate::statistics::{number, string};
use anyhow::{Context, Result, ensure};
use plotters::coord::Shift;
use plotters::prelude::*;
use serde_json::Value;
use std::path::Path;

#[expect(clippy::too_many_lines, reason = "Chart layout is kept together")]
fn draw<B: DrawingBackend>(
    area: &DrawingArea<B, Shift>,
    rows: &[Value],
    metadata: &Value,
) -> Result<()>
where
    B::ErrorType: 'static,
{
    area.fill(&RGBColor(251, 252, 254))?;
    let title = "Ristretto vs OpenJDK: interpreter performance";
    area.draw(&Text::new(
        title,
        (80, 40),
        ("sans-serif", 32).into_font().style(FontStyle::Bold),
    ))?;
    let info = metadata
        .pointer("/runtime_info/openjdk")
        .unwrap_or(&Value::Null)
        .as_str()
        .unwrap_or("");
    let version = info
        .lines()
        .find_map(|s| s.strip_prefix("java.version="))
        .unwrap_or("unknown");
    let cpu = metadata["cpu"]
        .as_str()
        .unwrap_or("Unknown CPU")
        .lines()
        .next()
        .unwrap_or("Unknown CPU");
    area.draw(&Text::new(
        format!("{cpu} | same OpenJDK {version} library | both -Xint"),
        (80, 80),
        ("sans-serif", 20).into_font(),
    ))?;
    area.draw(&Text::new(
        format!(
            "{} processes x {} samples; bars show approximate 95% ratio intervals",
            metadata.pointer("/options/forks").unwrap_or(&Value::Null),
            metadata.pointer("/options/samples").unwrap_or(&Value::Null)
        ),
        (80, 110),
        ("sans-serif", 20).into_font(),
    ))?;
    let minimum = rows
        .iter()
        .map(|r| number(r, "ratio"))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .fold(1.0, f64::min)
        * 0.8;
    let maximum = rows
        .iter()
        .map(|r| {
            r["ratio_ci_high"]
                .as_f64()
                .unwrap_or(r["ratio"].as_f64().unwrap_or(1.0))
        })
        .fold(1.0, f64::max)
        * 1.5;
    let mut chart = ChartBuilder::on(area)
        .margin_top(140)
        .margin_bottom(85)
        .margin_right(60)
        .margin_left(15)
        .x_label_area_size(45)
        .y_label_area_size(250)
        .build_cartesian_2d(
            (minimum..maximum).log_scale(),
            -0.5..rows.len() as f64 - 0.5,
        )?;
    chart
        .configure_mesh()
        .disable_y_mesh()
        .y_labels(0)
        .x_labels(8)
        .x_label_formatter(&|x| format!("{x:.1}x"))
        .axis_desc_style(("sans-serif", 18))
        .label_style(("sans-serif", 18))
        .x_desc("Ristretto time / OpenJDK time (log scale; lower is better)")
        .draw()?;
    chart.draw_series(std::iter::once(PathElement::new(
        vec![(1.0, -0.5), (1.0, rows.len() as f64 - 0.5)],
        RGBColor(150, 160, 170),
    )))?;
    for (index, row) in rows.iter().enumerate() {
        let y = (rows.len() - 1 - index) as f64;
        let ratio = number(row, "ratio")?;
        let low = row["ratio_ci_low"].as_f64().unwrap_or(ratio);
        let high = row["ratio_ci_high"].as_f64().unwrap_or(ratio);
        chart.draw_series(std::iter::once(PathElement::new(
            vec![(low, y), (high, y)],
            RGBColor(88, 121, 151).stroke_width(3),
        )))?;
        chart.draw_series(std::iter::once(Circle::new(
            (ratio, y),
            5,
            RGBColor(23, 105, 162).filled(),
        )))?;
        chart.draw_series(std::iter::once(Text::new(
            format!("{ratio:.2}x"),
            (high * 1.04, y),
            ("sans-serif", 18).into_font(),
        )))?;
        let name = string(row, "benchmark")?;
        let label = name.replace('_', " ") + if name == "exception_fresh" { "*" } else { "" };
        let (_, pixel_y) = chart.as_coord_spec().translate(&(minimum, y));
        area.draw(&Text::new(
            label,
            (18, pixel_y),
            ("sans-serif", 18).into_font(),
        ))?;
    }
    if rows.iter().any(|r| r["benchmark"] == "exception_fresh") {
        let (_, height) = area.dim_in_pixel();
        area.draw(&Text::new(
            "* Fresh exceptions: finite-run result; inspect raw samples for within-process drift.",
            (40, i32::try_from(height)? - 20),
            ("sans-serif", 17).into_font(),
        ))?;
    }
    area.present()?;
    Ok(())
}

pub fn plot(input: &Path, output: &Path) -> Result<()> {
    let mut rows: Vec<Value> = serde_json::from_slice(&std::fs::read(input.join("summary.json"))?)?;
    let metadata: Value = serde_json::from_slice(&std::fs::read(input.join("metadata.json"))?)?;
    rows.retain(|r| r["benchmark"] != "control" && r["complete"] == true);
    ensure!(
        !rows.is_empty(),
        "No complete non-control workloads to plot"
    );
    for row in &rows {
        ensure!(number(row, "ratio")? > 0.0, "Ratios must be positive");
    }
    rows.sort_by(|a, b| {
        a["ratio"]
            .as_f64()
            .unwrap_or(0.0)
            .total_cmp(&b["ratio"].as_f64().unwrap_or(0.0))
    });
    let height = u32::try_from(rows.len())?
        .checked_mul(40)
        .and_then(|n| n.checked_add(250))
        .context("Chart too tall")?;
    std::fs::create_dir_all(output)?;
    let svg = output.join("ratios.svg");
    let png = output.join("ratios.png");
    draw(
        &SVGBackend::new(&svg, (1200, height)).into_drawing_area(),
        &rows,
        &metadata,
    )?;
    draw(
        &BitMapBackend::new(&png, (1200, height)).into_drawing_area(),
        &rows,
        &metadata,
    )?;
    println!("Charts: {}", output.display());
    Ok(())
}
