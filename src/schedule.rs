use anyhow::Result;
use chrono::Month;
use serde::Deserialize;
use std::fmt::Display;
use wasm_bindgen::prelude::*;

use crate::{drafter::Drafter, games::NHL_KRAKEN_2021_SEASON};

#[derive(Debug, Deserialize, Clone)]
pub struct Team {
    id: u16,
    name: String,
    #[serde(rename(deserialize = "link"))]
    team_details: String,
}

const LOGO_URL_BASE: &'static str =
    "https://www-league.nhlstatic.com/images/logos/teams-current-primary-light/";

impl Team {
    pub fn get_logo_url(&self) -> String {
        format!("{}{}.svg", LOGO_URL_BASE, self.id)
    }

    pub fn get_team_name(&self) -> String {
        self.name.clone()
    }
}

#[wasm_bindgen]
#[derive(Debug, Deserialize, Clone)]
pub struct Game {
    date: String,
    #[serde(rename = "awayTeam")]
    away_team: Team,
    #[serde(rename = "homeTeam")]
    home_team: Team,
    #[serde(skip)]
    available_seats: (Option<Drafter>, Option<Drafter>),
}

// #[wasm_bindgen]
// impl Game {
//     pub fn render() -> String {
//         format!(r#"<div id=""></div>"#)
//     }
// }

impl Game {
    pub fn render(&self) -> String {
        format!(
            r#"<div id="game-{}" class="scheduled-game"><img src="{}"/><p>{:?}</p><p>{:?}</p></div>"#,
            self.date,
            self.away_team.get_logo_url(),
            self.available_seats.0,
            self.available_seats.1
        )
    }

    pub fn get_away_team(&self) -> &Team {
        &self.away_team
    }
    pub fn get_seat_details(&self) -> (Option<Drafter>, Option<Drafter>) {
        self.available_seats
    }

    pub fn add_drafter(&mut self, drafter: Drafter) -> Result<()> {
        // try and add to seat one:

        if self.available_seats.0.is_none() {
            self.available_seats.0 = Some(drafter);
            return Ok(());
        }

        if self.available_seats.1.is_none() {
            self.available_seats.1 = Some(drafter);
            return Ok(());
        }

        Err(anyhow::Error::msg("both seats are taken"))
    }
}

#[derive(Debug, Deserialize)]
pub struct Schedule {
    games: Vec<Game>,
}

impl Schedule {
    pub fn new_2021_season() -> Self {
        serde_json::from_str(NHL_KRAKEN_2021_SEASON).unwrap()
    }

    pub fn get_game_details(&self, day: u32, month: Month, year: i32) -> Option<&Game> {
        let month_format = match month.number_from_month() {
            m_num if m_num < 10 => format!("0{}", m_num),
            m_num => format!("{}", m_num),
        };

        let day_format = match day {
            d if d < 10 => format!("0{}", d),
            d => format!("{}", d),
        };

        let date = format!("{}-{}-{}", year, month_format, day_format);

        println!("finalized date: {:?}", date);
        let games = self
            .games
            .iter()
            .filter(|g| g.date.starts_with(&date))
            .collect::<Vec<&Game>>();

        if games.is_empty() {
            return None;
        }

        Some(*games.first().unwrap())
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

#[cfg(test)]
mod tests {
    use chrono::Month;

    use crate::schedule::Schedule;

    #[test]
    pub fn game_exists() {
        let schedule = Schedule::new_2021_season();

        let game = schedule.get_game_details(24, Month::October, 2021);

        assert!(game.is_some());

        assert_eq!(
            game.unwrap().get_away_team().get_team_name(),
            "Vancouver Canucks"
        );
    }

    #[test]
    pub fn game_exists_2022() {
        let schedule = Schedule::new_2021_season();

        let game = schedule.get_game_details(5, Month::January, 2022);

        assert!(game.is_some());

        assert_eq!(
            game.unwrap().get_away_team().get_team_name(),
            "New York Islanders"
        );
    }
}
