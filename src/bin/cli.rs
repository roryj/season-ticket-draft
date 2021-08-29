use anyhow::Result;
use season_ticket_draft::load_games;
use std::path::PathBuf;

fn main() -> Result<()> {
    let schedule = load_games(PathBuf::from(
        "/Users/roryj/development/season-ticket-draft/resources/games.json",
    ))?;

    println!("{}", schedule);

    Ok(())
}
