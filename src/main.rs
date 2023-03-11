use std::collections::HashMap;
use std::process::ExitCode;

use langtons_ant::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, TextureValueError};
use sdl2::video::Window;
use sdl2::{ttf::FontError, ttf::InitError, video::WindowBuildError, IntegerOrSdlError};

pub const FIELD_WIDTH: u32 = 200;
pub const FIELD_HEIGHT: u32 = 200;

pub const BEHAVIOR_NUMBER: u8 = 0;
pub const ANTS_COUNT: u8 = 1;

pub const SKIP_RENDER_FRAME: u32 = 10;
pub const CANVAS_SCALE: u32 = 4;
pub const ANTS_THICKNESS: u32 = 2;

use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("sdl2 initialization error: {0}")]
    SDL2Init(String),

    #[error("sdl2 video initialization error: {0}")]
    SDL2VideoInit(String),

    #[error("sdl2 ttf initialization error: {0}")]
    SDL2TTFInit(InitError),

    #[error("load font error: {0}")]
    LoadFont(String),

    #[error("window build error: {0}")]
    WindowBuild(WindowBuildError),

    #[error("a sdl2 error(IntegerOrSdlError): {0}")]
    IntegerOrSDL2(IntegerOrSdlError),

    #[error("a sdl2 error(String): {0}")]
    SDL2String(String),

    #[error("font error: {0}")]
    Font(FontError),

    #[error("texture value error: {0}")]
    TextureValue(TextureValueError),

    #[error("rendering error: {0}")]
    Rendering(String),

    #[error("ants number: {0} is incorrect")]
    NoAntsNumber(u8),

    #[error("behavior number: {0} is incorrect")]
    NoBehaviorNumber(u8),

    #[error("color number: {0} is incorrect")]
    NoColorNumber(u8),
}

pub fn main() -> ExitCode {
    match work() {
        Err(e) => {
            println!("{}", e);
            ExitCode::FAILURE
        }
        Ok(_) => ExitCode::SUCCESS,
    }
}

impl From<LibError> for Error {
    fn from(value: LibError) -> Self {
        match value {
            LibError::NoAntsNumber(n) => Self::NoAntsNumber(n),
            LibError::NoBehaviorNumber(n) => Self::NoBehaviorNumber(n),
        }
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

    let mut scene =
        Scene::init(FIELD_WIDTH, FIELD_HEIGHT, BEHAVIOR_NUMBER, ANTS_COUNT).map_err(Error::from)?;

    'running: loop {
        scene.work();

        if scene.loop_count() % SKIP_RENDER_FRAME == 0 {
            clear(&mut canvas);
            render_field(&mut canvas, scene.field(), scene.indexed_conditions())?;

            render_ants(&mut canvas, scene.ants()).map_err(Error::from)?;
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

enum RenderError {
    Font(FontError),
    TextureValue(TextureValueError),
    Copy(String),
    DrawPoints(String),
    FillRect(String),
}

impl From<RenderError> for Error {
    fn from(val: RenderError) -> Self {
        match val {
            RenderError::Font(e) => Error::Font(e),
            RenderError::TextureValue(e) => Error::TextureValue(e),
            RenderError::Copy(s) => Error::Rendering("render copy: ".to_owned() + s.as_str()),
            RenderError::DrawPoints(s) => {
                Error::Rendering("render draw_points: ".to_owned() + s.as_str())
            }
            RenderError::FillRect(s) => {
                Error::Rendering("render fill_rect: ".to_owned() + s.as_str())
            }
        }
    }
}

fn render_information(
    canvas: &mut Canvas<Window>,
    font: &sdl2::ttf::Font,
    loop_count: u32,
) -> Result<(), RenderError> {
    let text = format!("{}", loop_count);
    let white = Color::RGB(255, 255, 255);
    let surface = font.render(&text).solid(white).map_err(RenderError::Font)?;
    let texture_creator = canvas.texture_creator();
    let texture = surface
        .as_texture(&texture_creator)
        .map_err(RenderError::TextureValue)?;
    let scale = 2;

    let rect = Rect::new(0, 0, surface.width() / scale, surface.height() / scale);

    canvas.copy(&texture, None, rect).map_err(RenderError::Copy)
}

fn clear(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
}

fn render_field(
    canvas: &mut Canvas<Window>,
    field: &Field,
    indexed_states: &[State],
) -> Result<(), RenderError> {
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
        canvas
            .draw_points(&points[..])
            .map_err(RenderError::DrawPoints)?;
    }

    Ok(())
}

fn convert_color(color: &langtons_ant::Color) -> Color {
    Color::RGB(color.r, color.g, color.b)
}

enum RenderAntsError {
    RenderAnt(RenderError),
    NoColorNumber(u8),
}

impl From<RenderAntsError> for Error {
    fn from(value: RenderAntsError) -> Self {
        match value {
            RenderAntsError::RenderAnt(e) => e.into(),
            RenderAntsError::NoColorNumber(n) => Error::NoColorNumber(n),
        }
    }
}

fn render_ants(canvas: &mut Canvas<Window>, ants: &[Ant]) -> Result<(), RenderAntsError> {
    for (i, ant) in ants.iter().enumerate() {
        let (x, y) = ant.position();
        let color = find_ants_color(i as u8).map_err(|e| RenderAntsError::NoColorNumber(e.0))?;
        render_ant(canvas, x, y, color).map_err(RenderAntsError::RenderAnt)?;
    }

    Ok(())
}

struct NoColorNumber(u8);

impl From<NoColorNumber> for Error {
    fn from(value: NoColorNumber) -> Self {
        Error::NoColorNumber(value.0)
    }
}

fn find_ants_color(number: u8) -> Result<Color, NoColorNumber> {
    match number {
        0 => Ok(Color::RGB(255, 0, 0)),
        1 => Ok(Color::RGB(0, 255, 0)),
        2 => Ok(Color::RGB(0, 0, 255)),
        n => Err(NoColorNumber(n)),
    }
}

fn render_ant(
    canvas: &mut Canvas<Window>,
    x: i32,
    y: i32,
    color: Color,
) -> Result<(), RenderError> {
    let thickness: i32 = 1;
    let rect = Rect::new(
        x - thickness,
        y - thickness,
        thickness as u32 * 2,
        thickness as u32 * 2,
    );
    canvas.set_draw_color(color);
    canvas.fill_rect(rect).map_err(RenderError::FillRect)?;

    Ok(())
}
