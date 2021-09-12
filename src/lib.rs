pub mod calendar;
pub mod drafter;
mod games;
pub mod schedule;
pub mod web;

use anyhow::{anyhow, Result};
use schedule::Schedule;
use std::fs;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

pub fn try_load_games(game_file: &str) -> Schedule {
    let path = PathBuf::from(game_file);
    let p = path.parent().unwrap();
    for file in fs::read_dir(p).unwrap() {
        println!("{}", file.unwrap().path().display());
    }

    set_panic_hook();
    println!("calling this function");
    load_games(game_file).expect("failed to load games fron json")
}

fn load_games<P>(game_file: P) -> Result<Schedule>
where
    P: AsRef<Path>,
{
    println!("getting deeper");
    if !game_file.as_ref().exists() {
        return Err(anyhow!("{:?} does not exist!", game_file.as_ref()));
    }

    let mut file = File::open(game_file.as_ref()).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let s: Schedule = serde_json::from_str(&data).unwrap();

    Ok(s)
}

pub fn set_panic_hook() {
    /*
    When the `console_error_panic_hook` feature is enabled, we can call the
    `set_panic_hook` function at least once during initialization, and then
    we will get better error messages if our code ever panics.

    For more details see
    https://github.com/rustwasm/console_error_panic_hook#readme
    */
    // #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
