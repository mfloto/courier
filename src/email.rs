use mail_parser::PartType::Text;
use mail_parser::{MessageParser, MimeHeaders};
use std::borrow::Cow;

/// Email struct that contains all information that will be sent
#[derive(Debug)]
pub struct Email {
    pub(crate) from: String,
    pub(crate) subject: String,
    pub(crate) body: String,
    pub(crate) attachments: Option<Vec<Attachment>>,
}

/// Attachment struct that contains the filename and the contents of the attachment
#[derive(Debug)]
pub struct Attachment {
    pub(crate) filename: String,
    pub(crate) contents: Vec<u8>,
}

/// Parse a message to an email object
pub fn parse_message_to_email(message: Vec<u8>) -> Result<Email, &'static str> {
    let parsed_message = MessageParser::default().parse(&message).unwrap();

    // Collect all attachments
    let attachments: Vec<Attachment> = parsed_message
        .attachments()
        .cloned()
        .map(|attachment| Attachment {
            filename: attachment
                .attachment_name()
                .unwrap_or("filename".into())
                .to_string(),
            contents: attachment.contents().to_vec(),
        })
        .collect();

    println!("Text Body IDs: {:?}", parsed_message.text_body);

    // Concatenate all parts of the message that are text
    let body_text: Cow<'_, str> = parsed_message
        .text_body // Get Part IDs of all parts that are text
        .clone()
        .into_iter()
        .filter_map(|i| match parsed_message.part(i).unwrap().body.clone() {
            Text(message_body) => {
                // TODO: remove println
                println!("Found text body with ID {}:\n{}", i, message_body);
                Some(message_body)
            }
            _ => None,
        })
        .collect::<Vec<Cow<'_, str>>>()
        .join("")
        .into();

    // Get the from address
    let from = parsed_message
        .from()
        .and_then(|sender_list| sender_list.first())
        .and_then(|sender| Some(sender.address().unwrap_or("")))
        .unwrap_or("NO ADDRESS") // Handle missing from address
        .into();

    // Get the subject
    let subject = parsed_message.subject().unwrap_or("NO SUBJECT").to_string(); // Handle missing subject

    Ok(Email {
        from,
        subject,
        body: body_text.to_string(),
        attachments: attachments.into(),
    })
}
