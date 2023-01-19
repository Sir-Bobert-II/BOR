use std::{fs::{read_to_string}, path::PathBuf, io::{Error, ErrorKind}};
use serde_derive::*;
use serenity::json;
use structstruck::strike;
use serenity::model::channel::Channel;

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
    #[strikethrough[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Default)]]
    #[strikethrough[serde(rename_all = "camelCase")]]
    pub struct GuildSettings
    {
        pub guilds: Vec<pub struct GuildSetting {
            /// Guild Id
            pub gid: GuildId,
            pub settings: pub struct Settings {
                
                /// The channel to log events
                pub log_channel: Option<Channel>,

                /// Additional restricted words local to a guild
                pub restricted_words: Vec<String>,

                /// How to behave when a warning limit is reached
                pub warning_behavior: pub enum WarnBehavior{

                    /// Do nothing, no warning limit.
                    #[default]
                    Nothing,

                    /// Ban a user after a specified number of warnings
                    Ban(u8),

                    /// Kick a user after a specified number of warnings
                    Kick(u8),

                    /// Timeout a user after a specified number of warnings for a specified
                    /// lentgh of time
                    Timeout {
                        /// The number of warnings
                        pub warning_count: u8,

                        /// How long to timeout for
                        pub duration: crate::builtins::users::timeout::TimeoutTime,
                    },
                },

                >

            }
        }>
    }
}

impl GuildSettings
{

    /// Create a new, empty settings structure
    pub fn new() -> Self
    {
        Self{Default::default()}
    }

    /// Add a guild's settings
    pub fn add_guild(&mut self, gid:GuildId, settings: Settings) -> &mut Self
    {
        self.guilds.push(GuildSetting{gid,settings});
        self
    }

    /// Remove a guild's settings
    pub fn remove_guild(&mut self, gid: GuildId) -> &mut Result<&mut Self, ()>
    {
        if let Some(pos) = self.restricted_words.clone().iter().position(|g| g.gid == gid)
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
    pub fn load(path:PathBuf) -> Result<Self, Error>
    {
        let contents = match read_to_string(path)?;
        
        let config: Config = match serde_json::from_str(&contents)
        {
            Ok(x)=>x,
            Err(x) => {
                println!("{}", contents);
                return Err(Error::new(ErrorKind::Other, x.to_string()));
            }
        };

        Ok(config)
    }

    /// Save the settings to disk
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

        let serialized = serde_json::to_string(&self).unwrap();
        fs::write(path, serialized)?;

        Ok(())
    }

}

impl Settings
{
    /// Create a new Settings structure
    pub fn new() -> Self
    {
        Self{Default::default()}
    }

    /// Set the setting's log channel
    pub fn set_log_channel(&mut self, c: Channel) -> &mut Self
    {
        self.log_channel = Some(c);
        self
    }

    /// Set the restricted words
    pub fn set_restricted_words(&mut self, words: Vec<String>) -> &mut Self
    {
        self.restricted_words = words;
        self
    }

    /// Append a restricted word
    pub fn add_restricted_word(&mut self, word: String) -> &mut Self
    {
        self.restricted_words.push(word);
        self
    }

    /// Remove a restricted word
    pub fn remove_restricted_word(&mut self, word: String) -> Result<&mut Self, ()>
    {
        if let Some(pos) = self.restricted_words.clone().iter().position(|s| s == word)
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
            pub token: String
        },

        pub resources: pub struct
        {
            #[serde(default = "/etc/leb/restricted_words.json")]
            pub restricted_words: PathBuf,
            
            #[serde(default = "/var/local/leb/warnings.json")]
            pub warnings: PathBuf,

            #[serde(default = "/var/local/leb/guild_settings.json")]
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
