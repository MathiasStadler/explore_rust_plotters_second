// FROM HERE
// https://plotters-rs.github.io/book/basic/basic_data_plotting.html

use plotters::prelude::*;

fn is_prime(n: i32) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let root_area = BitMapBackend::new("images/2.13.png", (600, 400))
    .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Prime Distribution", ("sans-serif", 40))
        .build_cartesian_2d([true, false].into_segmented(), 0..50)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let prim:Vec<_> = (2..50).map(is_prime).collect();

    ctx.draw_series(
        Histogram::vertical(&ctx)
        .margin(100)
        .data(prim.iter().map(|x| (x, 1)))
    ).unwrap();
}