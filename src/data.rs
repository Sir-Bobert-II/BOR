use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serenity::model::prelude::{GuildId, UserId};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct UsageData
{
    gdata: HashMap<GuildId, GuildData>,
    udata: HashMap<UserId, UserData>,
    cdata: HashMap<String, CommandData>,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommandData
{
    cname: String,

    /// How many requests (command invocation) the server has made to this server over time
    requests: Vec<(u64, DateTime<Utc>)>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GuildData
{
    gid: GuildId,
    gname: String,

    /// How many requests (command invocation) the server has made to this server over time
    requests: Vec<(u64, DateTime<Utc>)>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserData
{
    uid: UserId,
    uname: String,

    /// How many requests (command invocation) the user has made to this server over time
    requests: Vec<(u64, DateTime<Utc>)>,
}

impl Gdata
{
    fn load() -> Self
    {

    }
}

impl Udata
{

}