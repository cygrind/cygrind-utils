use super::{canvas::Canvas, parser::Pattern};
use flo_curves::bezier;
use flo_curves::*;

fn evaluate_height(height: i32) -> f64 {
    let emulated_x = (height + 10) as f64 / 30.0;
    let col_curve = bezier::Curve::from_points(
        Coord2(0.0, 0.0),
        (Coord2(0.0, 255.0), Coord2(0.6, 0.0)),
        Coord2(1.0, 255.0),
    );
    let line = ColLine::from_points(Coord2(emulated_x, 0.0), Coord2(emulated_x, 255.0));

    bezier::curve_intersects_line(&col_curve, &line)[0].0 * 255.0
}

pub fn draw(pattern: Pattern) -> Vec<u8> {
    let mut canvas = Canvas::new(1920, 1920);
    let heightconst = (canvas.height() / 16) as f32;
    let widthconst = (canvas.width() / 16) as f32;

    for i in 0..16 {
        for j in 0..16 {
            let height = pattern.0[i][j].height();

            let x1 = widthconst * i as f32;
            let y1 = heightconst * j as f32;
            let x2 = widthconst * (i + 1) as f32;
            let y2 = heightconst * (j + 1) as f32;

            canvas.colour(evaluate_height(height));

            canvas.move_to(x1, y1);
            canvas.line_to(x1, y1);
            canvas.line_to(x1, y2);
            canvas.line_to(x2, y2);
            canvas.line_to(x2, y1);
            canvas.line_to(x1, y1);

            canvas.fill();
        }
    }

    canvas.data().as_bytes().to_vec()
}

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
