use serenity::builder::CreateApplicationCommand;

const SITE: &str = "https://github.com/Sir-Bobert-II/BOR";
const AUTHORS: [&str; 1] = ["Decator <decator.c@proton.me>"];

pub fn meta() -> String
{
    let mut buffer = String::new();

    for author in AUTHORS {
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
