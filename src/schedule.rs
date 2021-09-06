use serde::Deserialize;
use std::fmt::Display;
use wasm_bindgen::prelude::*;

use crate::{drafter::Drafter, games::NHL_KRAKEN_2021_SEASON};

#[derive(Debug, Deserialize)]
struct Team {
    id: u16,
    name: String,
    #[serde(rename(deserialize = "link"))]
    team_details: String,
}

#[derive(Debug, Deserialize)]
struct Game {
    #[serde(skip)]
    id: String,
    date: String,
    #[serde(rename = "awayTeam")]
    away_team: Team,
    #[serde(rename = "homeTeam")]
    home_team: Team,
    #[serde(skip)]
    available_seats: (Option<Drafter>, Option<Drafter>),
}

#[wasm_bindgen]
#[derive(Debug, Deserialize)]
pub struct Schedule {
    games: Vec<Game>,
}

#[wasm_bindgen]
impl Schedule {
    pub fn new_2021_season() -> Self {
        serde_json::from_str(NHL_KRAKEN_2021_SEASON).unwrap()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn add_ticket_owner(&self, game_id: String, drafter: Drafter) {
        println!("updating game: {} -> {}", game_id, drafter);
    }
}

impl Display for Schedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:-<100}", "-");
        writeln!(
            f,
            "{:^25}|{:^25}|{:^25}|{:^10}|{:^10}",
            "date", "away", "home", "seat1", "seat2"
        );
        self.games.iter().for_each(|g| {
            let seat1 = g
                .available_seats
                .0
                .map_or_else(|| String::from("None"), |s| s.to_string());
            let seat2 = g
                .available_seats
                .1
                .map_or_else(|| String::from("None"), |s| s.to_string());
            writeln!(f, "{:-<100}", "-");
            writeln!(
                f,
                "{:^25}|{:^25}|{:^25}|{:^10}|{:^10}",
                g.date, g.away_team.name, g.home_team.name, seat1, seat2
            );
        });
        Ok(())
    }
}
