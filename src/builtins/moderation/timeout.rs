use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use serenity::{
    builder::CreateApplicationCommand,
    model::{
        prelude::{command::CommandOptionType, GuildId, User},
        timestamp::Timestamp,
        Permissions,
    },
    prelude::Context,
};

use super::member_from_id;

#[derive(
    PartialEq, Eq, PartialOrd, Ord, Debug, Default, Clone, Copy, Deserialize, Serialize,
)]
pub struct TimeoutTime
{
    pub seconds: Option<i64>,
    pub minutes: Option<i64>,
    pub hours: Option<i64>,
    pub days: Option<i64>,
}

impl TimeoutTime
{
    pub fn is_none(&self) -> bool
    {
        if self.seconds.is_none() && self.minutes.is_none() && self.hours.is_none() && self.days.is_none()
        {
            true
        }
        else {
            false
        }
    }
}

pub fn generate_ending_time(time: TimeoutTime) -> Timestamp
{
    let mut when = Utc::now();
    if let Some(seconds) = time.seconds
    {
        when += Duration::seconds(seconds);
    }
    if let Some(minutes) = time.minutes
    {
        when += Duration::minutes(minutes);
    }
    if let Some(hours) = time.hours
    {
        when += Duration::hours(hours);
    }
    if let Some(days) = time.days
    {
        when += Duration::days(days);
    }

    Timestamp::from_unix_timestamp(when.timestamp()).unwrap()
}

pub async fn timeout(
    context: &Context,
    gid: &GuildId,
    user: User,
    time: TimeoutTime,
    auto: bool,
) -> Option<String>
{
    let mut member = member_from_id(&context, *gid, user.id).await;
    match member
        .disable_communication_until_datetime(&context.http, generate_ending_time(time))
        .await
    {
        Ok(_) =>
        {
            if !auto
            {
                Some(format!("Timed out user {}", user.name))
            }
            else
            {
                None
            }
        }
        Err(x) => Some(format!("Error: {x}")),
    }
}
    
pub async fn release(context: &Context, gid: &GuildId, user: User) -> String
{
    let mut member = member_from_id(&context, *gid, user.id).await;
    match member.enable_communication(&context.http).await
    {
        Ok(_) =>
        {
            format!("Released user {} from their timeout.", user.name)
        }
        Err(x) => format!("Error: {x}"),
    }
}

pub fn register_realease(
    command: &mut CreateApplicationCommand,
) -> &mut CreateApplicationCommand
{
    command
        .name("release")
        .description("Release a member from their timeout")
        .dm_permission(false)
        .default_member_permissions(Permissions::MODERATE_MEMBERS)
        .create_option(|option| {
            option
                .name("user")
                .description("The user to release")
                .kind(CommandOptionType::User)
                .required(true)
        })
}