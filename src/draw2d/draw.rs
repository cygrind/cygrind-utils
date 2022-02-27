use super::canvas::Canvas;
use crate::parser::Pattern;
use crate::util::evaluate_height;

pub struct Draw2d;

impl Draw2d {
    pub fn draw(pattern: Pattern) -> Vec<u8> {
        let mut canvas = Canvas::new(1920, 1920);
        let heightconst = (canvas.height() / 16) as f32;
        let widthconst = (canvas.width() / 16) as f32;

        for i in 0..16 {
            for j in 0..16 {
                let height = pattern.0[j][i].height();

                let x1 = widthconst * i as f32;
                let y1 = heightconst * j as f32;
                let x2 = widthconst * (i + 1) as f32;
                let y2 = heightconst * (j + 1) as f32;
            
                canvas.move_to(x1, y1);
                canvas.line_to(x1, y1);
                canvas.line_to(x1, y2);
                canvas.line_to(x2, y2);
                canvas.line_to(x2, y1);
                canvas.line_to(x1, y1);

                canvas.colour(evaluate_height(height));
                canvas.fill();
            }
        }
        canvas.data().as_bytes().to_vec()
    }
}
// cargo test --package cygrind-utils --lib -- parser::test::parser_draw2d --exact --nocapture
