use anyhow::{anyhow, Result};
use serde::{de, Deserialize, Deserializer};
use std::{fmt::Debug, fmt::Display, fs::File, io::Read, path::PathBuf, str::FromStr};

#[derive(Debug, Deserialize)]
struct Team {
    id: u16,
    name: String,
    #[serde(rename(deserialize = "link"))]
    team_details: String,
}

#[derive(Clone, Copy)]
enum Drafter {
    Rory,
    Jordy,
    Connor,
    Dan,
}

impl Debug for Drafter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for Drafter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rory => write!(f, "Rory"),
            Self::Jordy => write!(f, "Jordy"),
            Self::Connor => write!(f, "Connor"),
            Self::Dan => write!(f, "Dan"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Schedule {
    games: Vec<Game>,
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

#[derive(Debug, Deserialize)]
struct Game {
    date: String,
    #[serde(rename = "awayTeam")]
    away_team: Team,
    #[serde(rename = "homeTeam")]
    home_team: Team,
    #[serde(skip)]
    available_seats: (Option<Drafter>, Option<Drafter>),
}

pub fn load_games(game_file: PathBuf) -> Result<Schedule> {
    if !game_file.exists() {
        return Err(anyhow!("{:?} does not exist!", game_file));
    }

    let mut file = File::open(game_file).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let s: Schedule = serde_json::from_str(&data).unwrap();

    Ok(s)
}
