use dotenv;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let slack_webhook = dotenv::var("SLACK_WEBHOOK_URL").expect("SLACK_WEBHOOK_URL must be set");
    let response_body = reqwest::Client::new()
        .post(slack_webhook)
        .json(&serde_json::json!({
            "text": "Reqwest.rs",
        }))
        .send()
        .await?
        .text()
        .await?;

    println!("response body {:#?}", response_body);
    // Object(
    //     {
    //         "body": String(
    //             "https://docs.rs/reqwest"
    //         ),
    //         "id": Number(
    //             101
    //         ),
    //         "title": String(
    //             "Reqwest.rs"
    //         ),
    //         "userId": Number(
    //             1
    //         )
    //     }
    // )
    Ok(())
}
