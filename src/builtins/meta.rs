use serenity::builder::CreateApplicationCommand;

use crate::NAME;
const SITE: &str = "https://github.com/El-Wumbus/Law-Enforcement-Bot";
const AUTHORS: [&str; 1] = ["Decator <decator.c@proton.me>"];

pub fn meta() -> String
{
    let mut buffer = String::new();

    for author in AUTHORS
    {
        buffer.push_str(&format!(" {author}"));
    }

    format!("Written by:{buffer}.\nSee the source code: {SITE}")
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand
{
    command
        .name("meta")
        .description("Info about this bot")
        .dm_permission(true)
}
