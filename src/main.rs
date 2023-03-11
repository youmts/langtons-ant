mod error;

use std::collections::HashMap;
use std::process::ExitCode;

use error::Error;
use langtons_ant::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, TextureValueError};
use sdl2::ttf::FontError;
use sdl2::video::Window;

pub const FIELD_WIDTH: u32 = 200;
pub const FIELD_HEIGHT: u32 = 200;

pub const BEHAVIOR_NUMBER: u8 = 0;
pub const ANTS_COUNT: u8 = 1;

pub const SKIP_RENDER_FRAME: u32 = 10;
pub const CANVAS_SCALE: u32 = 4;
pub const ANTS_THICKNESS: u32 = 2;

pub fn main() -> ExitCode {
    match work() {
        Err(e) => {
            println!("{}", e);
            ExitCode::FAILURE
        }
        Ok(_) => ExitCode::SUCCESS,
    }
}

fn work() -> Result<(), Error> {
    let sdl_context = sdl2::init().map_err(Error::SDL2Init)?;
    let video_subsystem = sdl_context.video().map_err(Error::SDL2VideoInit)?;

    let ttf_context = sdl2::ttf::init().map_err(Error::SDL2TTFInit)?;
    let font = ttf_context
        .load_font("OpenSans-Regular.ttf", 24)
        .map_err(Error::LoadFont)?;

    let window = video_subsystem
        .window(
            "rust-sdl2 demo",
            FIELD_WIDTH * CANVAS_SCALE,
            FIELD_HEIGHT * CANVAS_SCALE,
        )
        .position_centered()
        .build()
        .map_err(Error::WindowBuild)?;

    let mut canvas = window.into_canvas().build().map_err(Error::IntegerOrSDL2)?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas
        .set_logical_size(FIELD_WIDTH, FIELD_HEIGHT)
        .map_err(Error::IntegerOrSDL2)?;
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().map_err(Error::SDL2String)?;

    let mut scene = Scene::init(FIELD_WIDTH, FIELD_HEIGHT, BEHAVIOR_NUMBER, ANTS_COUNT);

    'running: loop {
        scene.work();

        if scene.loop_count() % SKIP_RENDER_FRAME == 0 {
            clear(&mut canvas);
            render_field(&mut canvas, scene.field(), scene.indexed_conditions());

            render_ants(&mut canvas, scene.ants());
            render_information(&mut canvas, &font, scene.loop_count()).map_err(Error::from)?;
            canvas.present();
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

enum RenderInformationError {
    Font(FontError),
    TextureValue(TextureValueError),
    Copy(String),
}

impl From<RenderInformationError> for Error {
    fn from(val: RenderInformationError) -> Self {
        match val {
            RenderInformationError::Font(e) => Error::Font(e),
            RenderInformationError::TextureValue(e) => Error::TextureValue(e),
            RenderInformationError::Copy(s) => {
                Error::Rendering("render_information copy: ".to_owned() + s.as_str())
            }
        }
    }
}

fn render_information(
    canvas: &mut Canvas<Window>,
    font: &sdl2::ttf::Font,
    loop_count: u32,
) -> Result<(), RenderInformationError> {
    let text = format!("{}", loop_count);
    let white = Color::RGB(255, 255, 255);
    let surface = font
        .render(&text)
        .solid(white)
        .map_err(RenderInformationError::Font)?;
    let texture_creator = canvas.texture_creator();
    let texture = surface
        .as_texture(&texture_creator)
        .map_err(RenderInformationError::TextureValue)?;
    let scale = 2;

    let rect = Rect::new(0, 0, surface.width() / scale, surface.height() / scale);

    canvas
        .copy(&texture, None, rect)
        .map_err(RenderInformationError::Copy)
}

fn clear(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
}

fn render_field(canvas: &mut Canvas<Window>, field: &Field, indexed_states: &[State]) {
    let mut map: HashMap<usize, Vec<Point>> = HashMap::new();

    for (y, row) in field.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            map.entry(*cell)
                .or_insert_with(std::vec::Vec::new)
                .push(Point::new(x as i32, y as i32));
        }
    }

    for (state_index, points) in map {
        let state = &indexed_states[state_index];
        let color = convert_color(state.color());
        canvas.set_draw_color(color);
        canvas.draw_points(&points[..]).unwrap();
    }
}

fn convert_color(color: &langtons_ant::Color) -> Color {
    Color::RGB(color.r, color.g, color.b)
}

fn render_ants(canvas: &mut Canvas<Window>, ants: &[Ant]) {
    for (i, ant) in ants.iter().enumerate() {
        let (x, y) = ant.position();
        let color = find_ants_color(i as u8);
        render_ant(canvas, x, y, color);
    }
}

fn find_ants_color(number: u8) -> Color {
    match number {
        0 => Color::RGB(255, 0, 0),
        1 => Color::RGB(0, 255, 0),
        2 => Color::RGB(0, 0, 255),
        _default => panic!(),
    }
}

fn render_ant(canvas: &mut Canvas<Window>, x: i32, y: i32, color: Color) {
    let thickness: i32 = 1;
    let rect = Rect::new(
        x - thickness,
        y - thickness,
        thickness as u32 * 2,
        thickness as u32 * 2,
    );
    canvas.set_draw_color(color);
    canvas.fill_rect(rect).unwrap();
}
