extern crate gnuplot;
extern crate statistical;

use gnuplot::{Figure, Color, Caption, DataType, PlotOption};
use statistical::*;

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

fn print_description(data: &Vec<f32>){
    println!("mode:           {}", math::mode(data, 0.5));
    println!("mean:           {}", mean(data)           );
    println!("variance:       {}", variance(data, None) );
    println!("standart error: {}", standart_error(data) );
    println!("median:         {}", median(data)         );
    println!("max:            {}", max(data)            );
    println!("min:            {}", min(data)            );
}

fn main() {
    let digits = read_input("input");
    print_description(&digits);
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
            mean(&digits) - variance(&digits, None), (0, digits.len()),
            &[Caption("variance")]
        )
        .horizontal_line(
            mean(&digits) + variance(&digits, None), (0, digits.len()),
            &[]
        )
        .horizontal_line(
            median(&digits), (0, digits.len()),
            &[]
        );
    let mut distribution = Figure::new();
    distribution.axes2d()
        .function(
            |x| probability_less_than(x, &digits),
            (min(&digits), max(&digits)), 0.1,
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
    fg.show();
    distribution.show();
    intervals_figure.show();
}
