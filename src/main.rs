extern crate piston_window;
extern crate find_folder;

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
            format!("Mirage v{}.{}.{}",
                    env!("CARGO_PKG_VERSION_MAJOR"),
                    env!("CARGO_PKG_VERSION_MINOR"),
                    env!("CARGO_PKG_VERSION_PATCH")),
            [1280, 720]
        )
        .exit_on_esc(true)
        //.opengl(OpenGL::V2_1) // Set a different OpenGl version
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    println!("{:?}", assets);
    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

    window.set_lazy(true);
    let mut titlex = 0.0;
    let mut titley = 0.0;
    let mut keys = [false, false, false, false];
    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(&mut window) {
        window.draw_2d(&e, |c, g| {
            let transform = c.transform.scale(4.0, 4.0).trans(0.0, 24.0).trans(titlex, titley);

            clear([0.0, 0.0, 0.0, 1.0], g);
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                "   I",
                &mut glyphs,
                &c.draw_state,
                transform, g
            ).unwrap();
            text::Text::new_color([0.6, 0.6, 1.0, 1.0], 32).draw(
                "M RAGE",
                &mut glyphs,
                &c.draw_state,
                transform, g
            ).unwrap();
        });

        match e.press_args() {
            Some(Button::Keyboard(Key::D)) => { println!("Pressed D"); keys[0] = true },
            Some(Button::Keyboard(Key::A)) => keys[1] = true,
            Some(Button::Keyboard(Key::S)) => keys[2] = true,
            Some(Button::Keyboard(Key::W)) => keys[3] = true,
            _ => (),
        }
        match e.release_args() {
            Some(Button::Keyboard(Key::D)) => { println!("Released D"); keys[0] = false },
            Some(Button::Keyboard(Key::A)) => keys[1] = false,
            Some(Button::Keyboard(Key::S)) => keys[2] = false,
            Some(Button::Keyboard(Key::W)) => keys[3] = false,
            _ => (),
        }
        if keys[0] {
            titlex += 1.0;
        }
        if keys[1] {
            titlex -= 1.0;
        }
        if keys[2] {
            titley += 1.0;
        }
        if keys[3] {
            titley -= 1.0;
        }
    }
}
