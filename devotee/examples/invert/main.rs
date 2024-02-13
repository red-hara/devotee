use devotee::app;
use devotee::app::context::Context;
use devotee::app::input::key_mouse::{KeyMouse, VirtualKeyCode};
use devotee::app::root::Root;
use devotee::app::setup;
use devotee::util::vector::Vector;
use devotee::visual::canvas::Canvas;
use devotee::visual::prelude::*;
use devotee_backend_softbuffer::SoftbufferBackend;

fn main() {
    let init_config = setup::Builder::new()
        .with_render_target(Canvas::with_resolution(Color([0, 0, 0]), 128, 128))
        .with_input(KeyMouse::default())
        .with_root_constructor(|_| Invert::new())
        .with_title("invert")
        .with_scale(2);
    let app = app::App::<Invert, SoftbufferBackend>::with_setup(init_config).unwrap();

    app.run();
}

struct Invert {
    position: Vector<f64>,
    canvas: Canvas<bool>,
}

impl Invert {
    fn new() -> Self {
        let position = Vector::new(12.0, 12.0);
        let mut canvas = Canvas::with_resolution(false, 32, 32);

        let mut painter = canvas.painter();
        painter.line((0, 0), (31, 0), paint(true));
        painter.line((0, 0), (0, 31), paint(true));
        painter.line((0, 31), (31, 31), paint(true));
        painter.line((31, 0), (31, 31), paint(true));
        painter.line((0, 0), (31, 31), paint(true));
        painter.line((31, 0), (0, 31), paint(true));

        let mut counter = 0;
        painter.rect_f((4, 4), (32 - 8, 32 - 8), move |_, _, _| {
            counter += 1;
            counter % 5 == 0 || counter % 7 == 0
        });

        Self { position, canvas }
    }
}

impl Root for Invert {
    type Converter = Converter;
    type Input = KeyMouse;
    type RenderTarget = Canvas<Color>;

    fn update(&mut self, update: &mut Context<KeyMouse>) {
        if update.input().keys().just_pressed(VirtualKeyCode::Escape) {
            update.shutdown();
        }
        let delta = 16.0 * update.delta().as_secs_f64();
        if update.input().keys().is_pressed(VirtualKeyCode::Left) {
            *self.position.x_mut() -= delta;
        }
        if update.input().keys().is_pressed(VirtualKeyCode::Right) {
            *self.position.x_mut() += delta;
        }
        if update.input().keys().is_pressed(VirtualKeyCode::Up) {
            *self.position.y_mut() -= delta;
        }
        if update.input().keys().is_pressed(VirtualKeyCode::Down) {
            *self.position.y_mut() += delta;
        }
    }

    fn render(&self, render: &mut Canvas<Color>) {
        render.clear(Color([0, 0, 0]));
        let dimensions = render.dimensions();
        let mut painter = render.painter();

        painter.rect_f((8, 8).into(), dimensions - (16, 16), |x, y, _| {
            Color([2 * x as u8, 2 * y as u8, 0x00])
        });

        let (x, y) = (self.position.x() as i32, self.position.y() as i32);
        painter.image((x, y), &self.canvas, |_, _, value, _, _, invert| {
            if invert {
                Color([0xff - value.0[0], 0xff - value.0[1], 0xff - value.0[2]])
            } else {
                value
            }
        });
    }

    fn converter(&self) -> &Converter {
        &Converter
    }

    fn background_color(&self) -> Color {
        Color([0x00, 0x00, 0x00])
    }
}

#[derive(Clone, Copy, Default)]
struct Color([u8; 3]);

struct Converter;

impl devotee_backend::Converter for Converter {
    type Palette = Color;
    fn convert(&self, color: &Self::Palette) -> u32 {
        ((color.0[0] as u32) << 16) | ((color.0[1] as u32) << 8) | color.0[2] as u32
    }
}
