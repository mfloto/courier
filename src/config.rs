use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub(crate) struct DiscordConfigConfig {
    pub(crate) webhook_url: String,
    pub(crate) attachment_size_limit: u8,
    pub(crate) max_attachments: u8,
}

#[derive(Deserialize)]
pub(crate) struct ImapConfigConfig {
    pub(crate) server: String,
    pub(crate) port: u16,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) interval: u64,
    pub(crate) check_spf: bool,
}

#[derive(Deserialize)]
pub(crate) struct MailingListConfig {
    //pub(crate) name: String,
    pub(crate) email: String,
}

#[derive(Deserialize)]
pub(crate) struct Config {
    pub(crate) discord: DiscordConfigConfig,
    pub(crate) imap: ImapConfigConfig,
    pub(crate) mailing_list: Option<MailingListConfig>,
}

impl Config {
    /// Create a new config object from the config.toml file
    pub(crate) fn new() -> Self {
        let config_contents =
            fs::read_to_string("config.toml").expect("Could not read config.toml!");
        toml::from_str(&config_contents).expect("Could not parse config.toml!")
    }

    /// Validate the configuration
    pub(crate) fn validate(&self) -> Result<(), String> {
        if self.imap.server.is_empty() {
            return Err("IMAP server cannot be empty!".into());
        }

        if self.imap.port == 0 {
            return Err("IMAP port cannot be 0!".into());
        }

        if self.imap.username.is_empty() {
            return Err("IMAP username cannot be empty!".into());
        }

        if self.imap.password.is_empty() {
            return Err("IMAP password cannot be empty!".into());
        }

        if self.imap.interval < 1 {
            return Err("IMAP interval cannot be smaller than 1 minute!".into());
        }

        if self.discord.webhook_url.is_empty() {
            return Err("Discord webhook URL cannot be empty!".into());
        }

        Ok(())
    }
}
