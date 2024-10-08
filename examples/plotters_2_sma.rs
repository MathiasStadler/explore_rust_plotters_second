// FROM HERE
// https://github.com/tmsdev82/rust-plotters-sma-tutorial/blob/main/src/main.rs

use chrono::prelude::*;
use chrono::Duration;
use plotters::prelude::*;

fn main() {
    let dir = "plots-output";
    // plotters-doc-data
    // let dir = "plots-doc-data";
    let filepath = format!("{}/sma15.png", &dir);
    let root = BitMapBackend::new(&filepath, (1280, 960)).into_drawing_area();
    root.fill(&WHITE).expect("Error filling background.");

    let data = get_fake_data();
    // Convert timestamp to Date<Local>
    let data: Vec<(Date<Local>, f32, f32, f32, f32)> = data
        .iter()
        .map(|x| (timestamp_to_local_date(x.0), x.1, x.2, x.3, x.4))
        .collect();

    // Get date range
    let (end_date, start_date) = (
        data[0].0 + Duration::days(1),
        data[data.len() - 1].0 - Duration::days(1),
    );

    // Basc chart configuration
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .caption(
            "Candles + simple moving average",
            ("sans-serif", 50.0).into_font(),
        )
        .build_cartesian_2d(start_date..end_date, 0f32..300f32)
        .unwrap();

    chart
        .configure_mesh()
        .light_line_style(&WHITE)
        .draw()
        .unwrap();

    chart
        .draw_series(data.iter().map(|x| {
            CandleStick::new(
                x.0,
                x.1,
                x.2,
                x.3,
                x.4,
                RGBColor(98, 209, 61).filled(),
                RGBColor(209, 61, 61).filled(),
                10,
            )
        }))
        .unwrap();

    let close_price_data: Vec<f64> = data.iter().map(|x| x.4 as f64).collect();
    let sma_data = simple_moving_average(&close_price_data, 15).expect("Calculating SMA failed");

    let mut line_data: Vec<(Date<Local>, f32)> = Vec::new();
    for i in 0..sma_data.len() {
        line_data.push((data[i].0, sma_data[i] as f32));
    }

    chart
        .draw_series(LineSeries::new(line_data, BLUE.stroke_width(2)))
        .unwrap()
        .label("SMA 15")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    let sma_data = simple_moving_average(&close_price_data, 30).expect("Calculating SMA failed");

    let mut line_data: Vec<(Date<Local>, f32)> = Vec::new();
    for i in 0..sma_data.len() {
        line_data.push((data[i].0, sma_data[i] as f32));
    }

    chart
        .draw_series(LineSeries::new(
            line_data,
            RGBColor(150, 50, 168).stroke_width(2),
        ))
        .unwrap()
        .label("SMA 30")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(150, 50, 168)));

    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperMiddle)
        .label_font(("sans-serif", 30.0).into_font())
        .background_style(WHITE.filled())
        .draw()
        .unwrap();

    root.present().expect(&format!("Unable to write result to file please make sure directory '{}' exists under the current dir", &dir));

    println!("Plot has been saved to {}", &filepath);
}

pub fn simple_moving_average(data_set: &Vec<f64>, window_size: usize) -> Option<Vec<f64>> {
    if window_size > data_set.len() {
        return None;
    }

    let mut result: Vec<f64> = Vec::new();
    let mut window_start = 0;
    while window_start + window_size <= data_set.len() {
        let window_end = window_start + window_size;
        let data_slice = &data_set[window_start..window_end];
        let sum: f64 = data_slice.iter().sum();
        let average = sum / window_size as f64;
        result.push(average);
        window_start += 1;
    }

    Some(result)
}

pub fn timestamp_to_local_date(timestamp_milis: i64) -> Date<Local> {
    let naive = NaiveDateTime::from_timestamp(timestamp_milis / 1000, 0);
    Local.from_utc_datetime(&naive).date()
}

