#![deny(bare_trait_objects)]

use cgmath::*;
use fnv::*;
use std::panic;
use wasm_bindgen::prelude::*;
use web_sys::window;
use webgl_gui::gui::*;
use webgl_gui::widgets::*;
use webgl_gui::Color4;
use webgl_gui::*;
use webgl_wrapper::*;

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Trace).unwrap();

    let assets =
        Assets::load(vec!["DejaVuSansMono.ttf".to_owned()], vec!["mandelbrot.png".to_owned()])
            .await;

    if let Ok((context, screen_surface)) = GlContext::new(CANVAS_ID) {
        let demo = Demo::new(context, screen_surface, assets);
        start_main_loop(CANVAS_ID, Box::new(demo));
    } else {
        window().unwrap().alert_with_message("Unable to create WebGL 2 context. Try reloading the page; if that doesn't work, switch to Firefox or Chrome.").unwrap();
    }

    Ok(())
}

const CANVAS_ID: &str = "canvas";

struct Demo {
    context: GlContext,
    screen_surface: ScreenSurface,
    font: Font,
    draw_2d: Draw2d,
    pos: Point2<f32>,
    gui: Gui,
    theme: Theme,
    button: Box<Button>,
    button_presses: i32,
    image: Texture2d,
}

impl Demo {
    pub fn new(context: GlContext, screen_surface: ScreenSurface, mut assets: Assets) -> Self {
        let font = Font::new(&context, assets.remove("DejaVuSansMono.ttf").unwrap(), 16);
        let theme = Theme {
            font: font.clone(),
            label_color: Color4::BLACK,
            button_text_color: Color4::BLACK,
            button_fill_color: (Color4::from_srgba(0.0, 0.0, 0.0, 0.2).mul_srgb(0.2)),
            button_border_color: Color4::from_grayscale_srgb(0.2),
            button_selected_fill_color: (Color4::from_srgba(0.0, 0.0, 0.0, 0.6).mul_srgb(0.6)),
            button_active_fill_color: (Color4::from_srgba(0.0, 0.0, 0.0, 0.5).mul_srgb(0.5)),
            padding: 10,
        };
        let draw_2d_programs = Draw2dPrograms::new(&context);
        Self {
            image: Texture2d::from_image(
                &context,
                assets.get_image("mandelbrot.png").unwrap(),
                TextureFormat::SRGB,
                MinFilter::Linear,
                MagFilter::Linear,
                WrapMode::ClampToEdge,
            ),
            font,
            draw_2d: Draw2d::new(&context, &draw_2d_programs),
            pos: point2(150.0, 100.0),
            context,
            screen_surface,
            gui: Gui::new(),
            theme,
            button: Button::new("Click me"),
            button_presses: 0,
        }
    }

    pub fn handle_pressed_keys(&mut self, pressed_keys: &FnvHashSet<String>) {
        if pressed_keys.contains("ArrowLeft") {
            self.pos += vec2(-1.0, 0.0);
        }
        if pressed_keys.contains("ArrowRight") {
            self.pos += vec2(1.0, 0.0);
        }
        if pressed_keys.contains("ArrowUp") {
            self.pos += vec2(0.0, -1.0);
        }
        if pressed_keys.contains("ArrowDown") {
            self.pos += vec2(0.0, 1.0);
        }
    }

    pub fn handle_events(&mut self, events: Vec<Event>) {
        let mut gui_events = self.gui.handle_events(&events, &[self.button.id()]);
        if gui_events.update_component(&self.theme, &mut self.button).pressed() {
            self.button_presses += 1;
        }
    }

    pub fn draw(&mut self, cursor_pos: Option<Point2<i32>>) {
        self.screen_surface.clear(&self.context, &[ClearBuffer::Color(Color4::WHITE.into())]);

        // Draw some random geometry.
        self.draw_2d.fill_poly(
            &[point2(50.0, 50.0), point2(100.0, 50.0), point2(100.0, 100.0)],
            Color4::BLUE,
        );
        self.draw_2d.draw_line_strip(
            &[point2(50.0, 100.0), point2(100.0, 100.0), point2(50.0, 150.0)],
            Color4::RED,
            1.0,
        );

        self.draw_2d.fill_poly(
            &[
                self.pos + vec2(0.0, 5.0),
                self.pos + vec2(5.0, 0.0),
                self.pos + vec2(0.0, -5.0),
                self.pos + vec2(-5.0, 0.0),
            ],
            Color4::BLACK,
        );

        self.gui.draw(
            &self.context,
            &self.screen_surface,
            &self.theme,
            &mut self.draw_2d,
            cursor_pos.map(|x| x.cast().unwrap()),
            Col::new().child(0.0, NoFill::new(self.button.clone())).child(
                0.0,
                Label::new(&format!("The button has been pressed {} times", self.button_presses)),
            ),
        );

        self.draw_2d
            .render_queued(&self.screen_surface, compute_ortho_matrix(&self.screen_surface));
        self.font.render_queued(&self.screen_surface);
        self.draw_2d.draw_image(&self.screen_surface, &self.image, point2(300.0, 0.0), 1.0);
    }
}

impl App for Demo {
    fn render_frame(&mut self, events: Vec<Event>, event_state: &EventState, _dt: f64) {
        self.handle_pressed_keys(&event_state.pressed_keys);
        self.handle_events(events);
        self.draw(event_state.cursor_pos);
    }
}
