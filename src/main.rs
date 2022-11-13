use std::{thread::sleep, time::Duration};

use langtons_ant::*;
use pancurses::*;

fn main() {
    let mut window = initscr();
    curs_set(0);
    window.resize(FIELD_HEIGHT as i32, FIELD_WIDTH as i32 * 2);

    let mut scene = Scene::init();

    for count in 0..20000 {
        scene.work();

        if count % 10 == 0 {
            render(&window, scene.field());
            window.mvaddstr(0, 0, format!("{}", count));
        }

        window.refresh();
        // sleep(Duration::from_millis(10));
    }

    window.getch();
    endwin();
}

fn render(window: &Window, field: &Field) {
    window.clear();

    for (y, row) in field.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let x = x * 2;
            match cell {
                Cell::Black => 0,
                Cell::White => window.mvaddstr(y as i32, x as i32, "■"),
            };
        }
    }
}
