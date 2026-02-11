use std::{env::var, sync::Arc, time::Duration};

use futures_util::{SinkExt, StreamExt};
use teloxide::{Bot, prelude::Requester, types::ChatId};
use tokio::{sync::mpsc, time::sleep};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        Message,
        client::IntoClientRequest,
        http::header::{ORIGIN, USER_AGENT},
    },
};

use crate::max::WsClient;

mod max;
mod models;

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum ServerEvent {
    Message(models::Request<models::message::ReceivedMessage>),
    Profile(models::Request<models::profile::Profile>),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Vars
    let token_env = var("MAX_TOKEN").expect("MAX_TOKEN environment variable not set");
    let max_chat_id_env: i64 = var("MAX_CHAT_ID")
        .expect("MAX_CHAT_ID environment variable not set")
        .parse()
        .expect("MAX_CHAT_ID environment variable is not a valid integer");
    let telegram_chat_id_env: i64 = var("TELEGRAM_CHAT_ID")
        .expect("TELEGRAM_CHAT_ID environment variable not set")
        .parse()
        .expect("TELEGRAM_CHAT_ID environment variable is not a valid integer");

    // Teloxide
    let bot = Bot::from_env();
    let target_chat = ChatId(telegram_chat_id_env);

    // TLS provider installation
    rustls::crypto::ring::default_provider()
        .install_default()
        .unwrap();

    let mut req = "wss://ws-api.oneme.ru/websocket"
        .into_client_request()
        .unwrap();

    req.headers_mut()
        .insert(ORIGIN, "https://web.max.ru".parse()?);

    req.headers_mut().insert(
        USER_AGENT,
        models::handshake::UserAgent::default()
            .header_user_agent
            .parse()?,
    );

    // Connect to WSS
    let (ws_stream, _) = connect_async(req).await?;
    let (mut write, mut read) = ws_stream.split();

    let (tx, mut rx) = mpsc::channel::<Message>(32);

    // Spawn a task to send messages from the channel to the WebSocket
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let _ = write.send(msg).await;
        }
    });

    let client = Arc::new(WsClient::new(tx));

    let mut user_id = None::<i64>;

    // Send initial events
    client.send_handshake().await?;
    client.send_auth(token_env).await?;

    // Spawn a task to support session activity
    tokio::spawn(ping_loop(client));

    // Handle incoming messages
    while let Some(Ok(msg)) = read.next().await {
        if let Message::Text(text) = msg {
            let text = text.as_str();

            match serde_json::from_str::<ServerEvent>(&text) {
                Ok(ServerEvent::Message(msg)) => {
                    let payload = msg.payload;

                    if let Some(user_id) = user_id {
                        if payload.message.sender.ne(&user_id)
                            && payload.chat_id.eq(&max_chat_id_env)
                        {
                            let _ = bot.send_message(target_chat, payload.message.text).await;
                        }
                    }
                }
                Ok(ServerEvent::Profile(profile)) => {
                    user_id = Some(profile.payload.profile.contact.id);
                }
                Err(_) => {}
            }
        }
    }

    Ok(())
}

async fn ping_loop(client: Arc<WsClient>) {
    loop {
        sleep(Duration::from_secs(30)).await;

        let _ = client
            .send_request(
                1,
                serde_json::json!({
                    "interactive": true
                }),
            )
            .await;
    }
}
