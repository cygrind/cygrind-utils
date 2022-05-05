use flo_curves::{bezier, BezierCurveFactory, Coord2, Geo, Line};

/// Ported method for extracting colour information from height from [the official editor](https://gitlab.com/PITR_DEV/ultrakill.pattern-editor/-/blob/master/src/helpers/ColorHelper.js#L5)
pub fn evaluate_height(height: i32) -> f64 {
    let emulated_x = ((height + 10) as f64 / 30.0).clamp(0.0, f64::MAX);
    let col_curve = bezier::Curve::from_points(
        Coord2(0.0, 0.0),
        (Coord2(0.0, 255.0), Coord2(0.6, 0.0)),
        Coord2(1.0, 255.0),
    );
    let line = ColLine::from_points(Coord2(emulated_x, 0.0), Coord2(emulated_x, 255.0));

    bezier::curve_intersects_line(&col_curve, &line)[0].0 * 255.0
}

#[doc(hidden)]
pub struct ColLine(Coord2, Coord2);

impl Geo for ColLine {
    type Point = Coord2;
}

impl Line for ColLine {
    fn from_points(p1: Self::Point, p2: Self::Point) -> Self {
        ColLine(p1, p2)
    }

    fn points(&self) -> (Self::Point, Self::Point) {
        (self.0, self.1)
    }
}
