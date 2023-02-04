use serenity::{builder::CreateApplicationCommand, model::prelude::command::CommandOptionType};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
{
    command
        .name("random")
        .dm_permission(true)
        .description("Pseudo-Randomness")
        .create_option(|option| {
            option
                .name("coin")
                .kind(CommandOptionType::SubCommand)
                .description("Flip a coin")
        })
        .create_option(|option| {
            option
                .name("roulette")
                .kind(CommandOptionType::SubCommand)
                .description("Play russian roulette")
        })
}

pub fn coin() -> String
{
    let mut rng = rand::thread_rng();

    match rand::Rng::gen_bool(&mut rng, 1.0 / 2.0)
    {
        true => String::from("Heads"),
        false => String::from("Tails"),
    }
}

pub fn roulette() -> String
{
    let mut rng = rand::thread_rng();
    match rand::Rng::gen_bool(&mut rng, 1.0 / 6.0)
    {
        true => "Dead".to_string(),
        false => "Alive".to_string(),
    }
}
