use piston_window:: {
    clear, Button, PistonWindow, PressEvent, ReleaseEvent, UpdateEvent, WindowSettings
};
use crate::{draw::*, game::Game};

const WINDOW_SIZE: [f64; 2] = [80.0,60.0];

pub fn run_window() {
    let mut window: PistonWindow = WindowSettings::new("Ping pong", [to_coordinate(WINDOW_SIZE[0]), to_coordinate(WINDOW_SIZE[1])])
        .exit_on_esc(true)
        .build()
        .unwrap();

    
    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let mut glyphs = window.load_font(assets.join("NovaMono-Regular.ttf")).unwrap();



    let mut game = Game::new(WINDOW_SIZE);

    while let Some(event) = window.next() {

        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        if let Some(Button::Keyboard(_)) = event.release_args() {
            game.key_released();
        }
        
        window.draw_2d(&event, |c, g, device| {

            clear([1.0; 4], g);

            game.draw(&c, g, &mut glyphs);

            glyphs.factory.encoder.flush(device);
            
        });


        event.update(|arg| {
            game.update(arg.dt);
        });


    }
}