fn get_fake_data() -> Vec<(i64, f32, f32, f32, f32)> {
    vec![
        (1640995200000, 170.01, 179.2, 169.93, 177.2),
        (1640908800000, 172.53, 177.65, 167.67, 169.99),
        (1640822400000, 170.65, 175.74, 168.17, 172.52),
        (1640736000000, 177.28, 180.62, 170.01, 170.63),
        (1640649600000, 195.68, 195.68, 176.2, 177.32),
        (1640563200000, 197.85, 204.75, 194.71, 195.67),
        (1640476800000, 193.03, 200.59, 190.22, 197.81),
        (1640390400000, 190.37, 195.0, 189.08, 193.1),
        (1640304000000, 190.08, 197.55, 184.14, 190.36),
        (1640217600000, 178.38, 193.5, 175.64, 190.03),
        (1640131200000, 179.62, 187.55, 177.05, 178.4),
        (1640044800000, 173.9, 182.54, 170.27, 179.63),
        (1639958400000, 179.9, 181.18, 168.49, 173.9),
        (1639872000000, 182.78, 189.43, 177.99, 179.93),
        (1639785600000, 175.48, 184.24, 170.96, 182.78),
        (1639699200000, 176.89, 182.5, 167.88, 175.51),
        (1639612800000, 178.53, 189.0, 174.37, 176.86),
        (1639526400000, 161.25, 182.64, 156.81, 178.52),
        (1639440000000, 155.2, 163.35, 149.6, 161.26),
        (1639353600000, 172.83, 172.83, 148.04, 155.21),
        (1639267200000, 172.15, 176.85, 167.64, 172.8),
        (1639180800000, 167.39, 173.8, 161.33, 172.15),
        (1639094400000, 181.08, 184.6, 167.0, 167.39),
        (1639008000000, 194.61, 196.6, 178.64, 181.11),
        (1638921600000, 190.24, 196.27, 183.5, 194.61),
        (1638835200000, 193.57, 204.1, 187.26, 190.19),
        (1638748800000, 196.49, 197.71, 176.17, 193.56),
        (1638662400000, 200.68, 204.51, 181.44, 196.43),
        (1638576000000, 211.65, 212.62, 169.0, 200.67),
        (1638489600000, 233.59, 239.37, 204.68, 211.65),
        (1638403200000, 229.81, 243.12, 220.0, 233.6),
        (1638316800000, 208.52, 232.63, 207.89, 229.81),
        (1638230400000, 204.1, 218.28, 199.55, 208.53),
        (1638144000000, 200.71, 212.75, 199.63, 204.09),
        (1638057600000, 192.74, 202.1, 180.93, 200.69),
        (1637971200000, 192.37, 199.39, 190.61, 192.74),
        (1637884800000, 209.94, 210.53, 182.7, 192.42),
        (1637798400000, 205.76, 216.35, 201.93, 209.97),
        (1637712000000, 221.76, 222.47, 200.14, 205.7),
        (1637625600000, 215.58, 226.08, 210.91, 221.75),
        (1637539200000, 230.56, 231.99, 211.6, 215.59),
        (1637452800000, 217.64, 240.0, 210.53, 230.51),
        (1637366400000, 215.25, 221.17, 205.84, 217.63),
        (1637280000000, 195.19, 219.59, 188.99, 215.25),
        (1637193600000, 218.9, 222.78, 186.5, 195.01),
        (1637107200000, 219.09, 222.44, 209.53, 218.94),
        (1637020800000, 238.18, 238.21, 213.81, 219.04),
        (1636934400000, 238.63, 246.36, 234.49, 238.2),
        (1636848000000, 241.58, 242.0, 230.4, 238.64),
        (1636761600000, 228.46, 241.97, 224.79, 241.59),
        (1636675200000, 233.34, 239.09, 220.83, 228.48),
        (1636588800000, 233.06, 246.5, 229.4, 233.37),
        (1636502400000, 239.06, 248.0, 217.5, 233.09),
        (1636416000000, 248.27, 253.22, 237.62, 239.04),
        (1636329600000, 249.62, 253.3, 240.68, 248.26),
        (1636243200000, 258.41, 258.66, 245.65, 249.68),
        (1636156800000, 236.04, 259.9, 234.79, 258.44),
        (1636070400000, 247.06, 248.84, 230.34, 236.06),
        (1635984000000, 243.37, 250.5, 234.0, 247.07),
        (1635897600000, 220.81, 247.0, 215.69, 243.37),
        (1635811200000, 203.91, 222.22, 200.97, 220.82),
        (1635724800000, 202.53, 211.66, 197.7, 203.9),
        (1635638400000, 195.69, 205.0, 185.31, 202.49),
        (1635552000000, 200.36, 200.62, 187.3, 195.7),
        (1635465600000, 195.37, 205.22, 194.45, 200.41),
        (1635379200000, 184.45, 201.38, 181.64, 195.42),
        (1635292800000, 199.87, 205.49, 176.94, 184.49),
        (1635206400000, 209.73, 214.2, 196.36, 199.85),
        (1635120000000, 202.15, 218.93, 198.0, 209.7),
        (1635033600000, 197.75, 204.86, 185.22, 202.14),
        (1634947200000, 196.29, 205.78, 191.86, 197.75),
        (1634860800000, 190.46, 215.1, 187.96, 196.29),
        (1634774400000, 176.72, 194.21, 176.7, 190.5),
        (1634688000000, 155.77, 176.87, 155.0, 176.72),
        (1634601600000, 157.12, 159.45, 153.06, 155.78),
        (1634515200000, 160.11, 162.98, 154.91, 157.12),
        (1634428800000, 157.54, 167.65, 153.13, 160.12),
        (1634342400000, 163.15, 164.8, 156.36, 157.54),
        (1634256000000, 150.01, 165.75, 146.68, 163.18),
        (1634169600000, 148.05, 155.65, 147.27, 150.02),
        (1634083200000, 152.35, 155.56, 144.28, 148.04),
        (1633996800000, 144.79, 153.5, 137.61, 152.32),
        (1633910400000, 147.56, 154.0, 140.24, 144.77),
        (1633824000000, 156.8, 158.5, 145.55, 147.55),
        (1633737600000, 158.88, 161.54, 154.18, 156.84),
        (1633651200000, 154.26, 168.97, 152.39, 158.86),
        (1633564800000, 153.9, 161.42, 150.34, 154.26),
        (1633478400000, 164.64, 165.42, 150.1, 153.87),
        (1633392000000, 167.13, 170.18, 159.68, 164.62),
        (1633305600000, 172.99, 173.34, 162.0, 167.13),
        (1633219200000, 169.0, 177.79, 165.58, 172.98),
        (1633132800000, 161.62, 175.0, 156.06, 168.98),
        (1633046400000, 141.38, 164.95, 138.25, 161.56),
        (1632960000000, 135.24, 142.95, 134.02, 141.37),
        (1632873600000, 132.22, 140.17, 131.12, 135.25),
        (1632787200000, 136.34, 139.4, 128.17, 132.2),
        (1632700800000, 135.7, 148.92, 133.72, 136.35),
        (1632614400000, 136.05, 140.81, 124.8, 135.64),
        (1632528000000, 139.11, 144.18, 133.54, 136.06),
        (1632441600000, 149.9, 151.2, 128.15, 139.13),
    ]
}
