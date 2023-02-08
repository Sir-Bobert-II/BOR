use serde_derive::*;

use serenity::model::prelude::PartialChannel;
use serenity::{json, model::prelude::GuildId};
use std::{
    fs::{self, create_dir_all, read_to_string},
    io::{Error, ErrorKind},
    path::PathBuf,
};
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
        let contents = read_to_string(path)?;
        let words: Self = match json::prelude::from_str(&contents)
        {
            Ok(x) => x,
            Err(x) => return Err(Error::new(ErrorKind::Other, x)),
        };

        Ok(words)
    }
}

strike! {
    #[strikethrough[derive(Deserialize, Serialize, Debug, Clone, Default)]]
    #[strikethrough[serde(rename_all = "camelCase")]]
    pub struct GuildSettings
    {
        pub guilds: Vec<pub struct GuildSetting {
            /// Guild Id
            pub gid: GuildId,
            pub settings:
            pub struct Settings {

                /// The maximum number of characters to return from the wiki command
                pub wiki_limit: Option<usize>,

                /// The channel to log events
                pub log_channel: Option<PartialChannel>,

                /// Additional restricted words local to a guild
                pub restricted_words: Vec<String>,

                /// How to behave when a warning limit is reached
                pub warning_behavior:
                #[derive(Copy)]
                pub enum WarnBehavior{

                    /// Do nothing, no warning limit.
                    #[default]
                    Nothing,

                    /// Ban a user after a specified number of warnings
                    Ban(u8),

                    /// Kick a user after a specified number of warnings
                    Kick(u8),

                    /// Timeout a user after a specified number of warnings for a specified
                    /// lentgh of time
                    Timeout{
                        /// The number of warnings
                        warning_count: u8,

                        /// How long to timeout for
                        duration: crate::builtins::moderation::timeout::TimeoutTime,
                    },
                },
            }
        }>
    }
}

impl GuildSettings
{
    /// Create a new, empty settings structure
    pub fn new() -> Self { Self { guilds: Vec::new() } }

    /// Add a guild's settings
    pub fn add_guild(&mut self, gid: GuildId, settings: Settings) -> &mut Self
    {
        self.guilds.push(GuildSetting { gid, settings });
        self
    }

    /// If found, returns the location of the guild
    pub fn has_guild(&self, gid: &GuildId) -> (&Self, Option<usize>)
    {
        if let Some(pos) = self.guilds.clone().iter().position(|g| g.gid == *gid)
        {
            (self, Some(pos))
        }
        else
        {
            (self, None)
        }
    }

    /// Remove a guild's settings
    pub fn _remove_guild(&mut self, gid: GuildId) -> Result<&mut Self, ()>
    {
        if let Some(pos) = self.guilds.clone().iter().position(|g| g.gid == gid)
        {
            self.guilds.remove(pos);
            Ok(self)
        }
        else
        {
            Err(())
        }
    }

    /// Load the settings from disk
    pub fn load(path: PathBuf) -> Result<Self, Error>
    {
        let contents = read_to_string(path)?;

        let settings: GuildSettings = match serde_json::from_str(&contents)
        {
            Ok(x) => x,
            Err(x) =>
            {
                println!("{contents}");
                return Err(Error::new(ErrorKind::Other, x.to_string()));
            }
        };

        Ok(settings)
    }

    /// Save the settings to disk
    pub fn save(&self, path: PathBuf) -> Result<&Self, Error>
    {
        // If there's a parent to this path, ensure it exists
        if let Some(parent) = path.parent()
        {
            if !parent.exists()
            {
                create_dir_all(parent)?;
            }
        }

        let serialized = serde_json::to_string(&self).unwrap();
        fs::write(path, serialized)?;

        Ok(self)
    }
}

impl Settings
{
    /// Create a new Settings structure
    pub fn new() -> Self
    {
        Self {
            ..Default::default()
        }
    }

    /// Set the setting's log channel
    pub fn set_log_channel(&mut self, c: PartialChannel) -> &mut Self
    {
        self.log_channel = Some(c);
        self
    }

    /// Set the restricted words
    pub fn _set_restricted_words(&mut self, words: Vec<String>) -> &mut Self
    {
        self.restricted_words = words;
        self
    }

    pub fn set_wiki_limit(&mut self, limit: usize) -> &mut Self
    {
        self.wiki_limit = Some(limit);
        self
    }

    /// Append a restricted word
    pub fn _add_restricted_word(&mut self, word: String) -> &mut Self
    {
        self.restricted_words.push(word);
        self
    }

    /// Remove a restricted word
    pub fn _remove_restricted_word(&mut self, word: String) -> Result<&mut Self, ()>
    {
        if let Some(pos) = self
            .restricted_words
            .clone()
            .iter()
            .position(|s| *s == word)
        {
            self.restricted_words.remove(pos);
            Ok(self)
        }
        else
        {
            Err(())
        }
    }

    /// Set the warning behavior
    pub fn set_warning_behavior(&mut self, b: WarnBehavior) -> &mut Self
    {
        self.warning_behavior = b;
        self
    }
}

strike! {
    #[strikethrough[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Default)]]
    #[strikethrough[serde(rename_all = "camelCase")]]
    pub struct Config
    {
        pub secrets: pub struct
        {
            pub token: String,
            pub keys: pub struct ApiKeys
            {
                pub currency_api: String,
            }
        },

        pub resources: pub struct Resources
        {
            #[serde(default = "_d_restricted_words")]
            pub restricted_words: PathBuf,

            #[serde(default = "_d_warnings")]
            pub warnings: PathBuf,

            #[serde(default = "_d_guild_settings" )]
            pub guild_settings: PathBuf,
        },

    }
}

fn _d_restricted_words() -> PathBuf { PathBuf::from("/etc/bor/restricted_words.json") }

fn _d_warnings() -> PathBuf { PathBuf::from("/var/local/bor/warnings.json") }

fn _d_guild_settings() -> PathBuf { PathBuf::from("/var/local/bor/guild_settings.json") }

impl Config
{
    pub fn from(path: PathBuf) -> Result<Self, Error>
    {
        let contents = read_to_string(path)?;

        let config: Config = match toml::from_str(&contents)
        {
            Ok(x) => x,
            Err(x) =>
            {
                println!("{contents}");
                return Err(Error::new(ErrorKind::Other, x.to_string()));
            }
        };

        Ok(config)
    }

    pub fn _save(&self, path: PathBuf) -> Result<(), Error>
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
