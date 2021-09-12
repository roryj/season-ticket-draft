use anyhow::Result;
use season_ticket_draft::try_load_games;
use season_ticket_draft::web::SomeStruct;
use std::path::PathBuf;

fn main() -> Result<()> {
    let schedule =
        try_load_games("/Users/roryj/development/season-ticket-draft/resources/games.json");

    println!("{}", schedule);

    // let c = build_month(chrono::Month::October, 2021);

    // println!("{}", c);

    // let t = SomeStruct::new(String::from("helloworld"));

    // println!("{}", t);

    Ok(())
}
