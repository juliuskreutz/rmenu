pub struct Draw {
    context: cairo::Context,
    layout: pangocairo::pango::Layout,
    text_middle: f64,
}

impl Draw {
    pub fn new(context: cairo::Context, layout: pangocairo::pango::Layout) -> Self {
        let text_middle = (crate::config::HEIGHT
            - (layout.size().1 / pangocairo::pango::SCALE) as u16) as f64
            / 2.;

        Self {
            context,
            layout,
            text_middle,
        }
    }

    pub fn text_size(&self, text: &str) -> (i32, i32) {
        self.layout.set_text(text);
        let (text_width, text_height) = self.layout.size();

        (
            text_width / pangocairo::pango::SCALE,
            text_height / pangocairo::pango::SCALE,
        )
    }

    pub fn draw_text(&self, x: u16, text: &str, color: (f64, f64, f64)) {
        self.layout.set_text(text);
        self.context
            .move_to((x + crate::config::MARGIN) as f64, self.text_middle);
        self.set_color(color);
        pangocairo::show_layout(&self.context, &self.layout);
    }

    pub fn draw_rectangle(&self, x: u16, width: u16, color: (f64, f64, f64)) {
        self.context
            .rectangle(x as f64, 0., width as f64, crate::config::HEIGHT as f64);
        self.set_color(color);
        self.context.fill().unwrap();
    }

    fn set_color(&self, color: (f64, f64, f64)) {
        self.context.set_source_rgb(color.0, color.1, color.2);
    }
}
