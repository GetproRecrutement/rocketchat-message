# RocketChat Message for Rust
This library is an implementation of rocket chat hooks for messages

## Send text example

```rust
let client = RocketChat::new("ROCKET_CHAT_WEBHOOK_URL", "#channel");

client.send_text("Text").await?;
```

## Send message example

```rust
let client = RocketChat::new("ROCKET_CHAT_WEBHOOK_URL", "#channel");

let msg = RocketChatMessage::new()
    .set_text("Text")
    .set_attachments(vec![RocketChatAttachment::new()
        .set_title("Attachment title")
        .set_title_link("https://google.fr")
        .set_text("Attachment text")
        .set_author_name("Author name")
        .set_color("#c97149")]);

client.send_message(msg).await?;
```

## Send messages example

```rust
let client = RocketChat::new("ROCKET_CHAT_WEBHOOK_URL", "#channel");

let msgs = vec![
    RocketChatMessage::new().set_text("Message1"),
    RocketChatMessage::new().set_text("Message2"),
];

client.send_messages(msgs).await?;
```