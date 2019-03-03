use std::ops::Range;
use gnuplot::{DataType, PlotOption};

pub trait Axes2DExtension{
    fn horizontal_line<'l, Tx: DataType, Ty: DataType>(
        &'l mut self, y: Ty, interval: Range<Tx>, options: &[PlotOption<&str>],
    ) -> &'l mut Self;

    fn function<'l, F:FnMut(f32) -> f32>(
        &'l mut self,
        func: F,
        interval: Range<f32>,
        step: f32,
        options: &[PlotOption<&str>]
    ) -> &'l mut Self;

    fn horizontal_fill<Tx: DataType>(&mut self, center: f32, hight: f32, interval: Range<Tx>,
                                     options: &[PlotOption<&str>]) -> &mut Self;
}

impl Axes2DExtension for gnuplot::Axes2D {
    fn horizontal_line<'l, Tx: DataType, Ty: DataType>(
        &'l mut self, y: Ty, interval: Range<Tx>, options: &[PlotOption<&str>],
    ) -> &'l mut Self {
        self.lines(&[interval.start.get(), interval.end.get()], &[y.get(), y.get()], options)
    }

    fn function<'l, F:FnMut(f32) -> f32>(
        &'l mut self,
        func: F,
        interval: Range<f32>,
        step: f32,
        options: &[PlotOption<&str>]
    ) -> &'l mut Self {
        let rng = interval.start as i32 .. (interval.end/step) as i32;
        let x = rng.map(|x| x as f32 * step);
        let y = x.clone().map(func);
        self.lines( x, y, options )
    }

    fn horizontal_fill<Tx: DataType>(&mut self, center: f32, hight: f32, interval: Range<Tx>,
                                     options: &[PlotOption<&str>]) -> &mut Self {
        self.fill_between(
            &[interval.start.get(), interval.end.get()],
            &[center - hight/2.0, center - hight/2.0],
            &[center + hight/2.0, center + hight/2.0],
            options
        )
    }
}
