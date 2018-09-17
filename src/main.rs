extern crate rand;
extern crate piston_window;

mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::Color;

use game::Game;
use draw::to_coord_u32;

const BACKGROUND_COLOR : Color = [0.5,0.5,0.5,1.0];

fn main() {
    let (width,height) = (30,30);
    
    // create the game window 
    let mut window : PistonWindow = WindowSettings::new(
        "Snake",
        [to_coord_u32(width), to_coord_u32(height)],

    ).exit_on_esc(true)
        .opengl(OpenGL::V3_1)
        .srgb(false)
        .build()
        .unwrap();

    // // create a new game 
    // let mut game = Game::new(width,height);

    // // handle game , window.next() cleans the UI every event 
    // while let Some(event) = window.next(){
    //     if let Some(Button::Keyboard(key)) = event.press_args(){
    //         game.key_pressed(key);
    //     }
    //     window.draw_2d(&event,|c,g|{
    //         clear(BACKGROUND_COLOR,g);
    //         game.draw(&c, g);
    //     });

    //     event.update(|arg|{
    //         game.update(arg.dt);
    //     });
    // }
}
