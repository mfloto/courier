mod config;
mod discord;
mod email;

use crate::config::Config;
use crate::email::{parse_message_to_email, Email};
use imap::error::Result as ImapResult;

async fn courier(config: &Config) -> ImapResult<()> {
    // Create client and authenticated session
    let client = imap::ClientBuilder::new(&config.imap.server, config.imap.port).connect()?;
    let mut imap_session = client
        .login(&config.imap.username, &config.imap.password)
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

    // Check if mailing list address is in mail list id field (if a mailing list address is set in the config)
    let emails = emails
        .into_iter()
        .filter(|email| {
            if let Some(mailing_list) = &config.mailing_list {
                email
                    .list_id
                    .as_ref()
                    .unwrap()
                    .iter()
                    .any(|recipient| recipient.address == mailing_list.email)
            } else {
                true
            }
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
        match discord::send_message(&email, &config.discord).await {
            Ok(_) => println!("Sent email to Discord!"),
            Err(e) => println!("Could not send email to Discord: {}", e),
        }
    }

    // Logout and disconnect
    imap_session.logout()?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // Read config
    let config = Config::new();

    // Validate config
    match config.validate() {
        Ok(_) => println!("Config validated successfully!"),
        Err(e) => {
            println!("Config validation failed: {}", e);
            return;
        }
    }

    // Run courier on a loop
    loop {
        // TODO: logging
        // TODO: notification on error
        match courier(&config).await {
            Ok(_) => println!("Courier finished successfully!"),
            Err(e) => println!("Courier finished with an error: {}", e),
        }
        tokio::time::sleep(std::time::Duration::from_secs(config.imap.interval * 60)).await;
    }
}
