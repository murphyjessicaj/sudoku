extern crate piston;
extern crate piston_window;
extern crate graphics;

use piston_window::*;

mod board;

fn initialize_the_board(window: &mut PistonWindow, board: &board::Board) {
    while let Some(e) = window.next() {
        window.draw_2d(&e,|c, g| {
            clear([1.0;4],g);
            rectangle(
                [0.0,0.0,0.0,1.0],
                [0.0,0.0,100.0,100.0],
                c.transform,
                g
            );
        });
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("SDL Window", (800, 800))
        .exit_on_esc(true)
        .fullscreen(false)
        .vsync(true)
        .build()
        .unwrap();

    window.events.set_max_fps(1);
    window.set_ups(1);
    let mut board: board::Board = board::Board::new();
    board.update();

    //graphics::text::Text
    //let mut count: i64 = 0;
    while let Some(e) = window.next() {

        println!("{:?}", e);
        //count+=1;
        if &e == &piston::input::Input::Move {
            println!("boo", );
        }


        //draw squares

        let interval: f64 = 800.0/9.0;

        for i in 0..9 {
            for j in 0..9 {
                window.draw_2d(&e,|c, g| {
                    clear([1.0;4],g);
                    rectangle(
                        [0.0,0.0,0.0,1.0],
                        [
                            (i as f64) * interval + 5.0,
                            (j as f64) * interval + 5.0,
                            ((i+1) as f64) * interval - 5.0,
                            ((j+1) as f64) * interval - 5.0
                        ],
                        c.transform,
                        g
                    );
                });
            }
        }
    }

}
