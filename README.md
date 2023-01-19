# LEB

The oddly named *Law Enforcement Bot* is an open-source, self-hostable,
dicord bot with moderation capabilites. The bot is written in rust for performance, and
stability.

The bot currently only support Linux.

## Compiling

d

## Running

To self-host this discord bot you first must [compile](#compiling) it.

After building an executable program the next step is configuring it.

### Configuring

The program, by default looks at `/etc/leb/config.toml` for it's configuration file.
The layout of this file is as follows:

```Toml
# /etc/leb/config.toml
[secrets]
token = "<your_discord_token>"

[resources]
restrictedWords = "/etc/leb/words.json" # Default: "/etc/leb/restricted_words.json"
guildSettings = "/var/local/leb/guild_settings.json" # Default: "/var/local/leb/guild_settings.json"
warnings = "/var/local/leb/warnings.json" # Default: "/var/local/leb/warnings.json"
```

`secrets` contains `token`, your discord application token. Next, `resources` contains
paths to where resources are stored. These aren't meant to be read by the end user and so
are JSON documents. Specifing any of these resources is optional as the default values are
usually sufficiant.

### Bot as a Service

For systems using *systemd* a service file is included with the source code of the program.
To make the bot run on system startup we can install this service file and enable it.

```sh
# In leb/tools
```
