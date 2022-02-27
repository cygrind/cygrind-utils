use skia_safe::{
    Color, Data, EncodedImageFormat, Font, Paint, PaintStyle, Path, Surface, TextBlob, Typeface,
};
use std::mem;

pub struct Canvas {
    surface: Surface,
    path: Path,
    paint: Paint,
}

impl Canvas {
    pub fn new(width: i32, height: i32) -> Canvas {
        let mut surface = Surface::new_raster_n32_premul((width, height)).expect("no surface!");
        let path = Path::new();
        let mut paint = Paint::default();
        paint.set_color(Color::BLACK);
        paint.set_anti_alias(true);
        paint.set_stroke_width(1.0);
        surface.canvas().clear(Color::WHITE);
        Canvas {
            surface,
            path,
            paint,
        }
    }

    #[inline]
    pub fn height(&self) -> i32 {
        self.surface.height()
    }

    #[inline]
    pub fn width(&self) -> i32 {
        self.surface.width()
    }

    #[inline]
    pub fn save(&mut self) {
        self.canvas().save();
    }

    #[inline]
    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.canvas().translate((dx, dy));
    }

    #[inline]
    pub fn scale(&mut self, sx: f32, sy: f32) {
        self.canvas().scale((sx, sy));
    }

    #[inline]
    pub fn move_to(&mut self, x: f32, y: f32) {
        self.begin_path();
        self.path.move_to((x, y));
    }

    #[inline]
    pub fn line_to(&mut self, x: f32, y: f32) {
        self.path.line_to((x, y));
    }

    #[inline]
    pub fn quad_to(&mut self, cpx: f32, cpy: f32, x: f32, y: f32) {
        self.path.quad_to((cpx, cpy), (x, y));
    }

    #[allow(dead_code)]
    #[inline]
    pub fn bezier_curve_to(&mut self, cp1x: f32, cp1y: f32, cp2x: f32, cp2y: f32, x: f32, y: f32) {
        self.path.cubic_to((cp1x, cp1y), (cp2x, cp2y), (x, y));
    }

    #[allow(dead_code)]
    #[inline]
    pub fn close_path(&mut self) {
        self.path.close();
    }

    #[inline]
    pub fn begin_path(&mut self) {
        let new_path = Path::new();
        self.surface.canvas().draw_path(&self.path, &self.paint);
        let _ = mem::replace(&mut self.path, new_path);
    }

    #[inline]
    pub fn stroke(&mut self) {
        self.paint.set_style(PaintStyle::Stroke);
        self.surface.canvas().draw_path(&self.path, &self.paint);
    }

    #[inline]
    pub fn fill(&mut self) {
        self.paint.set_style(PaintStyle::Fill);
        self.surface.canvas().draw_path(&self.path, &self.paint);
    }

    #[inline]
    pub fn set_line_width(&mut self, width: f32) {
        self.paint.set_stroke_width(width);
    }

    #[inline]
    pub fn data(&mut self) -> Data {
        let image = self.surface.image_snapshot();
        image.encode_to_data(EncodedImageFormat::PNG).unwrap()
    }

    #[inline]
    fn canvas(&mut self) -> &mut skia_safe::Canvas {
        self.surface.canvas()
    }

    #[inline]
    pub fn colour(&mut self, col: f64) {
        let col = col as u8;
        self.paint
            .set_color(u32::from_le_bytes([col, col, col, 255]));
    }

    #[inline]
    pub fn fill_text(&mut self, text: &str, pos: (f32, f32)) {
        let paint = self.paint.clone();
        let blob = TextBlob::new(
            text,
            &Font::from_typeface_with_params(Typeface::default(), 64.0, 1.5, 0.0),
        )
        .unwrap();
        self.canvas().draw_text_blob(blob, pos, &paint);
    }
}
