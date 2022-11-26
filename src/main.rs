use std::collections::HashMap;

use langtons_ant::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

pub const FIELD_WIDTH: u32 = 300;
pub const FIELD_HEIGHT: u32 = 300;

pub const SKIP_RENDER_FRAME: u32 = 100;
pub const CANVAS_SCALE: u32 = 4;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let ttl_context = sdl2::ttf::init().unwrap();
    let font = ttl_context.load_font("OpenSans-Regular.ttf", 24).unwrap();

    let window = video_subsystem
        .window(
            "rust-sdl2 demo",
            FIELD_WIDTH as u32 * CANVAS_SCALE,
            FIELD_HEIGHT as u32 * CANVAS_SCALE,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas
        .set_logical_size(FIELD_WIDTH as u32, FIELD_HEIGHT as u32)
        .unwrap();
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut scene = Scene::init(FIELD_WIDTH, FIELD_HEIGHT);

    'running: loop {
        scene.work();

        if scene.loop_count() % SKIP_RENDER_FRAME == 0 {
            clear(&mut canvas);
            render_field(&mut canvas, scene.field(), scene.indexed_conditions());

            let (x, y) = scene.ant_position();
            render_ant(&mut canvas, x, y);
            render_information(&mut canvas, &font, scene.loop_count());
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
}

fn render_information(canvas: &mut Canvas<Window>, font: &sdl2::ttf::Font, loop_count: u32) {
    let text = format!("{}", loop_count);
    let white = Color::RGB(255, 255, 255);
    let surface = font.render(&text).solid(white).unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = surface.as_texture(&texture_creator).unwrap();
    let scale = 2;

    let rect = Rect::new(0, 0, surface.width() / scale, surface.height() / scale);
    canvas.copy(&texture, None, rect).unwrap();
}

fn clear(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
}

fn render_field(canvas: &mut Canvas<Window>, field: &Field, indexed_states: &Vec<State>) {
    let mut map: HashMap<usize, Vec<Point>> = HashMap::new();

    for (y, row) in field.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            map.entry(*cell)
                .or_insert_with(|| vec![])
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

fn render_ant(canvas: &mut Canvas<Window>, x: i32, y: i32) {
    let thickness: i32 = 2;
    let rect = Rect::new(
        x - thickness,
        y - thickness,
        thickness as u32 * 2,
        thickness as u32 * 2,
    );
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.fill_rect(rect).unwrap();

    let thickness: i32 = 1;
    let rect = Rect::new(
        x - thickness,
        y - thickness,
        thickness as u32 * 2,
        thickness as u32 * 2,
    );
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.fill_rect(rect).unwrap();
}
