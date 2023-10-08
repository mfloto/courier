mod config;
mod discord;
mod email;

use crate::config::Config;
use crate::email::{parse_message_to_email, Email};
use imap::error::Result as ImapResult;
use serde::Deserialize;
use std::fs;

#[tokio::main]
async fn main() -> ImapResult<()> {
    // Read config
    let config = Config::new();

    // Create client and authenticated session
    let client = imap::ClientBuilder::new(config.imap.server, config.imap.port).rustls()?;
    let mut imap_session = client
        .login(config.imap.username, config.imap.password)
        .map_err(|e| e.0)?;

    // Query specific message from INBOX
    // TODO: Query all unread messages
    imap_session.select("INBOX")?;
    let messages = imap_session.fetch(60.to_string(), "(RFC822)")?;

    // Parse all messages to emails
    // TODO: Filter for emails addressed to the email list -> Check field
    let emails: Vec<Email> = messages
        .iter()
        .map(|message| parse_message_to_email(message.body().unwrap().to_vec()).unwrap())
        .collect();

    // Send all emails to Discord
    for email in emails {
        discord::send_message(&email, config.discord.webhook_url.clone())
            .await
            .expect("TODO: panic message");
    }
    // TODO: better error handling

    // Logout and disconnect
    imap_session.logout()?;
    Ok(())
}
