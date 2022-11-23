use langtons_ant::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // let ttl_context = sdl2::ttf::init().unwrap();
    // let font = ttl_context.load_font("Sans.ttf", 24).unwrap();

    let scale = 2;
    let window = video_subsystem
        .window(
            "rust-sdl2 demo",
            FIELD_WIDTH as u32 * scale,
            FIELD_HEIGHT as u32 * scale,
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

    let mut scene = Scene::init();

    'running: loop {
        scene.work();

        if scene.loop_count() % 100 == 0 {
            render(&mut canvas, scene.field());
            canvas.present();
        }
        // render_information(&mut canvas, &font, scene.loop_count());

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
        // The rest of the game loop goes here...

        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn render(canvas: &mut Canvas<Window>, field: &Field) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let mut points = vec![];

    // TODO: draw only diffs
    for (y, row) in field.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            match cell {
                Cell::Black => (),
                Cell::White => points.push(Point::new(x as i32, y as i32)),
            };
        }
    }

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.draw_points(&points[..]).unwrap();
}
