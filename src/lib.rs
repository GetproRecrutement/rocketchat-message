//! # RocketChat Message for Rust
//! This library is an implementation of rocket chat hooks for messages
//!
//! * Send text example
//!
//! ```
//! let client = RocketChat::new("ROCKET_CHAT_WEBHOOK_URL", "#channel");
//!
//! client.send_text("Text").await?;
//! ```
//!
//! * Send message example
//!
//! ```
//! let client = RocketChat::new("ROCKET_CHAT_WEBHOOK_URL", "#channel");
//!
//! let msg = RocketChatMessage::new()
//!     .set_text("Text")
//!     .set_attachments(vec![RocketChatAttachment::new()
//!         .set_title("Attachment title")
//!         .set_title_link("https://google.fr")
//!         .set_text("Attachment text")
//!         .set_author("Author name", None)
//!         .set_color("#c97149")]);
//!
//! client.send_message(msg).await?;
//! ```
//!
//! * Send messages example
//!
//! ```
//! let client = RocketChat::new("ROCKET_CHAT_WEBHOOK_URL", "#channel");
//!
//! let msgs = vec![
//!     RocketChatMessage::new().set_text("Message1"),
//!     RocketChatMessage::new().set_text("Message2"),
//! ];
//!
//! client.send_messages(msgs).await?;
//! ```

use anyhow::*;
use reqwest::blocking::Response;
use serde::Serialize;

/// A structure representing a rocket chat client
#[derive(Debug)]
pub struct RocketChat {
    /// Webhook url from rocket chat
    webhook_url: String,
    /// Channel used to send messages (@user or #channel)
    channel: String,
}

impl RocketChat {
    /// Creates a new rocket chat client
    ///
    /// ```
    /// let client = RocketChat::new("ROCKET_CHAT_WEBHOOK_URL", "#channel");
    /// ```
    pub fn new<S: Into<String>>(webhook_url: S, channel: S) -> Self {
        Self {
            webhook_url: webhook_url.into(),
            channel: channel.into(),
        }
    }

    /// Changes the channel to post messages
    ///
    /// ```
    /// let mut client = RocketChat::new("ROCKET_CHAT_WEBHOOK_URL", "#channel");
    ///
    /// client = client.set_channel("#channel2");
    /// ```
    pub fn set_channel<S: Into<String>>(mut self, channel: S) -> Self {
        self.channel = channel.into();
        self
    }

    /// Send simple text message
    ///
    /// ```
    /// let client = RocketChat::new("ROCKET_CHAT_WEBHOOK_URL", "#channel");
    ///
    /// client.send_text("Text").await?;
    /// ```
    pub async fn send_text<S: Into<String>>(&self, msg: S) -> Result<reqwest::Response, Error> {
        let msg = RocketChatMessage::new().set_text(msg.into());

        self.send_message(msg).await
    }

    /// Send simple text message (sync)
    ///
    /// ```
    /// let client = RocketChat::new("ROCKET_CHAT_WEBHOOK_URL", "#channel");
    ///
    /// client.send_text_sync("Text");
    /// ```
    pub fn send_text_sync<S: Into<String>>(&self, msg: S) -> Result<Response, Error> {
        let msg = RocketChatMessage::new().set_text(msg.into());

        self.send_message_sync(msg)
    }

    /// Send a rocket chat message
    ///
    /// ```
    /// let client = RocketChat::new("ROCKET_CHAT_WEBHOOK_URL", "#channel");
    /// let msg = RocketChatMessage::new().set_text("Text");
    ///
    /// client.send_message(msg).await;
    /// ```
    pub async fn send_message(&self, msg: RocketChatMessage) -> Result<reqwest::Response, Error> {
        let client = reqwest::Client::new();

        let msg = RocketChatMessagePayload::from((msg, self.channel.clone()));

        let res = client
            .post(&self.webhook_url)
            .json(&msg)
            .send()
            .await
            .map_err(|e| anyhow!("Request error: {:?}", e.status()))?;

        if res.status() == 200 {
            Ok(res)
        } else {
            Err(anyhow!("Response error: {}", res.status())) // Manage error if status is not 200
        }
    }

