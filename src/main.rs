use std::collections::HashMap;
use std::process::ExitCode;

use clap::Parser;
use langtons_ant::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, TextureValueError};
use sdl2::video::Window;
use sdl2::{ttf::FontError, ttf::InitError, video::WindowBuildError, IntegerOrSdlError};

pub const ANTS_THICKNESS: u32 = 2;

use thiserror::Error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    behavior: u8,

    #[arg(short, long, default_value_t = 1)]
    ant_number: u8,

    #[arg(long, default_value_t = 200)]
    field_width: u32,

    #[arg(long, default_value_t = 200)]
    field_height: u32,

    #[arg(long, default_value_t = 4)]
    canvas_scale: u32,

    #[arg(long, default_value_t = 10)]
    skip_render_frame: u32,
}

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
    let args = Args::parse();

    match work(args) {
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

fn work(args: Args) -> Result<(), Error> {
    let sdl_context = sdl2::init().map_err(Error::SDL2Init)?;
    let video_subsystem = sdl_context.video().map_err(Error::SDL2VideoInit)?;

    let ttf_context = sdl2::ttf::init().map_err(Error::SDL2TTFInit)?;
    let font = ttf_context
        .load_font("OpenSans-Regular.ttf", 24)
        .map_err(Error::LoadFont)?;

    let window = video_subsystem
        .window(
            "rust-sdl2 demo",
            args.field_width * args.canvas_scale,
            args.field_height * args.canvas_scale,
        )
        .position_centered()
        .build()
        .map_err(Error::WindowBuild)?;

    let mut canvas = window.into_canvas().build().map_err(Error::IntegerOrSDL2)?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas
        .set_logical_size(args.field_width, args.field_height)
        .map_err(Error::IntegerOrSDL2)?;
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().map_err(Error::SDL2String)?;

    let mut scene = Scene::init(
        args.field_width,
        args.field_height,
        args.behavior,
        args.ant_number,
    )
    .map_err(Error::from)?;

    'running: loop {
        scene.work();

        if scene.loop_count() % args.skip_render_frame == 0 {
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

fn render_information(
    canvas: &mut Canvas<Window>,
    font: &sdl2::ttf::Font,
    loop_count: u32,
) -> Result<(), Error> {
    let text = format!("{}", loop_count);
    let white = Color::RGB(255, 255, 255);
    let surface = font.render(&text).solid(white).map_err(Error::Font)?;
    let texture_creator = canvas.texture_creator();
    let texture = surface
        .as_texture(&texture_creator)
        .map_err(Error::TextureValue)?;
    let scale = 2;

    let rect = Rect::new(0, 0, surface.width() / scale, surface.height() / scale);

    canvas.copy(&texture, None, rect).map_err(Error::Rendering)
}

fn clear(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
}

fn render_field(
    canvas: &mut Canvas<Window>,
    field: &Field,
    indexed_states: &[State],
) -> Result<(), Error> {
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
        canvas.draw_points(&points[..]).map_err(Error::Rendering)?;
    }

    Ok(())
}

fn convert_color(color: &langtons_ant::Color) -> Color {
    Color::RGB(color.r, color.g, color.b)
}

fn render_ants(canvas: &mut Canvas<Window>, ants: &[Ant]) -> Result<(), Error> {
    for (i, ant) in ants.iter().enumerate() {
        let (x, y) = ant.position();
        let color = find_ants_color(i as u8).map_err(|e| Error::NoColorNumber(e.0))?;
        render_ant(canvas, x, y, color)?;
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

fn render_ant(canvas: &mut Canvas<Window>, x: i32, y: i32, color: Color) -> Result<(), Error> {
    let thickness: i32 = 1;
    let rect = Rect::new(
        x - thickness,
        y - thickness,
        thickness as u32 * 2,
        thickness as u32 * 2,
    );
    canvas.set_draw_color(color);
    canvas.fill_rect(rect).map_err(Error::Rendering)?;

    Ok(())
}
