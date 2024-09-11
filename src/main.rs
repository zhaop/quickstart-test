// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
use quickstart_test::AppPlugin;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}
