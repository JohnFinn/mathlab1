extern crate gnuplot;
extern crate statistical;

use gnuplot::{Figure, Color, Caption, LineWidth, AxesCommon, AutoOption::*};
use statistical::*;

use std::io::Read;

mod math;
mod gnuplot_extension;

use math::*;
use gnuplot_extension::*;

fn read_input(filename: &str) -> Vec<f32> {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::default();
    file.read_to_string(&mut contents).unwrap();
    contents.split(" ").filter_map(|s| s.parse().ok()).collect()
}


fn print_description(data: &[f32]){
    println!("mode:           {}", math::mode(data, 0.5));
    println!("mean:           {}", mean(data)           );
    println!("variance:       {}", variance(data, None) );
    println!("standart deviation {}", standard_deviation(data, None));
    println!("standart error: {}", standart_error(data) );
    println!("median:         {}", median(data)         );
    println!("max:            {}", max(data)            );
    println!("min:            {}", min(data)            );
    println!("quartille 1:    {}", percentile(data, 0.25));
    println!("quartille 2:    {}", percentile(data, 0.50));
    println!("quartille 3:    {}", percentile(data, 0.75));
    println!("skewness        {}", univariate::skewness(data, None, None));
    println!("kurtosis        {}", univariate::kurtosis(data, None, None));
    println!("confidence mean interval +/-{}",  standart_error(data) * 1.96);
    println!("confidence stdev interval [{} {}]",  standard_deviation(data, None) * 0.88, standard_deviation(data, None) * 1.16);
}

fn main() {
    let digits = read_input("input");
    print_description(&digits);
    let mut fg = Figure::new();
    let rng = 0..digits.len();
    fg.axes2d()
        .boxes(
            rng.clone(),
            &digits,
            &[Color("blue")],
        )
        .horizontal_fill(
            mean(&digits), standard_deviation(&digits, None)*2.0, rng.clone(),
            &[Color("#01ffffff"), Caption("standard deviation")]
        )
        .horizontal_line(
            mean(&digits), rng.clone(),
            &[Color("red"), Caption("mean")]
        )
        .horizontal_line(
            percentile(&digits, 0.25), rng.clone(),
            &[Caption("quartille 1")]
        )
        .horizontal_line(
            median(&digits), rng.clone(),
            &[Caption("median")]
        )
        .horizontal_line(
            percentile(&digits, 0.75), rng.clone(),
            &[Caption("quartille 3")]
        )
        .horizontal_fill(
            math::mode(&digits, 0.5) + 0.25, 0.5, rng.clone(),
            &[LineWidth(20.0), Color("#8000ff00"), Caption("mode interval (+/- 0.5)")]
        );
    let mut distribution = Figure::new();
    distribution.axes2d()
        .function(
            |x| probability_less_than(x, &digits),
            min(&digits)..max(&digits), 0.1,
            &[]
        );
    let interval = 1.0/2.0;
    let intervals = intervals(&digits, interval);
    let mut intervals_figure = Figure::new();
    intervals_figure.axes2d()
        .boxes(
            intervals.iter().enumerate().map(|(i, _)| interval / 2.0 + i as f32 * interval + min(&digits)),
            intervals.iter().map(|&x| x as f32 / digits.len() as f32),
            &[]
        );
    let mut boxes_whiskers = Figure::new();
    boxes_whiskers.axes2d()
        .set_x_range(Fix(-1.0), Fix(1.0))
        .box_and_whisker_set_width(
            &[0],
            &[percentile(&digits, 0.25)],
            &[percentile(&digits, 0.05)],
            &[percentile(&digits, 0.95)],
            &[percentile(&digits, 0.75)],
            &[1],
            &[]
        );
    fg.show();
    distribution.show();
    intervals_figure.show();
    boxes_whiskers.show();
}
