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
pub async fn send_message(
    email: &Email,
    webhook_url: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // Craete a client
    let client = reqwest::Client::builder().build()?;

    // Construct payload
    let payload = DiscordMessage {
        username: email.from.clone(),
        embeds: vec![DiscordEmbed {
            title: email.subject.clone(),
            description: email.body.clone(),
            color: 0x00ff00,
        }],
    };

    // Create a multipart form
    let mut payload_form =
        Form::new().part("payload_json", Part::text(serde_json::to_string(&payload)?));

    // Include attachments if they exist
    if let Some(attachments) = email.attachments.as_ref() {
        for (index, attachment) in attachments.iter().enumerate() {
            // Discord only allows 10 attachments per message
            if index == 10 {
                break;
            }
            payload_form = payload_form.part(
                attachment.filename.clone(),
                Part::bytes(attachment.contents.to_vec()).file_name(attachment.filename.clone()),
            );
        }
    }

    // Send request
    let response = client
        .post(webhook_url)
        .multipart(payload_form)
        .send()
        .await
        .unwrap();

    println!("Response: {:}", response.text().await.unwrap());
    Ok(())
}
