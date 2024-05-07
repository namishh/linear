pub mod auth;
pub mod game;
pub mod help;
pub mod helpers;
pub mod tests;
use crate::Data;

pub fn get_commands(
) -> Vec<poise::Command<Data, Box<dyn std::error::Error + std::marker::Send + Sync>>> {
    let commands: Vec<poise::Command<Data, Box<dyn std::error::Error + std::marker::Send + Sync>>> = vec![
        tests::ping(),
        tests::avatar(),
        // Auth
        auth::register_team(),
        auth::register_user(),
        auth::login_team(),
        auth::logout(),
        // The Game
        game::get_question(),
        game::answer(),
        game::hint(),
        game::giveaway(),
        game::leaderboard(),
        // Silly Help Function
        help::howto(),
    ];
    commands
}
