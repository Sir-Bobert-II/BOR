use log::error;
use serenity::{
    model::prelude::{
        command::CommandOptionType,
        interaction::{
            application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
            InteractionResponseType,
        },
    },
    prelude::Context,
};

use crate::{
    builtins::{self, meta, moderation},
    config::WarnBehavior,
};
pub async fn run(context: Context, command: ApplicationCommandInteraction)
{
    let command_name = command.data.name.as_str();
    let content = match command_name
    {
        "meta" => meta::meta(),

        "moderation" =>
        {
            let mut ret = "Failed".to_string();
            for option in command.data.options.clone()
            {
                match option.kind
                {
                    CommandOptionType::SubCommand => match &*option.name
                    {
                        "kick" =>
                        {
                            let mut user = None;
                            let mut reason = "No reason provided.".to_string();
                            for option in option.options
                            {
                                let opt = option.resolved.unwrap();

                                match &*option.name
                                {
                                    "user" =>
                                    {
                                        if let CommandDataOptionValue::User(u, _) = opt
                                        {
                                            user = Some(u);
                                        }
                                    }
                                    "reason" =>
                                    {
                                        if let CommandDataOptionValue::String(r) = opt
                                        {
                                            reason = r.clone();
                                        }
                                    }

                                    _ => unreachable!(),
                                }
                            }

                            if user.is_none()
                            {
                                error!("Cannot respond to slash command: No 'User' provided");
                                return;
                            }
                            let user = user.unwrap();

                            let guild_id = command.guild_id.unwrap();

                            ret = moderation::kick::run(&context, &guild_id, &user, reason).await;
                        }

                        "ban" =>
                        {
                            let mut user = None;
                            let mut days: u8 = 0;
                            let mut reason = "No reason provided.".to_string();

                            for option in option.options
                            {
                                let opt = option.resolved.unwrap();

                                match &*option.name
                                {
                                    "user" =>
                                    {
                                        if let CommandDataOptionValue::User(u, _) = opt
                                        {
                                            user = Some(u);
                                        }
                                    }
                                    "reason" =>
                                    {
                                        if let CommandDataOptionValue::String(r) = opt
                                        {
                                            reason = r.clone();
                                        }
                                    }
                                    "days" =>
                                    {
                                        if let CommandDataOptionValue::Integer(i) = opt
                                        {
                                            days = i.clamp(0, 7) as u8;
                                        }
                                    }

                                    _ => unreachable!(),
                                }
                            }

                            if user.is_none()
                            {
                                error!("Cannot respond to slash command: No 'User' provided");
                                return;
                            }
                            let user = user.unwrap();

                            let guild_id = command.guild_id.unwrap();

                            ret = moderation::ban::run(&context, &guild_id, &user, reason, days)
                                .await;
                        }

                        "warn" =>
                        {
                            let mut user = None;
                            let mut reason = "No reason provided.".to_string();
                            for option in option.options
                            {
                                let opt = option.resolved.unwrap();

                                match &*option.name.to_lowercase()
                                {
                                    "user" =>
                                    {
                                        if let CommandDataOptionValue::User(u, _) = opt
                                        {
                                            user = Some(u);
                                        }
                                    }
                                    "reason" =>
                                    {
                                        if let CommandDataOptionValue::String(r) = opt
                                        {
                                            reason = r.clone();
                                        }
                                    }

                                    _ => unreachable!(),
                                }
                            }

                            if user.is_none()
                            {
                                error!("Cannot respond to slash command: No 'User' provided");
                                return;
                            }
                            let user = user.unwrap();

                            let guild_id = command.guild_id.unwrap();

                            ret = moderation::warn::warn(&context, &guild_id, user, reason).await
                        }

                        "get_warnings" =>
                        {
                            let mut user = None;
                            for option in option.options
                            {
                                let opt = option.resolved.unwrap();

                                match &*option.name
                                {
                                    "user" =>
                                    {
                                        if let CommandDataOptionValue::User(u, _) = opt
                                        {
                                            user = Some(u);
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }

                            if user.is_none()
                            {
                                error!("Cannot respond to slash command: No 'User' provided");
                                return;
                            }
                            let user = user.unwrap();

                            let guild_id = command.guild_id.unwrap();

                            ret = moderation::warn::get_warns(&guild_id, user)
                        }

                        "remove_warns" =>
                        {
                            let mut user = None;
                            for option in option.options
                            {
                                let opt = option.resolved.unwrap();

                                match &*option.name
                                {
                                    "user" =>
                                    {
                                        if let CommandDataOptionValue::User(u, _) = opt
                                        {
                                            user = Some(u);
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }

                            if user.is_none()
                            {
                                error!("Cannot respond to slash command: No 'User' provided");
                                return;
                            }
                            let user = user.unwrap();

                            let guild_id = command.guild_id.unwrap();

                            ret = moderation::warn::remove_warns(&guild_id, user)
                        }

                        "timeout" =>
                        {
                            let mut user = None;
                            let mut time: moderation::timeout::TimeoutTime =
                                moderation::timeout::TimeoutTime::default();
                            for option in option.options
                            {
                                let opt = option.resolved.unwrap();

                                match &*option.name
                                {
                                    "user" =>
                                    {
                                        if let CommandDataOptionValue::User(u, _) = opt
                                        {
                                            user = Some(u);
                                        }
                                    }
                                    "days" =>
                                    {
                                        if let CommandDataOptionValue::Integer(t) = opt
                                        {
                                            time.days = Some(t)
                                        }
                                    }
                                    "hours" =>
                                    {
                                        if let CommandDataOptionValue::Integer(t) = opt
                                        {
                                            time.hours = Some(t)
                                        }
                                    }
                                    "minutes" =>
                                    {
                                        if let CommandDataOptionValue::Integer(t) = opt
                                        {
                                            time.minutes = Some(t)
                                        }
                                    }
                                    "seconds" =>
                                    {
                                        if let CommandDataOptionValue::Integer(t) = opt
                                        {
                                            time.seconds = Some(t)
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }

                            if user.is_none()
                            {
                                error!("Cannot respond to slash command: No 'User' provided");
                                return;
                            }

                            if time.is_none()
                            {
                                ret = "Error: No units of time were provided!".to_string()
                            }
                            else
                            {
                                ret = moderation::timeout::timeout(
                                    &context,
                                    &command.guild_id.unwrap(),
                                    user.unwrap(),
                                    time,
                                    false,
                                )
                                .await
                                .unwrap()
                            }
                        }

                        "release" =>
                        {
                            let mut user = None;
                            for option in option.options
                            {
                                let opt = option.resolved.unwrap();

                                match &*option.name
                                {
                                    "user" =>
                                    {
                                        if let CommandDataOptionValue::User(u, _) = opt
                                        {
                                            user = Some(u);
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }

                            if user.is_none()
                            {
                                error!("Cannot respond to slash command: No 'User' provided");
                                return;
                            }
                            let user = user.unwrap();

                            let guild_id = command.guild_id.unwrap();

                            ret = moderation::timeout::release(&context, &guild_id, user).await;
                        }

                        _ =>
                        {
                            ret = format!("{} Failed!", option.name);
                        }
                    },
                    _ => unreachable!(),
                }
            }
            ret
        }

        "settings" =>
        {
            let mut ret = "Failed".to_string();
            let guild_id = command.guild_id.unwrap();
            for option in command.data.options.clone()
            {
                match option.kind
                {
                    CommandOptionType::SubCommand => match &*option.name
                    {
                        "set_log" =>
                        {
                            for opt in option.options
                            {
                                match &*opt.name
                                {
                                    "channel" =>
                                    {
                                        if let CommandDataOptionValue::Channel(c) =
                                            opt.resolved.unwrap()
                                        {
                                            ret = builtins::settings::Log::set_log(c, &guild_id)
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }
                        }
                        "remove_log" =>
                        {
                            let guild_id = command.guild_id.unwrap();
                            builtins::settings::Log::remove_log(&guild_id);
                            ret = "Removed logging channel".to_string()
                        }

                        "set_warn_behavior" =>
                        {
                            let (mut count, mut behavior) = (255_u8, "nothing".to_string());
                            for opt in option.options
                            {
                                match &*opt.name
                                {
                                    "behavior" =>
                                    {
                                        if let CommandDataOptionValue::String(s) =
                                            opt.resolved.unwrap()
                                        {
                                            behavior = s;
                                        }
                                    }

                                    "max" =>
                                    {
                                        if let CommandDataOptionValue::Integer(c) =
                                            opt.resolved.unwrap()
                                        {
                                            // Ensure sane numbers
                                            count = c.clamp(0, 255) as u8;
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                            }

                            let w = match behavior.to_lowercase().as_str()
                            {
                                "ban" => WarnBehavior::Ban(count),
                                "timeout" => WarnBehavior::Timeout {
                                    warning_count: count,
                                    duration: moderation::timeout::TimeoutTime {
                                        seconds: None,
                                        minutes: None,
                                        hours: None,
                                        days: Some(1),
                                    },
                                },
                                _ => WarnBehavior::Nothing,
                            };

                            ret = builtins::settings::set_warning_behavior(&guild_id, w);
                        }

                        _ =>
                        {
                            ret = format!("{} Failed!", option.name);
                        }
                    },
                    _ => unreachable!(),
                }
            }
            ret
        }

        _ => format!("Error: Unrecognized command {command_name}."),
    };

    if let Err(why) = command
        .create_interaction_response(&context.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content(content))
        })
        .await
    {
        error!("Cannot respond to slash command: {}", why);
    }
}
