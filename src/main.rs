use std::env;
use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use log::{error, info};
use serde::{Deserialize, Serialize};
use serenity::all::ChannelType::Text;
use serenity::all::{GatewayIntents, GuildId};
use serenity::Client;

struct AppState {
    client: Arc<Client>,
    server_id: u64,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN is empty");
    let server_id = env::var("DISCORD_SERVER_ID").expect("SERVER_ID is empty");
    let host = env::var("HTTP_SERVER_HOST").unwrap_or("0.0.0.0".to_string());
    let port = env::var("HTTP_SERVER_PORT").unwrap_or("8080".to_string());

    let instance = GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES;
    let client = Client::builder(&token, instance)
        .await
        .expect("Error creating client");

    let shared_state = Arc::new(AppState {
        client: Arc::new(client),
        server_id: server_id.parse::<u64>().unwrap(),
    });

    let app = Router::new()
        .route("/", post(index))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    info!("Listening on {}:{}", host, port);
}

#[derive(Deserialize)]
struct Request {
    channel_name: String,
    message: String,
}

#[derive(Serialize)]
struct SimpleResponse {
    status_code: u16,
    message: &'static str,
}

async fn index(
    State(state): State<Arc<AppState>>,
    Json(request): Json<Request>,
) -> impl IntoResponse {
    let client = state.client.clone();
    let guild_id = GuildId::new(state.server_id);
    let channels = client
        .http
        .get_channels(guild_id)
        .await
        .expect("Failed to get channels");

    let channels = channels
        .into_iter()
        .filter(|channel| channel.name == request.channel_name)
        .filter(|channel| channel.kind == Text)
        .collect::<Vec<_>>();

    if channels.len() == 0 {
        error!("No channel found");
        return (
            StatusCode::BAD_REQUEST,
            Json(SimpleResponse {
                status_code: StatusCode::BAD_REQUEST.as_u16(),
                message: "No channel found",
            }),
        );
    }

    if channels.len() > 1 {
        error!("Multiple channels found");
        return (
            StatusCode::BAD_REQUEST,
            Json(SimpleResponse {
                status_code: StatusCode::BAD_REQUEST.as_u16(),
                message: "Multiple channels found",
            }),
        );
    }

    channels[0]
        .clone()
        .say(&client.http, request.message)
        .await
        .expect("Failed to send message");

    return (
        StatusCode::OK,
        Json(SimpleResponse {
            status_code: StatusCode::OK.as_u16(),
            message: "Success",
        }),
    );
}
