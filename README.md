# Bot of Retribution

The oddly named *Bot of Retribution* is an open-source, self-hostable,
dicord bot with moderation capabilites. The bot is written in rust for performance, and
stability.

The bot currently only supports Linux environments.

## Features

* [x] Manual moderation
  * [x] kick
  * [x] ban
  * [x] warn
  * [x] timeout

* [x] Guild settings
  * [x] Store guild settings on disk

* [ ] Warning system
  * [x] Track warnings
    * [x] Store warnings on disk
  * [ ] Manage warnings
    * [x] Add warnings
    * [x] Remove warnings
    * [ ] Automation
      * [ ] Create new warnings
      * [ ] Remove old warnings
      * [x] Perform configured adminstrative action
* [ ] Logging
  * [ ] Logging restricted word matches
  * [ ] Logging admistriative action
    * [ ] Bans
    * [ ] Kicks
    * [ ] Warnings
    * [ ] Timeout
* [ ] Hosted web configuration interface

### Commmands

#### Moderation

* `moderation kick` -- Kick a member
* `moderation ban` -- Ban a member
* `moderation warn` -- Warn a member
* `moderation get_warnings` -- Get all warnings for a member
* `moderation remove_warnings` -- Remove all warnings for a member
* `moderation timeout` -- Timeout a member
* `moderation release` -- End a member's timeout

#### Configuration

* `settings set_log` -- Set the channel to log bot messages to
* `settings remove_log` -- Remove the logging channel from the bot (Disables logging)
* `settings set_warn_behavior` -- Set the behavior for when a set max number of warnings is met
* `settings set_wiki_limit` -- Set the maximum number of output characters for the `wiki` command

#### Other

* `meta` -- Information about the bot
* `wiki` -- Search something on wikipedia
* `define` -- Define an English word
* `random coin` -- Flip a coin
* `random roulette` -- Play Russian Roulette
* `conversions hours` -- Convert between 24 hour and 12 hour time
* `conversions temperature` -- Convert between Kelvin, Celsius, and Fahrenheit
* `conversions currency` -- Convert to and from various currencies

## Compiling

### Linux

First, clone the source code with git. Next move into the directory and compile with
`cargo`. If running Arch Linux (or a variant) use the [Arch](#pkgbuild-arch-recommended)
instructions instead.

```sh
git clone https://github.com/Sir-Bobert-II/BOR bor
cd bor
cargo build --release # Build program
install target/release/bor -Dm755 /usr/bin/bor # Install program
install -Dvm754 ./installation/bor.service /etc/systemd/system/bor.service # Install systemd service
```

#### PKGBUILD (Arch) *Recommended*

```sh
# Download PKGBUILD
curl -LO https://github.com/Sir-Bobert-II/BOR/raw/master/installation/PKGBUILD

# Build the package and install it, along with any dependencies
makepkg -si
```

### Build dependencies

* Cargo
* Git

## Running

To run this discord bot you must first [compile](#compiling) it.

After building an executable program the next step is configuring it.

### Configuring

The program, by default looks at `/etc/bor/config.toml` for it's configuration file.
The layout of this file is as follows:

```Toml
# /etc/bor/config.toml
[secrets]
token = "<your_discord_token>"

[resources]
restrictedWords = "/etc/bor/words.toml" # Default: "/etc/bor/restricted_words.toml"
guildSettings = "/var/local/bor/guild_settings.toml" # Default: "/var/local/bor/guild_settings.toml"
warnings = "/var/local/bor/warnings.toml" # Default: "/var/local/bor/warnings.toml"
```

`secrets` contains `token`, your discord application token. Next, `resources` contains
paths to where resources are stored. Specifing any of these resources is optional as the default values are
usually sufficiant.

### Bot as a Service

For systems using *systemd* a service file is included with the source code of the program.
To make the bot run on system startup we can enable the service.

```sh
sudo systemctl enable --now bor.service # Enable service
```

This enables and starts BOR. To read logs and see the status of BOR use systemd's `status`:

```sh
sudo systemctl status bor
```
