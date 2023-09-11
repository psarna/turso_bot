use anyhow::{Context, Result};
use spin_sdk::{
    config,
    http::{Request, Response},
    http_component,
};

#[http_component]
fn handle_turso_bot(req: Request) -> Result<Response> {
    println!("Request: {req:?}");
    let telegram_bot = config::get("telegram_bot_token")?;
    let api_endpoint = format!("https://api.telegram.org/bot{telegram_bot}/sendMessage");

    // TODO: handle /start and other commands

    let params: serde_json::Value =
        serde_json::from_slice(req.body().as_deref().unwrap_or_default())?;
    let message = params
        .get("message")
        .unwrap_or_else(|| params.get("edited_message"))
        .context("message not found")?;

    let chat = message.get("chat").context("chat field not found")?;
    let chat_id = chat.get("id").context("chat id field not found")?;
    let text = message.get("text").context("text field not found")?;
    let response = format!("Hey, Turso bot here, thanks for sending '{text}', much appreciated!");
    let response = urlencoding::encode(&response);

    let url = format!("{api_endpoint}?chat_id={chat_id}&text={response}");

    let api_req = http::Request::builder().uri(url).body(None)?;
    let _ = spin_sdk::outbound_http::send_request(api_req)?;

    Ok(http::Response::builder().status(200).body(None)?)
}
