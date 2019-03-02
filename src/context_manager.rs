use cairo::{Context, Format, ImageSurface};
use std::f64::consts::PI;
use std::fs::File;

#[derive(Debug)]
pub struct ContextManager {
    context: Context,
    surface: ImageSurface,
    filename: String,
    file: Option<File>,
    width: i32,
    height: i32,
}

impl ContextManager {
    pub fn init(width: i32, height: i32) -> ContextManager {
        let surface = ImageSurface::create(Format::ARgb32, width, height)
            .expect("Couldn’t create a surface!");
        let context = Context::new(&surface);

        ContextManager {
            context: context,
            surface: surface,
            filename: "output.png".to_string(),
            file: None,
            width: width,
            height: height,
        }
    }

    pub fn clip(&mut self) -> &mut Self {
        self.context.clip();

        self
    }

    pub fn fill(&mut self) -> &mut Self {
        self.context.fill();

        self
    }

    pub fn paint(&mut self) -> &mut Self {
        self.context.paint();

        self
    }

    pub fn stroke(&mut self) -> &mut Self {
        self.context.stroke();

        self
    }

    pub fn set_line_width(&mut self, width: f32) -> &mut Self {
        self.context.set_line_width(width as f64);

        self
    }

    pub fn circle(&mut self, x: f32, y: f32, r: f32) -> &mut Self {
        self.context
            .arc(x as f64, y as f64, r as f64, 0.0, PI * 2.0);

        self
    }

    pub fn move_to(&mut self, x: f32, y: f32) -> &mut Self {
        self.context.move_to(x as f64, y as f64);

        self
    }

    pub fn line_to(&mut self, x: f32, y: f32) -> &mut Self {
        self.context.line_to(x as f64, y as f64);

        self
    }

    pub fn set_source_rgb(&mut self, r: f32, g: f32, b: f32) -> &mut Self {
        self.context.set_source_rgb(r as f64, g as f64, b as f64);

        self
    }

    pub fn set_source_rgba(&mut self, r: f32, g: f32, b: f32, a: f32) -> &mut Self {
        self.context
            .set_source_rgba(r as f64, g as f64, b as f64, a as f64);

        self
    }

    pub fn set_filename(&mut self, filename: String) -> &mut Self {
        self.filename = filename;

        self
    }

    pub fn save(&mut self) -> &mut Self {
        let mut file = File::create(&self.filename).expect("Couldn'n create output file!");

        self.surface
            .write_to_png(&mut file)
            .expect("Couldnt not write to png");

        self
    }
}