    /// Send a rocket chat message (sync)
    ///
    /// ```
    /// let client = RocketChat::new("ROCKET_CHAT_WEBHOOK_URL", "#channel");
    /// let msg = RocketChatMessage::new().set_text("Text");
    ///
    /// client.send_message_sync(msg);
    /// ```
    pub fn send_message_sync(&self, msg: RocketChatMessage) -> Result<Response, Error> {
        let client = reqwest::blocking::Client::new();

        let msg = RocketChatMessagePayload::from((msg, self.channel.clone()));

        let res = client
            .post(&self.webhook_url)
            .json(&msg)
            .send()
            .map_err(|e| anyhow!("Request error: {:?}", e.status()))?;

        if res.status() == 200 {
            Ok(res)
        } else {
            Err(anyhow!("Response error: {}", res.status())) // Manage error if status is not 200
        }
    }

    /// Send multiple messages at the same time on the same channel
    ///
    /// ```
    /// let client = RocketChat::new("ROCKET_CHAT_WEBHOOK_URL", "#channel");
    ///
    /// let msgs = vec![
    ///    RocketChatMessage::new().set_text("Text"),
    ///    RocketChatMessage::new().set_text("Text2"),
    /// ];
    ///
    /// client.send_messages(msgs).await?;
    /// ```
    pub async fn send_messages(&self, msgs: Vec<RocketChatMessage>) -> Result<(), Error> {
        for msg in msgs {
            self.send_message(msg).await?;
        }
        Ok(())
    }

    /// Send multiple messages at the same time on the same channel (sync)
    ///
    /// ```
    /// let client = RocketChat::new("ROCKET_CHAT_WEBHOOK_URL", "#channel");
    ///
    /// let msgs = vec![
    ///    RocketChatMessage::new().set_text("Text"),
    ///    RocketChatMessage::new().set_text("Text2"),
    /// ];
    ///
    /// client.send_messages_sync(msgs);
    /// ```
    pub fn send_messages_sync(&self, msgs: Vec<RocketChatMessage>) -> Result<(), Error> {
        for msg in msgs {
            self.send_message_sync(msg)?;
        }
        Ok(())
    }
}

/// A structure representing a rocket chat field for attachments
#[derive(Serialize, Default)]
pub struct Field {
    /// Size of field (default false by rocket chat)
    pub short: Option<bool>,
    /// Title of field
    pub title: String,
    /// Value of field
    pub value: String,
}

impl Field {
    /// Create new field
    ///
    /// ```
    /// let field = Field::new();
    /// ```
    pub fn new() -> Self {
        Field::default()
    }

    /// Change the title of the field
    ///
    /// ```
    /// let field = Field::new().set_title("Title");
    /// ```
    pub fn set_title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = title.into();
        self
    }

    /// Change the value of the field
    ///
    /// ```
    /// let field = Field::new().set_value("Value");
    /// ```
    pub fn set_value<S: Into<String>>(mut self, value: S) -> Self {
        self.value = value.into();
        self
    }

    /// Change the short of the field
    ///
    /// ```
    /// let field = Field::new().set_short(true);
    /// ```
    pub fn set_short(mut self, value: bool) -> Self {
        self.short = Some(value);
        self
    }
}

/// A structure representing a rocket chat attachment
#[derive(Serialize, Default)]
pub struct RocketChatAttachment {
    /// Title of attachment
    pub title: Option<String>,
    /// Link for title of attachment
    pub title_link: Option<String>,
    /// Color on border left of attachment
    pub color: Option<String>,
    /// Author name of attachment
    pub author_name: Option<String>,
    /// Author icon of attachment (displayed only if author name is defined)
    pub author_icon: Option<String>,
    /// Text of attachment
    pub text: Option<String>,
    /// Image of attachment
    pub image_url: Option<String>,
    /// Fields of attachment
    pub fields: Vec<Field>,
}

