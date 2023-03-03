use std::collections::HashMap;

use anyhow::{Result, Ok};
use regex::Regex;

async fn get_bili_audio_info(bvid: &str) -> Result<String> {
    
    let client = reqwest::ClientBuilder::new().build().unwrap();
    let resp = client.get(format!("https://www.bilibili.com/video/{}/",bvid)).send().await.unwrap();
    
    let mut result: HashMap<&str, serde_json::Value> = HashMap::new();
    let rtxt = resp.text().await.unwrap();
    let re_subtitle = Regex::new(r#""subtitle_url":"(.+?)""#).unwrap();
    let mut urls = vec![];
    for cap in re_subtitle.captures_iter(&rtxt) {
        urls.push(cap[1].replace("\\u002F", "/"));
    }
    result.insert("subtitle", serde_json::Value::from(urls));

    let re_audio = Regex::new(r#""audio":\[\{"id":.+?,"baseUrl":"(.+?)","base_url":"(.+?)""#).unwrap();
    let mut urls = vec![];
    for cap in re_audio.captures_iter(&rtxt) {
        urls.push(cap[1].replace("\\u002F", "/"));
        urls.push(cap[2].replace("\\u002F", "/"));
    }
    result.insert("audio", serde_json::Value::from(urls));

    let re_ids = Regex::new(r#""aid":(\d+),"bvid":"([^"]+?)","cid":(\d+)"#).unwrap();
    let mut p = 0;
    for cap in re_ids.captures_iter(&rtxt) {
        println!("bvid{}: {}",p,cap[2].to_string());
        result.insert("aid" , serde_json::Value::from(cap[1].to_string()));
        result.insert("bvid", serde_json::Value::from(cap[2].to_string()));
        result.insert("cid", serde_json::Value::from(cap[3].to_string()));
        p=p+1;
    }
    println!("p: {}",p);
    
    let result = serde_json::to_string_pretty(&result)?;
    Ok(result)
}

#[tokio::main]
async fn main() -> Result<()> {
// 有字幕 BV1ov4y1t7Qj
// 无字幕 BV1js4y1j7vt
    let r = get_bili_audio_info("BV1aM4y1R7mL").await?;
    // let r = r.replace("\\u002F", "/");
    println!("r: {}",r);
    
Ok(())
}