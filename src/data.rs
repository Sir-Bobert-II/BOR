use bincode;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serenity::{
    http,
    model::prelude::{GuildId, UserId},
};
use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
    path,
};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct UsageData
{
    pub gdata: HashMap<GuildId, GuildData>,
    pub udata: HashMap<UserId, UserData>,
    pub cdata: HashMap<String, CommandData>,
}

impl UsageData
{
    pub fn load(path: path::PathBuf) -> Option<Self>
    {
        if let Ok(mut f) = fs::File::open(path) {
            let mut encoded = Vec::new();
            if f.read_to_end(&mut encoded).is_err() {
                return None;
            }
            bincode::deserialize(&encoded[..]).ok()
        } else {
            None
        }
    }

    pub fn save(&self, path: path::PathBuf) -> Result<(), bincode::Error>
    {
        if let Ok(mut f) = fs::File::create(path) {
            let encoded = bincode::serialize(&self)?;
            match f.write_all(&encoded) {
                Err(e) => return Err(bincode::Error::from(e)),
                _ => (),
            }
        }

        Ok(())
    }

    pub async fn increment_user_count(
        &mut self,
        http: &http::Http,
        uid: UserId,
        count: u64,
    ) -> Result<(), ()>
    {
        if !self.udata.contains_key(&uid) {
            self.udata.insert(
                uid,
                UserData {
                    uid,
                    uname: uid.to_user(http).await.unwrap().name,
                    requests: vec![(count, Utc::now())],
                },
            );
        }

        self.udata.get_mut(&uid).unwrap().increment(count);
        Ok(())
    }

    pub fn increment_command_count(&mut self, command_name: String, count: u64)
    {
        if !self.cdata.contains_key(&command_name) {
            self.cdata.insert(
                command_name.clone(),
                CommandData {
                    cname: command_name,
                    requests: vec![(count, Utc::now())],
                },
            );
        } else {
            self.cdata.get_mut(&command_name).unwrap().increment(count);
        }
    }

    pub async fn increment_guild_count(&mut self, http: &http::Http, gid: GuildId, count: u64)
    {
        if !self.gdata.contains_key(&gid) {
            self.gdata.insert(
                gid,
                GuildData {
                    gid,
                    gname: gid.to_partial_guild(http).await.unwrap().name,
                    requests: vec![(count, Utc::now())],
                },
            );
        } else {
            self.gdata.get_mut(&gid).unwrap().increment(count);
        }
    }
}

pub trait DataManipulation
{
    fn increment(&mut self, count: u64);
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommandData
{
    cname: String,

    /// How many requests (command invocation) the server has made to this
    /// server over time
    requests: Vec<(u64, DateTime<Utc>)>,
}

impl DataManipulation for CommandData
{
    fn increment(&mut self, count: u64) { self.requests.push((count, Utc::now())) }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GuildData
{
    pub gid: GuildId,
    pub gname: String,

    /// How many requests (command invocation) the server has made to this
    /// server over time
    pub requests: Vec<(u64, DateTime<Utc>)>,
}

impl DataManipulation for GuildData
{
    fn increment(&mut self, count: u64) { self.requests.push((count, Utc::now())) }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserData
{
    uid: UserId,
    uname: String,

    /// How many requests (command invocation) the user has made to this server
    /// over time
    pub requests: Vec<(u64, DateTime<Utc>)>,
}

impl DataManipulation for UserData
{
    fn increment(&mut self, count: u64) { self.requests.push((count, Utc::now())) }
}


impl IntoIterator for UserData
{
    type Item = (u64, DateTime<Utc>);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter { self.requests.into_iter() }
}

impl IntoIterator for GuildData
{
    type Item = (u64, DateTime<Utc>);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter { self.requests.into_iter() }
}
