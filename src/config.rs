use std::{fs::{read_to_string}, path::PathBuf, io::{Error, ErrorKind}};
use serde_derive::*;
use serenity::json;
use structstruck::strike;

#[derive(Deserialize, Clone, Serialize)]
pub struct RestrictedWords
{
    pub words: Vec<String>,
}

impl RestrictedWords
{
    pub fn from(path: PathBuf) -> Result<Self, Error>
    {
        let contents = match read_to_string(path)
        {
            Ok(x) =>x,
            Err(x) => return Err(Error::new(x.to_string(), 1).fatal())
        };
        
        let words: Self = match json::prelude::from_str(&contents)
        {
            Ok(x)=>x,
            Err(x) => return Err(Error::new(x.to_string(), 1).fatal())
        };

        Ok(words)
    }
}

strike!
{
    pub struct GuildSettings
    {
        guilds: Vec<pub struct GuildSetting {
            /// Guild Id
            gid: GuildId,
            settings: pub struct Settings {
                pub warning_behavior: pub enum {
                    Ban(u8),
                    Kick(u8),
                    Timeout {
                        warning_count: u8,
                        duration: crate::builtins::users::timeout::TimeoutTime,
                    }
                }
            }
        }>
    }
}

strike! {
    #[strikethrough[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]]
    #[strikethrough[serde(rename_all = "camelCase")]]
    pub struct Config
    {
        pub secrets: pub struct
        {
            pub token: String
        },

        pub resources: pub struct
        {
            pub restricted_words: PathBuf,
            pub warnings: PathBuf,
            pub guild_settings: PathBuf,
        }

    }
}

impl Config {
    pub fn from(path: PathBuf) -> Result<Self, Error> {
        let contents = match read_to_string(path)?;
        
        let config: Config = match toml::from_str(&contents)
        {
            Ok(x)=>x,
            Err(x) => {
                println!("{}", contents);
                return Err(Error::new(ErrorKind::Other, x.to_string()));
            }
        };

        Ok(config)
    }

    pub fn save(&self, path: PathBuf) -> Result<(), Error>
    {
        // If there's a parent to this path, ensure it exists
        if let Some(parent) = path.parent()
        {
            if !parent.exists()
            {
                create_dir_all(parent)?;
            }
        }

        let serialized = toml::ser::to_string(&self).unwrap();
        fs::write(path, serialized)?;

        Ok(())
    }
}
