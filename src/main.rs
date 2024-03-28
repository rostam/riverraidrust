use rand:: thread_rng;
use std::io::stdout;
use std::{thread, time};
use piston_window::{PistonWindow, WindowSettings};
use piston_window::*;

mod physics;
mod world;
mod events;

use world::world::{*};
use physics::physics::{*};
use events::events::{*};


fn physics(world: &mut World) {
    let mut rng = thread_rng();

    // check if player hit the ground
    check_player_status(world);

    // check enemy hit something
    check_enemy_status(world);
    check_fuel_status(world);

    // move the map Downward
    update_map(&mut rng, world);

    // create new enemy
    create_enemy(&mut rng, world);
    create_fuel(&mut rng, world);

    // Move elements along map movements
    move_enemies(world);
    move_fuel(world);
    move_bullets(world);

    if world.gas >= 1 {
        world.gas -= 1;
    }
}


fn main() -> std::io::Result<()> {
    let maxc = 1000;
    let maxr = 800;
    let slowness = 100;
    let mut world = World::new(maxc, maxr);

    // show welcoming banner
    // welcome_screen(&sc, &world);

    let mut window: PistonWindow =
        WindowSettings::new("RiverRaid Example", [1000, 800])
            .exit_on_esc(true).build().unwrap();


    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            handle_pressed_keys(&mut world, key)
            // player.update(key);
        }

        // Add this line inside the game loop to continuously update the player's position
        if event.update_args().is_some() {
            // player.auto_move();
        }
        physics(&mut world);
        window.draw_2d(&event, |c, g, _| {
            clear([0.5, 0.5, 1.0, 1.0], g); // Clear the screen with a blue-ish color
            world.draw(c,g)
            // player.render(c, g);
        });

        // thread::sleep(time::Duration::from_millis(slowness));
    }


    // while world.status == PlayerStatus::Alive || world.status == PlayerStatus::Paused {
    //     handle_pressed_keys(&mut world);
    //     if world.status != PlayerStatus::Paused {
    //         physics(&mut world);
    //         world.draw(&sc)?;
    //     } else {
    //         pause_screen(&sc, &world);
    //     }
    //     thread::sleep(time::Duration::from_millis(slowness));
    // }
    //
    

    // game is finished
    // sc.queue(Clear(crossterm::terminal::ClearType::All))?;
    // goodbye_screen(&sc, &world);
    // sc.queue(Clear(crossterm::terminal::ClearType::All))?
    //     .execute(Show)?;
    // disable_raw_mode()?;
    Ok(())
}