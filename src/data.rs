use bincode;
use chrono::{DateTime, Duration, Utc};
use log::info;
// use poloto::build::SinglePlotBuilder;
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
        if let Ok(mut f) = fs::File::open(&path) {
            let mut encoded = Vec::new();
            if f.read_to_end(&mut encoded).is_err() {
                return None;
            }
            info!("Loaded Usage Data from '{}'", path.display());
            bincode::deserialize(&encoded[..]).ok()
        } else {
            None
        }
    }

    pub fn save(&self, path: path::PathBuf) -> Result<(), bincode::Error>
    {
        if let Ok(mut f) = fs::File::create(&path) {
            let encoded = bincode::serialize(&self)?;
            match f.write_all(&encoded) {
                Err(e) => return Err(bincode::Error::from(e)),
                _ => (),
            }
        }

        info!("Saved UsageData to '{}'", path.display());

        Ok(())
    }

    pub async fn increment_user_count(&mut self, http: &http::Http, uid: UserId, count: u64)
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

// pub async fn build_graphs()
// {
//     use poloto::{build, num::timestamp};
//     let data = crate::DATA.lock().await;

//     let scratch = crate::CONFIG.resources.scratch.clone();
//     if !scratch.exists() {
//         std::fs::create_dir_all(&scratch).unwrap();
//     }

//     // Create a plot for each guild's usage
//     for (gid, gdat) in data.gdata.iter() {
//         let data = gdat
//             .requests
//             .iter()
//             .map(|(x, y)| (timestamp::UnixTime::from(*y), *x as f64));

//         // Create a histogram  for each guild containing its usage over time.
// Save this svg         let plot =
// poloto::data(poloto::plots!(build::plot("").histogram(data)))
// .build_and_label((                 format!("Command Calls for '{} ({})'",
// gdat.gname, gid),                 "Time",
//                 "Command Calls",
//             ))
//             .append_to(poloto::header().dark_theme())
//             .render_string()
//             .unwrap();

//         // Write the plot svg to a temporary file
//         let mut f =
//             std::fs::File::create(scratch.join(format!("{}.svg",
// gid.to_string()))).unwrap();         f.write_all(plot.as_bytes()).unwrap();
//     }

//     let _d = data.udata.iter().next().unwrap().1
//     .requests
//     .iter()
//     .map(|(x, y)| (timestamp::UnixTime::from(*y), *x as f64));

//     let mut plot =
//         build::plot(format!(
//             "Command Calls for '{} ({})'",
//             data.udata[0].gname,
//             data.udata[0]
//         )).line(_d);

//     // Create a plot for user usage
//     for (uid, udat) in data.udata.iter().skip(1) {
//         let data = udat
//             .requests
//             .iter()
//             .map(|(x, y)| (timestamp::UnixTime::from(*y), *x as f64));


//         for dat in data.into_iter().skip(1)
//         {
//             plot.chain(build::plot(format!(
//                 "Command Calls for '{} ({})'",
//                 dat.1.gname,
//                 data.gdata[0].0
//             )).line(_d));
//         }
//         let plot = poloto::data(poloto::plots!())
//             .build_and_label((
//                 "Command calls per user",
//                 "Time",
//                 "Command Calls",
//             ))
//             .append_to(poloto::header().dark_theme())
//             .render_string()
//             .unwrap();
//         let mut f =
//             std::fs::File::create(scratch.join(format!("user-{}.svg",
// uid.to_string()))).unwrap();         f.write_all(plot.as_bytes()).unwrap();
//     }
// }

/// Clean old data
pub async fn mangage_data()
{
    let mut data = crate::DATA.lock().await;
    let threshold = Utc::now() - Duration::days(30);

    // Remove data older than the threshold
    for (_, gdat) in data.gdata.iter_mut() {
        gdat.requests
            .retain(|(_, timestamp)| *timestamp > threshold)
    }
    for (_, udat) in data.udata.iter_mut() {
        udat.requests
            .retain(|(_, timestamp)| *timestamp > threshold)
    }
}
