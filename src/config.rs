use std::{fs::{read_to_string}, path::PathBuf};
use serde_derive::*;
use serenity::json;
use structstruck::strike;
use crate::error::Error;

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
            pub restricted_words: PathBuf
        }
    }
}

impl Config {
    pub fn from(path: PathBuf) -> Result<Self, Error> {
        let contents = match read_to_string(path)
        {
            Ok(x) =>x,
            Err(x) => return Err(Error::new(x.to_string(), 1).fatal())
        };
        
        let config: Config = match toml::from_str(&contents)
        {
            Ok(x)=>x,
            Err(x) => {
                println!("{}", contents);
                return Err(Error::new(x.to_string(), 1).fatal())}
        };

        Ok(config)
    }
}
