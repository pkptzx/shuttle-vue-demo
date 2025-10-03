use reqwest::StatusCode;
use serde::Deserialize;
use siumai::models;
use siumai::prelude::*;
use tracing::info;

async fn xfai_qwen3(sys_msg: &str, user_msg: &str) -> Result<(), Box<dyn std::error::Error>> {
    let openai_client = Provider::openai()
        .enable_tracing()
        .base_url("https://maas-api.cn-huabei-1.xf-yun.com/v1")
        .api_key("sk-KQQuo9MlSxCAvmuR22F978D25c6d4018Ab98Ba1aAc870e67")
        .model("xop3qwen1b7")
        .temperature(0.7)
        .max_tokens(8192)
        .build()
        .await
        .unwrap();

    // You can now call both standard and OpenAI-specific methods
    let response = openai_client
        .chat(vec![system!(sys_msg), user!(user_msg)])
        .await
        .unwrap();
    // let assistant = openai_client.create_assistant(...).await?; // Example of specific feature

    println!("OpenAI says: {}", response.text().unwrap_or_default());
    Ok(())
}

#[tokio::test]
async fn test_xfai_qwen3() {
    let client = reqwest::ClientBuilder::new().build().unwrap();
    let resp = client
        .get("https://tonihub.com/sonsdumps/items.json")
        .send()
        .await
        .unwrap();
    let r_json = resp.json::<serde_json::Value>().await.unwrap();
    let arr = r_json.as_array().unwrap();
    let mut content = String::new();
    let mut count = 0;

    for item in arr.iter() {
        let itm = item["_name"].as_str().unwrap();
        content.push_str(itm);
        content.push('\n');
        count = count + 1;
        if count % 10 == 0 {
            println!("OpenAI...{count}");
            let _resp = xfai_qwen3(
                r#"你是游戏本地化专家，严格遵守：1)逐行翻译 2)输出格式示例:{"apple":"苹果","banana":"香蕉"}"#,
                format!("翻译为简体中文:\n{}", content).as_str(),
            )
            .await
            .unwrap();
            println!("OpenAI end");
            // println!("{content}");
            content.clear();
        }
    }
    if !content.is_empty() {
        println!("OpenAI...{count}");
        let _resp = xfai_qwen3(
            r#"你是游戏本地化专家，严格遵守：1)逐行翻译 2)输出格式示例:{"apple":"苹果","banana":"香蕉"}"#,
            format!("翻译为简体中文:\n{}", content).as_str(),
        )
        .await
        .unwrap();
        println!("OpenAI end");
        // println!("{content}");
    }
}

#[tokio::test]
async fn test_xfai_qwen3_single() {
    let _resp = xfai_qwen3(
        r#"你是游戏本地化专家，严格遵守：1)逐行翻译 2)输出格式示例:{"apple":"苹果","banana":"香蕉"}"#,
        format!("翻译为简体中文:\nYarro\nYarroSeed\nZiplineRope\n").as_str(),
    )
    .await
    .unwrap();
    println!("OpenAI end");
    // println!("{content}");
}
