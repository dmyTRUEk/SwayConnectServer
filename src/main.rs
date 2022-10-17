// main.rs

use std::{
    fmt::Display,
    net::Ipv4Addr,
};

#[macro_use] extern crate rocket;
use rocket::config::Config;



trait ExtResultErrorToString<T> {
    fn map_err_to_string(self) -> Result<T, String>;
}
impl<T, E: Display> ExtResultErrorToString<T> for Result<T, E> {
    fn map_err_to_string(self) -> Result<T, String> {
        self.map_err(|e| e.to_string())
    }
}


mod event {
    use std::{env::var, process::Command};
    use crate::ExtResultErrorToString;

    fn get_xdg_seat() -> String {
        var("XDG_SEAT").unwrap()
    }

    pub mod mouse {
        use super::*;

        #[allow(non_snake_case)] // because this is like enum with values
        #[derive(Debug, Clone, Copy)]
        pub enum Button {
            Primary,   // aka Left
            Secondary, // aka Right
            Tertiary,  // aka Middle
        }

        fn button_to_name<'a>(button: Button) -> &'a str {
            match button {
                Button::Primary => { "button1" }
                Button::Secondary => { "button3" }
                Button::Tertiary => { "button2" }
            }
        }

        pub fn move_(dx: i16, dy: i16) -> Result<(), String> {
            // swaymsg seat $XDG_SEAT cursor move '"+300"' '"+100"'
            Command::new(format!("swaymsg"))
                .args([
                    "seat",
                    &get_xdg_seat(),
                    "cursor",
                    "move",
                    &format!("'{}'", dx),
                    &format!("'{}'", dy)
                ])
                .output() // `output` or `spawn`?
                .map_err_to_string()?;
            Ok(())
        }

        pub fn click(button: Button) -> Result<(), String> {
            // read xdg seat
            let xdg_seat: &str = &get_xdg_seat();
            // press and release
            _press(button, xdg_seat)?;
            _release(button, xdg_seat)?;
            Ok(())
        }

        fn _press(button: Button, xdg_seat: &str) -> Result<(), String> {
            Command::new(format!("swaymsg"))
                .args(["seat", xdg_seat, "cursor", "press", button_to_name(button)])
                .output()
                .map_err_to_string()?;
            Ok(())
        }

        pub fn press(button: Button) -> Result<(), String> {
            let xdg_seat: &str = &get_xdg_seat();
            _press(button, xdg_seat)?;
            Ok(())
        }

        fn _release(button: Button, xdg_seat: &str) -> Result<(), String> {
            Command::new(format!("swaymsg"))
                .args(["seat", xdg_seat, "cursor", "release", button_to_name(button)])
                .output()
                .map_err_to_string()?;
            Ok(())
        }

        pub fn release(button: Button) -> Result<(), String> {
            let xdg_seat: &str = &get_xdg_seat();
            _release(button, xdg_seat)?;
            Ok(())
        }
    }

    mod keyboard {
        // use super::*;

        pub fn click_key(key_name: &str) -> Result<(), String> {
            press_key(key_name)?;
            release_key(key_name)?;
            Ok(())
        }

        pub fn press_key(key_name: &str) -> Result<(), String> {
            todo!()
        }

        pub fn release_key(key_name: &str) -> Result<(), String> {
            todo!()
        }
    }
}



// TODO: refactor

#[get("/mouse_move/<dx>/<dy>")]
fn mouse_move(dx: i16, dy: i16) -> Result<String, String> {
    event::mouse::move_(dx, dy)?;
    Ok(format!("OK"))
}

#[get("/mouse_click_primary")]
fn mouse_click_primary() -> Result<String, String> {
    event::mouse::click(event::mouse::Button::Primary)?;
    Ok(format!("OK"))
}

#[get("/mouse_click_secondary")]
fn mouse_click_secondary() -> Result<String, String> {
    event::mouse::click(event::mouse::Button::Secondary)?;
    Ok(format!("OK"))
}

#[get("/mouse_click_tertiary")]
fn mouse_click_tertiary() -> Result<String, String> {
    event::mouse::click(event::mouse::Button::Tertiary)?;
    Ok(format!("OK"))
}

#[launch]
fn rocket() -> _ {
    let config = Config {
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        ..Config::default()
    };
    rocket::custom(config)
        .mount("/", routes![
            mouse_move,
            mouse_click_primary,
            mouse_click_secondary,
            mouse_click_tertiary,
        ])
}

