use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serenity::model::prelude::{GuildId, UserId};

pub struct Gdata(HashMap<GuildId, GuildData>);
pub struct Udata(HashMap<UserId, UserData>);

#[derive(Debug, Clone, Hash, Default)]
pub struct GuildData
{
    gid: GuildId,
    gname: String,

    /// How many requests (command invocation) the server has made to this server over time
    requests: Vec<(u64, DateTime<Utc>)>,
}

#[derive(Debug, Clone, Hash, Default)]
pub struct UserData
{
    uid: UserId,
    uname: String,

    /// How many requests (command invocation) the user has made to this server over time
    requests: Vec<(u64, DateTime<Utc>)>,
}

impl Gdata
{
    
}

impl Udata
{

}