impl RocketChatAttachment {
    /// Create new attachment
    ///
    /// ```
    /// let attachment = RocketChatAttachment::new();
    /// ```
    pub fn new() -> Self {
        RocketChatAttachment::default()
    }

    /// Change the title of the attachment
    ///
    /// ```
    /// let attachment = RocketChatAttachment::new().set_title("Title");
    /// ```
    pub fn set_title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Change the title link of attachment
    ///
    /// ```
    /// let attachment = RocketChatAttachment::new().set_title_link("https://google.fr");
    /// ```
    pub fn set_title_link<S: Into<String>>(mut self, title_link: S) -> Self {
        self.title_link = Some(title_link.into());
        self
    }

    /// Change the color of attachment
    ///
    /// ```
    /// let attachment = RocketChatAttachment::new().set_color("#c97149");
    /// ```
    pub fn set_color<S: Into<String>>(mut self, color: S) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Change the author name & icon of attachment
    ///
    /// ```
    /// let attachment = RocketChatAttachment::new().set_author("Author Name", Some("ICON_URL"));
    /// ```
    pub fn set_author<S: Into<String>>(mut self, name: S, icon: Option<S>) -> Self {
        self.author_name = Some(name.into());
        if let Some(icon) = icon {
            self.author_icon = Some(icon.into());
        }
        self
    }

    /// Change the content of attachment
    ///
    /// ```
    /// let attachment = RocketChatAttachment::new().set_text("Text");
    /// ```
    pub fn set_text<S: Into<String>>(mut self, text: S) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Change the image of attachment
    ///
    /// ```
    /// let attachment = RocketChatAttachment::new().set_image("IMAGE_URL");
    /// ```
    pub fn set_image<S: Into<String>>(mut self, url: S) -> Self {
        self.image_url = Some(url.into());
        self
    }

    /// Change the fields of attachment
    ///
    /// ```
    /// let attachment = RocketChatAttachment::new().set_fields(vec![Field::new()
    ///     .set_title("Field title")
    ///     .set_value("Field value")
    ///     .set_short(true)]);
    /// ```
    pub fn set_fields(mut self, fields: Vec<Field>) -> Self {
        self.fields = fields;
        self
    }
}

#[derive(Serialize, Default)]
struct RocketChatMessagePayload {
    text: Option<String>,
    channel: Option<String>,
    attachments: Vec<RocketChatAttachment>,
}

impl From<(RocketChatMessage, String)> for RocketChatMessagePayload {
    fn from(message: (RocketChatMessage, String)) -> Self {
        Self {
            text: message.0.text,
            channel: Some(message.1),
            attachments: message.0.attachments,
        }
    }
}

/// A structure representing a rocket chat message
#[derive(Serialize, Default)]
// #[serde(rename_all = "camelCase")]
pub struct RocketChatMessage {
    /// Text on top of attachments
    pub text: Option<String>,
    /// Attachments linked to message
    pub attachments: Vec<RocketChatAttachment>,
}

impl RocketChatMessage {
    /// Create new message
    ///
    /// ```
    /// let message = RocketChatMessage::new();
    /// ```
    pub fn new() -> Self {
        RocketChatMessage::default()
    }

    /// Change the content of message
    ///
    /// ```
    /// let message = RocketChatMessage::new().set_text("Text");
    /// ```
    pub fn set_text<S: Into<String>>(mut self, text: S) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Change the attachments of message
    ///
    /// ```
    /// let attachments = vec![RocketChatAttachment::new().set_title("Title")]
    /// let message = RocketChatMessage::new().set_attachments(attachments);
    /// ```
    pub fn set_attachments(mut self, attachments: Vec<RocketChatAttachment>) -> Self {
        self.attachments = attachments;
        self
    }
}
