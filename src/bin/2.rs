use std::error::Error;

use reqwest::{
    header::{HeaderMap, HeaderValue},
    ClientBuilder,
};
use rust_kimi::request::{KimiModel, RequestBody, Role, SingleMessage};
use serde_json::json;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //请求头
    let mut headers = HeaderMap::new();
    let key = "sk-Go4yGE6BswBsfrty5XlVxOW96p8RDXYyAmEZ0ImrlJmv1TVW";
    headers.insert("Content-Type", HeaderValue::from_str("application/json")?);
    headers.insert(
        "Authorization",
        HeaderValue::from_str(format!("Bearer {}", key).as_str())?,
    );

    let req_client = ClientBuilder::new().default_headers(headers).build()?;

    let req_body: RequestBody = RequestBody {
        model: KimiModel::MoonshotV1Auto,
        messages: vec![
            SingleMessage {
                role: Role::System,
                content: "你是人工智能助手".to_string(),
                partial: None,
                name: None,
            },
            SingleMessage {
                role: Role::User,
                content: "".to_string(),
                partial: None,
                name: None,
            },
        ],
        max_tokens: None,
        top_p: None,
        n: None,
        presence_penalty: None,
        frequency_penalty: None,
        response_format: None,
        temperature: None,
        stop: None,
        stream: None,
    };

    let mut tasks: JoinSet<()> = JoinSet::new();
    for _ in 0..1 {
        let client0 = req_client.clone();
        let req_body0 = req_body.clone();
        tasks.spawn(async move {
            let str = client0
                .post("https://api.moonshot.cn/v1/chat/completions")
                .json(&json!(req_body0))
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            println!("{}", str);
        });
    }
    tasks.join_all().await;

    Ok(())
}
