// FROM HERE
// https://plotters-rs.github.io/book/basic/draw_3d_plots.html

use plotters::prelude::*;
fn main() {
    let root = BitMapBackend::new("images/3d-1-empty_3d_figure.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("Empty 3D Figure", ("sans-serif", 40))
        .build_cartesian_3d(0.0..1.0, 0.0..1.0, 0.0..1.0)
        .unwrap();
    chart.configure_axes().draw().unwrap();
}

// cargo run --example 