use dotenv;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let echo_json: serde_json::Value = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&serde_json::json!({
            "title": "Reqwest.rs",
            "body": "https://docs.rs/reqwest",
            "userId": 1
        }))
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", echo_json);
    // let slack_webhook = dotenv::var("SLACK_WEBHOOK_URL").expect("SLACK_WEBHOOK_URL must be set");
    // let response_body = reqwest::Client::new()
    //     .post(slack_webhook)
    //     .json(&serde_json::json!({
    //         "text": "Hello, World!",
    //     }))
    //     .send()
    //     .await?
    //     .text()
    //     .await?;

    // println!("response body {:#?}", response_body);
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
