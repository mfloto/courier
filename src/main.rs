mod discord;
mod email;

use crate::email::{parse_message_to_email, Email};
use imap::error::Result as ImapResult;
use serde::Deserialize;
use std::fs;

/// Configuration
#[derive(Deserialize)]
struct Config {
    server: String,
    port: u16,
    username: String,
    password: String,
    discord_webhook: String,
}

#[tokio::main]
async fn main() -> ImapResult<()> {
    // Read the config file
    let config_contents =
        fs::read_to_string("config.toml").expect("Could not read the config.toml!");

    // Parse the config file
    let config: Config =
        toml::from_str(&config_contents).expect("Could not parse the config.toml!");

    // Create client and authenticated session
    let client = imap::ClientBuilder::new(config.server, config.port).rustls()?;
    let mut imap_session = client
        .login(config.username, config.password)
        .map_err(|e| e.0)?;

    // Query specific message from INBOX
    // TODO: Query all unread messages
    imap_session.select("INBOX")?;
    let messages = imap_session.fetch(60.to_string(), "(RFC822)")?;

    // Parse all messages to emails
    let emails: Vec<Email> = messages
        .iter()
        .map(|message| parse_message_to_email(message.body().unwrap().to_vec()).unwrap())
        .collect();

    // Send all emails to Discord
    for email in emails {
        discord::send_message(&email, config.discord_webhook.clone())
            .await
            .expect("TODO: panic message");
    }

    // Logout and disconnect
    imap_session.logout()?;
    Ok(())
}
