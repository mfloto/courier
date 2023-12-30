use crate::Email;
use serde::Serialize;

use reqwest::multipart::{Form, Part};

/// Discord webhook payload
#[derive(Debug, Serialize)]
struct DiscordPayload {
    json_payload: DiscordMessage,
}

/// Discord webhook content
#[derive(Debug, Serialize)]
struct DiscordMessage {
    username: String,
    embeds: Vec<DiscordEmbed>,
}

/// Discord webhook embed
#[derive(Debug, Serialize)]
struct DiscordEmbed {
    title: String,
    description: String,
    color: u32,
}

/// Send a message to a Discord webhook
pub(crate) async fn send_message(
    email: &Email,
    discord_config: &crate::config::DiscordConfigConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = reqwest::Client::builder().build()?;

    // Construct payload
    // TODO: Handle maximum length of message (Cut off message and add note to footer)
    let payload = DiscordMessage {
        username: email.from.clone(),
        embeds: vec![DiscordEmbed {
            title: email.subject.clone(),
            description: email.body.clone(),
            color: 0x000000,
        }],
    };

    // Create a multipart form
    let mut payload_form =
        Form::new().part("payload_json", Part::text(serde_json::to_string(&payload)?));

    let mut total_attachment_size = 0;

    // Include attachments if they exist
    if let Some(attachments) = email.attachments.as_ref() {
        for (index, attachment) in attachments.iter().enumerate() {
            // Discord only allows a limited number of attachments per message. This number can be set in the config file
            if index == discord_config.max_attachments as usize {
                // TODO: Add note to footer that not all attachments are included
                break;
            }

            // Discord only allows a specific size of attachments per message. This size can be set in the config file
            if total_attachment_size + attachment.contents.len()
                > discord_config.attachment_size_limit as usize * 1_000_000
            {
                // TODO: Add note to footer that not all attachments are included
                break;
            }

            total_attachment_size += attachment.contents.len();

            payload_form = payload_form.part(
                attachment.filename.clone(),
                Part::bytes(attachment.contents.to_vec()).file_name(attachment.filename.clone()),
            );
        }
    }

    // Send request
    let response = client
        .post(discord_config.webhook_url.clone())
        .multipart(payload_form)
        .send()
        .await?;

    println!("Response: {:}", response.text().await?);
    Ok(())
}
