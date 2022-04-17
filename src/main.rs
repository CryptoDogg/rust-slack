use dotenv;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let slack_webhook = dotenv::var("SLACK_WEBHOOK_URL").expect("SLACK_WEBHOOK_URL must be set");
    let response_body = reqwest::Client::new()
        .post(slack_webhook)
        .json(&serde_json::json!({
            "text": "Hello, World!",
        }))
        .send()
        .await?
        .text()
        .await?;

    println!("response body {:#?}", response_body);
    Ok(())
}
