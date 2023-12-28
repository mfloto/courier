mod config;
mod discord;
mod email;

use crate::config::Config;
use crate::email::{parse_message_to_email, Email};
use imap::error::Result as ImapResult;

#[tokio::main]
async fn main() -> ImapResult<()> {
    // Read config
    let config = Config::new();

    // Create client and authenticated session
    let client = imap::ClientBuilder::new(config.imap.server, config.imap.port).connect()?;
    let mut imap_session = client
        .login(config.imap.username, config.imap.password)
        .map_err(|e| e.0)?;

    // Query messages from INBOX
    imap_session.select("INBOX")?;
    // Fetch unread messages only
    let message_ids = imap_session.search("UNSEEN")?;
    let message_ids = message_ids
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let messages = imap_session.fetch(message_ids, "(RFC822)")?;

    // Parse all messages to emails
    let emails: Vec<Email> = messages
        .iter()
        .map(|message| parse_message_to_email(message.body().unwrap().to_vec()).unwrap())
        .collect();

    // Check if mailing list address is in mail list id field
    let emails = emails
        .into_iter()
        .filter(|email| {
            email
                .list_id
                .as_ref()
                .unwrap()
                .iter()
                .any(|recipient| recipient.address == config.mailing_list.email)
        })
        .collect::<Vec<Email>>();

    // Check if SPF check is enabled and if so, check if the email passed the SPF check
    let emails = if config.imap.check_spf {
        emails
            .into_iter()
            .filter(|email| email.passed_spf)
            .collect::<Vec<Email>>()
    } else {
        emails
    };

    // Send all emails to Discord
    for email in emails {
        discord::send_message(&email, &config.discord)
            .await
            .expect("TODO: panic message");
    }
    // TODO: better error handling

    // Logout and disconnect
    imap_session.logout()?;
    Ok(())
}
