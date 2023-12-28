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
    pub(crate) mailing_list: MailingListConfig,
}

impl Config {
    pub(crate) fn new() -> Self {
        let config_contents =
            fs::read_to_string("config.toml").expect("Could not read config.toml!");
        toml::from_str(&config_contents).expect("Could not parse config.toml!")
    }
}
