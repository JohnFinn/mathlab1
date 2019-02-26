extern crate gnuplot;

use gnuplot::{Figure, Color, Caption, DataType, PlotOption};

use std::io::Read;
use std::ops::{Range};

mod math;
use math::*;

fn read_input(filename: &str) -> Vec<f32> {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::default();
    file.read_to_string(&mut contents).unwrap();
    contents.split(" ").filter_map(|s| s.parse().ok()).collect()
}

trait Axes2DExtension{
	fn horizontal_line<'l, Tx: DataType, Ty: DataType>(
		&'l mut self, y: Ty, interval: (Tx, Tx), options: &[PlotOption<&str>],
	) -> &'l mut Self;

    fn function<'l, F:FnMut(f32) -> f32>(
        &'l mut self,
        func: F,
        interval: (f32, f32),
        step: f32,
        options: &[PlotOption<&str>]
    ) -> &'l mut Self;
}

impl Axes2DExtension for gnuplot::Axes2D {
	fn horizontal_line<'l, Tx: DataType, Ty: DataType>(
		&'l mut self, y: Ty, interval: (Tx, Tx), options: &[PlotOption<&str>],
	) -> &'l mut Self {
        self.lines(&[interval.0.get(), interval.1.get()], &[y.get(), y.get()], options)
    }

    fn function<'l, F:FnMut(f32) -> f32>(
        &'l mut self,
        func: F,
        interval: (f32, f32),
        step: f32,
        options: &[PlotOption<&str>]
    ) -> &'l mut Self {
        let end = interval.1/step;
        let rng = interval.0 as i32 .. end as i32;
        let x = rng.map(|x| x as f32 * step);
        let y = x.clone().map(func);
        self.lines( x, y, options )
    }
}

fn main() {
    let digits = read_input("input");
    let mean_value = mean(&digits);
    println!("mean: {}", mean_value);
    println!("variance: {}", variance(&digits));
    println!("standart error: {}", standart_error(&digits));
    let mut fg = Figure::new();
    fg.axes2d()
        .boxes(
            Range{start: 0, end: digits.len()},
            &digits,
            &[Color("blue")],
        )
        .horizontal_line(
            mean(&digits), (0, digits.len()),
            &[Color("red"), Caption("mean")]
        )
        .horizontal_line(
            mean(&digits) - variance(&digits), (0, digits.len()),
            &[Caption("variance")]
        )
        .horizontal_line(
            mean(&digits) + variance(&digits), (0, digits.len()),
            &[]
        );
    let mut distribution = Figure::new();
    distribution.axes2d()
        .function(
            |x| probability_less_than(x, &digits),
            (12.0, 23.0), 0.1,
            &[]
        );
    fg.show();
    distribution.show();
}
