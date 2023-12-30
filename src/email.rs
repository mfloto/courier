use html2text::from_read;
use mail_parser::PartType::{Html, Text};
use mail_parser::{Addr, Address, HeaderName, HeaderValue, MessageParser, MimeHeaders};
use std::borrow::Cow;

/// Email struct that contains all information that will be sent
#[derive(Debug)]
pub struct Email {
    pub(crate) to: Vec<Recipient>,
    pub(crate) from: String,
    pub(crate) subject: String,
    pub(crate) body: String,
    pub(crate) attachments: Option<Vec<Attachment>>,
    pub(crate) passed_spf: bool,
    pub(crate) list_id: Option<Vec<Recipient>>,
}

/// Attachment struct that contains the filename and the contents of the attachment
#[derive(Debug)]
pub struct Attachment {
    pub(crate) filename: String,
    pub(crate) contents: Vec<u8>,
}

/// Recipient struct that contains the name and the email address of the recipient
#[derive(Debug)]
pub struct Recipient {
    pub(crate) name: String,
    pub(crate) address: String,
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
            Html(message_body) => Some(from_read(message_body.as_bytes(), 140).into()),
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

    // Check if the message passed SPF
    let passed_spf = parsed_message
        .headers()
        .iter()
        .any(|header| match &header.name {
            HeaderName::Other(name) => {
                name == "Received-SPF" && header.value.as_text().unwrap().starts_with("Pass")
            }
            _ => false,
        });

    // Get all recipients
    let to: Vec<Recipient> = match parsed_message.to() {
        Some(Address::List(list)) => addresses_to_recipients(list),
        _ => Vec::new(),
    };

    // Get all list IDs this message was sent to (if any)
    let list_ids: Vec<Recipient> = parsed_message
        .headers()
        .iter()
        .filter_map(|header| {
            if let HeaderName::ListId = &header.name {
                if let HeaderValue::Address(addr) = &header.value {
                    if let Address::List(list) = addr {
                        let recipients: Vec<Recipient> = addresses_to_recipients(list);
                        if !recipients.is_empty() {
                            return Some(recipients);
                        }
                    }
                }
            }
            None
        })
        .flatten()
        .collect();

    Ok(Email {
        to,
        from,
        subject,
        body: body_text.to_string(),
        attachments: attachments.into(),
        passed_spf,
        list_id: Some(list_ids),
    })
}

/// Convert a list of addresses to a list of recipients
fn addresses_to_recipients(list: &Vec<Addr>) -> Vec<Recipient> {
    list.iter()
        .map(|addr| {
            let name = addr
                .name
                .clone()
                .unwrap_or_else(|| Cow::from(""))
                .to_string();
            let address = addr
                .address
                .clone()
                .unwrap_or_else(|| Cow::from(""))
                .to_string();

            Recipient { name, address }
        })
        .collect()
}
