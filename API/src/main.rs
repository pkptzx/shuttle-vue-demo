use std::collections::HashMap;
use std::fs::File;
use std::sync::Arc;
use std::sync::Mutex;
use std::io::Write;
use std::process::{Command, Stdio};
#[cfg(target_os = "linux")]
use std::os::unix::prelude::PermissionsExt;

use anyhow::anyhow;
use anyhow::Result;
use poem::middleware::AddData;
use poem::{
    endpoint::StaticFilesEndpoint,
    error::NotFoundError,
    get, handler,
    http::StatusCode,
    post,
    web::{Json, Multipart},
    EndpointExt, Response, Route,
};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService, Tags};
use regex::Regex;
use reqwest;
use reqwest::cookie::Jar;
use reqwest::Url;
use shuttle_poem::ShuttlePoem;
use shuttle_runtime::SecretStore;
use tracing::{info,debug,error};
use translate::{InputLang, OutputLang};
pub mod lang;
pub mod translate;

pub mod xfai;
pub mod notion;

// #[derive(Clone)]
pub struct AppState {
}

#[handler]
fn hello_world() -> &'static str {
    "Hello, world!"
}

#[derive(Tags)]
enum MyTags {
    V1,
}

struct Api;

#[OpenApi]
impl Api {
    /// 这是一个接口说明
    #[oai(path = "/hello2", method = "get", tag = "MyTags::V1")]
    async fn index(
        &self,
        /// 这是一个参数说明
        name: poem_openapi::param::Query<Option<String>>,
    ) -> PlainText<String> {
        // 额 , name.clone() 返回的是 name.0 的副本
        let txt = name.clone().unwrap_or("None".to_string());
        info!("name: {txt}");

        match name.0 {
            Some(aname) => PlainText(format!("hello, {}!", aname)),
            None => PlainText("hello!".to_string()),
        }
    }
}

/// 上传图片
#[handler]
async fn upload(mut multipart: Multipart) -> Json<HashMap<String, String>> {
    info!("upload.....");
    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().map(ToString::to_string);
        if name.eq(&Some(String::from("name"))) {
            let val = field.text().await.unwrap();
            println!("name={:?}", val);
            continue;
        }

        let file_name = field.file_name().map(ToString::to_string);
        if let Ok(bytes) = field.bytes().await {
            println!(
                "name={:?} filename={:?} length={}",
                name,
                file_name,
                bytes.len()
            );
            let k = bytes.as_slice();
            let content = qr_decode(k);
            if let Ok(txt) = content {
                let txt2 = txt.clone();
                if txt.starts_with("http") {
                    let rtxt = qr_decode_by_url(&txt).await.unwrap();
                    let r = Json(HashMap::from([
                        ("result_origin".to_string(), txt2),
                        ("result".to_string(), rtxt),
                    ]));
                    return r;
                } else {
                    let r = Json(HashMap::from([("result".to_string(), txt2)]));
                    return r;
                }
            }
        }
    }
    Json(HashMap::from([(
        String::from("result"),
        String::from("未识别出二维码,请检查后重试!!!"),
    )]))
}

// lazy_static! {
//     static ref GLOBAL_STRING: RwLock<String> = RwLock::new("string".to_string());
// }

// fn main() {
//     {
//         let nice = GLOBAL_STRING.read().unwrap();
//         println!("{}", *nice);
//     }

//     {
//         let mut mystr = GLOBAL_STRING.write().unwrap();
//         *mystr = "assign new".to_string();
//     }

//     {
//         let nice = GLOBAL_STRING.read().unwrap();
//         println!("{}", *nice);
//     }
// }

lazy_static::lazy_static! {
    static ref SPEC_JSON: Mutex<Vec<u8>> = Mutex::new(vec![]);
    pub static ref CONFIG: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

#[handler]
async fn api_docs() -> Json<serde_json::Value> {
    Json(
        serde_json::from_str(&String::from_utf8(SPEC_JSON.lock().unwrap().to_vec()).unwrap())
            .unwrap(),
    )
}

#[handler]
async fn ai_translation(
    poem::web::Query(params): poem::web::Query<HashMap<String, String>>,
) -> Json<translate::TranslateResult> {
        let txt = params.get("txt").expect("txt必传");
    let to = params.get("to");
    info!("txt: {} to: {:?}", txt, to);
    match to {
        Some(t) => Json(
            translate::translate(vec![txt.to_owned()], InputLang::Auto, t)
                .await
                .unwrap(),
        ),
        None => Json(
            translate::translate(
                vec![txt.to_owned()],
                InputLang::Auto,
                OutputLang::SimplifiedChinese,
            )
            .await
            .unwrap(),
        ),
    }
}

#[handler]
async fn aitranslation(
    poem::web::Query(params): poem::web::Query<HashMap<String, String>>,
) -> Json<translate::TranslateResult> {
    todo!()
    // let txt = params.get("txt").expect("txt必传");
    // let to = params.get("to");
    // 先从缓存文件中读取,如果不存在就去ai,ai返回结果保存到文件?最好可视化管理
    //如果同步到其他地方管理,那么就需要先去查是否变更,再读缓存文件
}


#[handler]
async fn translation(
    poem::web::Query(params): poem::web::Query<HashMap<String, String>>,
) -> Json<translate::TranslateResult> {
    let txt = params.get("txt").expect("txt必传");
    let to = params.get("to");
    info!("txt: {} to: {:?}", txt, to);
    match to {
        Some(t) => Json(
            translate::translate(vec![txt.to_owned()], InputLang::Auto, t)
                .await
                .unwrap(),
        ),
        None => Json(
            translate::translate(
                vec![txt.to_owned()],
                InputLang::Auto,
                OutputLang::SimplifiedChinese,
            )
            .await
            .unwrap(),
        ),
    }

    // let t = translate::translate_one_line(txt, InputLang::Auto, OutputLang::SimplifiedChinese).await;
    // let r =  match t {
    //     Ok(r1) => r1,
    //     Err(r2) => r2,
    // };
    // r
    // format!("txt: {} to: {}",txt,to)
}

#[handler]
async fn get_bili_video_info(
    poem::web::Query(params): poem::web::Query<HashMap<String, String>>,
) -> Json<HashMap<&'static str, serde_json::Value>> {
    let bvid = params.get("bvid").expect("bvid必传");
    let client = reqwest::ClientBuilder::new().build().unwrap();
    let resp = client
        .get(format!("https://www.bilibili.com/video/{}/", bvid))
        .send()
        .await
        .unwrap();

    let mut result: HashMap<&'static str, serde_json::Value> = HashMap::new();
    let rtxt = resp.text().await.unwrap();
    let re_subtitle = Regex::new(r#""subtitle_url":"(.+?)""#).unwrap();
    let mut urls = vec![];
    for cap in re_subtitle.captures_iter(&rtxt) {
        urls.push(cap[1].replace("\\u002F", "/"));
    }
    result.insert("subtitle", serde_json::Value::from(urls));

    let re_video =
        Regex::new(r#""video":\[\{"id":.+?,"baseUrl":"(.+?)","base_url":"(.+?)""#).unwrap();
    let mut urls = vec![];
    for cap in re_video.captures_iter(&rtxt) {
        urls.push(cap[1].replace("\\u002F", "/"));
        urls.push(cap[2].replace("\\u002F", "/"));
    }
    result.insert("video", serde_json::Value::from(urls));

    let re_audio =
        Regex::new(r#""audio":\[\{"id":.+?,"baseUrl":"(.+?)","base_url":"(.+?)""#).unwrap();
    let mut urls = vec![];
    for cap in re_audio.captures_iter(&rtxt) {
        urls.push(cap[1].replace("\\u002F", "/"));
        urls.push(cap[2].replace("\\u002F", "/"));
    }
    result.insert("audio", serde_json::Value::from(urls));

    let re_ids = Regex::new(r#""aid":(\d+),"bvid":"([^"]+?)","cid":(\d+)"#).unwrap();
    for cap in re_ids.captures_iter(&rtxt) {
        result.insert("aid", serde_json::Value::from(cap[1].to_string()));
        result.insert("bvid", serde_json::Value::from(cap[2].to_string()));
        result.insert("cid", serde_json::Value::from(cap[3].to_string()));
    }
    Json(result)
}


#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> ShuttlePoem<impl poem::Endpoint> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }

    info!("{:?}",std::env::current_dir().unwrap());
    info!("{:?}",std::env::current_exe().unwrap());

    // let conn = dbclient.connect().unwrap();
    // let state = Arc::new(AppState {
    //     db_client:Mutex::new(dbclient),
    // });
        // let db = Arc::new(dbclient);
    // #[cfg(target_os = "linux")]
    // {
    //     info!("准备设置bito权限");
    //     let dir = std::env::current_dir().unwrap();
    //     let file = File::open(dir.join("bito"))?;
    //     let mut perms = file.metadata()?.permissions();
    //     perms.set_mode(0x777);
    //     file.set_permissions(perms)?;
    //     info!("设置bito权限完毕");
    // }
    let token = if let Some(token) = secret_store.get("XXX_TOKEN") {
        token
    } else {
        return Err(anyhow!("'TOKEN' was not found").into());
    };
    info!("token: {token}");

    let ai_access_key = if let Some(token) = secret_store.get("AI_ACCESS_KEY") {
        token
    } else {
        return Err(anyhow!("'ai_access_key' was not found").into());
    };
    info!("ai_access_key: {ai_access_key}");
    CONFIG.lock().unwrap().insert("AI_ACCESS_KEY".to_string(), ai_access_key);
    CONFIG.lock().unwrap().insert("XF_QWEN_API_KEY".to_string(), secret_store.get("AI_ACCESS_KEY").unwrap_or_default());
    CONFIG.lock().unwrap().insert("XF_QWEN_TRANSLATE_PROMPT".to_string(), secret_store.get("AI_ACCESS_KEY").unwrap_or_default());


    let api_service = OpenApiService::new(Api, "Hello World", "1.0")
        .server("/api")
        .server("http://127.0.0.1:8000/api");
    let ui = api_service.swagger_ui();
    // println!("{}",api_service.SPEC_YAMLl());
    SPEC_JSON
        .lock()
        .unwrap()
        .append(api_service.spec().into_bytes().as_mut());

    let app = Route::new()
        .nest(
            "/",
            StaticFilesEndpoint::new("public").index_file("index.html"),
        )
        .nest("/api", api_service.with(poem::middleware::Cors::new()))
        .nest("/docs", ui)
        .at(
            "/v3/api-docs/swagger-config",
            get(api_docs).with(poem::middleware::Cors::new()),
        )
        .at(
            "/v2/api-docs",
            get(api_docs).with(poem::middleware::Cors::new()),
        )
        .at("/hello", get(hello_world))
        .at("/upload", post(upload))
        .at(
            "/translate",
            get(translation).with(poem::middleware::Cors::new()),
        )
        .at(
            "/aitranslate",
            post(aitranslation).with(poem::middleware::Cors::new()),
        )
        .at(
            "/bili_video_info",
            get(get_bili_video_info).with(poem::middleware::Cors::new()),
        )
        // .at(
        //     "/ai",
        //     get(ai).with(poem::middleware::Cors::new()),
        // )
        .at(
            "/wechatbot",
            post(wechatbot).with(poem::middleware::Cors::new()),
        )
        .at(
            "/bili_video_url",
            get(get_bili_video_url).with(poem::middleware::Cors::new()),
        )
        .catch_error(|_: NotFoundError| async move {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .content_type("text/html; charset=utf-8")
                .body("404 <span style=\"color:red\">NOT_FOUND</span>!")
        });

    Ok(app.into())
}

/**
 * 解析二维码
 */
fn qr_decode(buffer: &[u8]) -> Result<String> {
    // Load on image to search, convert it to grayscale
    let img = image::load_from_memory(buffer)?.to_luma8();
    // let img = image::open("tests/data/github.gif")?.to_luma8();
    // Prepare for detection
    let mut img = rqrr::PreparedImage::prepare(img);
    // Search for grids, without decoding
    let grids = img.detect_grids();
    // assert_eq!(grids.len(), 1);
    if grids.len() == 0 {
        return Ok(String::from("未识别出二维码,请检查后重试!!!"));
    }
    // Decode the grid
    let (meta, content) = grids[0].decode()?;
    println!("{:?}", grids[0].bounds);
    println!("{:?}", meta);
    // assert_eq!(meta.ecc_level, 0);
    println!("{content}");
    Ok(content)
}

async fn qr_decode_by_url(qrcode_url: &str) -> Result<String> {
    let client = reqwest::ClientBuilder::new().build().unwrap();
    let resp = client.get(qrcode_url).send().await.unwrap();
    println!("{:?}", resp.headers());
    let content_type = resp.headers().get("content-type");
    if let Some(c_type) = content_type {
        if c_type.to_str().unwrap().starts_with("image") {
            let bs = resp.bytes().await.unwrap();
            let content = qr_decode(&bs[..]).unwrap();
            return Ok(content);
        } else {
            let rtxt = resp.text().await.unwrap();
            return Ok(rtxt);
        }
    }
    Ok(String::new())
}

#[handler]
async fn get_bili_video_url(
    poem::web::Query(params): poem::web::Query<HashMap<String, String>>,
) -> String {
    let bvid = params.get("bvid").expect("bvid必传");
    let client = reqwest::ClientBuilder::new().build().unwrap();
    let resp = client
        .get(format!(
            "https://api.bilibili.com/x/player/pagelist?bvid={bvid}"
        ))
        .send()
        .await
        .unwrap();
    println!("{:?}", resp.headers());
    let bs = resp.json::<serde_json::Value>().await.unwrap();
    println!("pagelist: {bs}");
    let success = &bs["code"].as_i64().unwrap();
    if *success == 0i64 {
        let cid = &bs["data"][0]["cid"];
        println!("{}", cid.as_i64().unwrap());
        let cid = cid.as_i64().unwrap().to_string();
        // 获取视频地址
        let url = format!("https://api.bilibili.com/x/player/playurl?bvid={bvid}&cid={cid}&qn=80&fnval=1&fnver=0&fourk=1&high_quality=1&platform=html5").parse::<Url>().unwrap();
        let cookie = "SESSDATA=d961ea05%2C1737035628%2C99761%2A72CjBi4nE0hMm5m5FRUzgLMwdTL7RhkvGVhrVGenJrQGsSwpdhSd-T9SvVJtY4S9rbwYcSVnBBTnU2a0sxeU9iWHY2VHFfT1lXQjgwZGNyLXhoX2JVamxod1MxY3d5dlg2R05SN0hMRWxFNDBDQ2Fzd3RUR0NRSHF5SFlQS2NXeVpQaldRV3ZlZXRBIIEC;";
        let jar = Jar::default();
        jar.add_cookie_str(cookie, &url);
        let client = reqwest::ClientBuilder::new()
            .cookie_provider(Arc::new(jar))
            .build()
            .unwrap();
        let resp = client.get(url).send().await.unwrap();
        let rjson = resp.json::<serde_json::Value>().await.unwrap();
        let success = &bs["code"].as_i64().unwrap();
        if *success == 0i64 {
            println!("playurl: {rjson}");
            let playurl = &rjson["data"]["durl"][0]["url"]
                .as_str()
                .unwrap()
                .to_string();
            return playurl.to_string();
        } else {
            return "not found playurl, please check bvid!".to_string();
        }
    } else {
        return "not found video info, please check bvid!".to_string();
    }
}
#[handler]
async fn wechatbot(mut params: Multipart) -> Json<serde_json::Value> {
    info!("收到消息:");
    let mut params_map = HashMap::new();
    while let Ok(Some(field)) = params.next_field().await {
        let field_name = field.name().map(ToString::to_string);
        if let Some(f_name) = field_name {
            let f_value = field.text().await.unwrap();
            info!("{}: {}",f_name,f_value);
            params_map.insert(f_name, f_value);
        }
    }
    let p_type = params_map.get("type").unwrap();
    let p_is_system_event = params_map.get("isSystemEvent").unwrap();
    let p_content = params_map.get("content").unwrap();
    let p_source: serde_json::Value = serde_json::from_str(params_map.get("source").unwrap()).unwrap();
    if p_type.eq("text") && p_is_system_event.eq("0"){
        //info!("内容字节: {:?}",p_content.as_bytes());
        //只回复@我的
        if p_content.contains("@💻 ")||p_content.contains("@ ")||p_content.contains("@💻 ") {
            let p_content = p_content.replace("@💻 ", "").replace("@ ", "").replace("@💻 ", "");
            let topic = p_source["room"]["payload"]["topic"].as_str().unwrap().to_string();
            let to = p_source["from"]["payload"]["name"].as_str().unwrap().to_string();
            if p_content.trim().is_empty(){
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert("Content-Type", "application/json".parse().unwrap());
                let data = serde_json::json!({
                    "to": topic,//b友在微信
                    "isRoom": true,
                    "type": "text",
                    "content": format!("@{} {}",to,"有什么问题吗?你倒是问啊"),
                });
                info!("回复: {}",data);
                let client = reqwest::ClientBuilder::new().build().unwrap();
                let resp = client
                    .post("http://43.139.117.89:3001/webhook/msg")
                    .headers(headers)
                    .json(&data)
                    .send()
                    .await
                    .unwrap();
                info!("resp header: {:?}", resp.headers());
                let resp_json = resp.json::<serde_json::Value>().await.unwrap();
                info!("resp: {resp_json}");
            }else{
                tokio::spawn(async move {
                    ai_reply(&topic,&to,&p_content).await;
                });
            }
        }
    }

    Json(serde_json::json!({
        "to": "测试机器人",//b友在微信
        "isRoom": true,
        "type": "text",
        "content": "收到",
    }))
}
#[handler]
async fn ai(
    poem::web::Query(params): poem::web::Query<HashMap<String, String>>,
) -> String {
    let msg = params.get("msg").expect("msg必传");
    info!("msg: {msg}");
    exec_bito(msg.to_owned()).await
}

async fn exec_bito(msg:String) -> String {
    let m = CONFIG.lock().unwrap();
    let token = m.get("AI_ACCESS_KEY").unwrap();
    info!("read token:{}",token);
    let token = "eyJhbGciOiJIUzI1NiJ9.eyJkYXRhIjoidjFfMTYyOV8yMjY2NjFfOTM3MV9UdWUgT2N0IDEwIDE0OjQ0OjMzIFVUQyAyMDIzIn0.7eTp0JzrCPOulIROzadOJrrgwEa1vkQQGXGVWioYtL0";
    info!("{:?}",std::env::current_dir().unwrap());
    info!("{:?}",std::env::current_exe().unwrap());
    let dir = std::env::current_dir().unwrap();
    
    #[cfg(target_os = "windows")]
    let mut bito = std::process::Command::new(dir.join("bito.exe"))
        .arg("-k")
        .arg(token)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

        #[cfg(target_os = "linux")]
        let mut bito = std::process::Command::new(dir.join("bito"))
            .arg("-k")
            .arg(token)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute command");

    let mut stdin = bito.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(msg.as_bytes())
            .expect("Failed to write to stdin");
    });

    let output = bito.wait_with_output().expect("Failed to read stdout");
    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        info!(
            "Command executed successfully: {} ",
            result
        );
        result.to_string()
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        error!("Command failed: {}", error);
        error.to_string()
    }
}

async fn ai_reply(topic:&str,to:&str,p_content:&str){
    info!("准备进行ai回复...");
    let mut answer = exec_bito(p_content.to_string()).await;
    answer = answer.trim_end().to_string();
    if answer.trim().is_empty() {
        answer = "也许你的问题被和谐了,请尝试换个问题或换个问法".to_string();
    }
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let data = serde_json::json!({
        "to": topic,//b友在微信
        "isRoom": true,
        "type": "text",
        "content": format!("@{} {}",to,answer),
    });
    info!("回复: {}",data);
    let client = reqwest::ClientBuilder::new().build().unwrap();
    let resp = client
        .post("http://43.139.117.89:3001/webhook/msg")
        .headers(headers)
        .json(&data)
        .send()
        .await
        .unwrap();
    info!("resp header: {:?}", resp.headers());
    let resp_json = resp.json::<serde_json::Value>().await.unwrap();
    info!("resp: {resp_json}");
}

#[test]
fn test_exec_bito() {
    // 创建一个运行时环境
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    // positive test case
    let result = rt.block_on(exec_bito("hello".to_string()));
    println!("result:{result}");
    assert_eq!(result, "Command executed successfully: ...");

    // negative test case
    let result = rt.block_on(exec_bito("你好".to_string()));
    assert_eq!(result, "Command failed: ...");

}

#[test]
fn test_exec_echo(){
    let token = "eyJhbGciOiJIUzI1NiJ9.eyJkYXRhIjoidjFfMTYyOV8yMjY2NjFfOTM3MV9UdWUgT2N0IDEwIDE0OjQ0OjMzIFVUQyAyMDIzIn0.7eTp0JzrCPOulIROzadOJrrgwEa1vkQQGXGVWioYtL0";
    let output = if cfg!(windows) {
        let msg = "什么是节流";
        // let msg = utf8_to_gbk(msg);
        // println!("msg2: {msg2}");
        Command::new("cmd")
        .arg("/C")
        .arg("echo")
        .arg(msg)
        .arg("|")
        .arg("bito.exe")
        .arg("-k")
        .arg(token)
        .output()
        .expect("Failed to execute command")
    }else{
        Command::new("echo")
        .arg("你好!")
        .arg("|")
        .arg("bito.exe")
        .arg("-k")
        .arg(token)
        .output()
        .expect("Failed to execute command")
    };

    if output.status.success() {
        // let result = String::from_utf8_lossy(&output.stdout);
        let result = gbk_to_utf8_by_bytes(&output.stdout);
        println!("Command executed successfully: {}", result);
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        println!("Command failed: {}", error);
    }
}

#[test]
fn test_exec_echo2(){
    let token = "eyJhbGciOiJIUzI1NiJ9.eyJkYXRhIjoidjFfMTYyOV8yMjY2NjFfOTM3MV9UdWUgT2N0IDEwIDE0OjQ0OjMzIFVUQyAyMDIzIn0.7eTp0JzrCPOulIROzadOJrrgwEa1vkQQGXGVWioYtL0";
     
    let mut bito = std::process::Command::new("bito")
        .arg("-k")
        .arg(token)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");
    let arg1 = "rust实现群聊";
    let mut stdin = bito.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(arg1.as_bytes())
            .expect("Failed to write to stdin");
    });

    let output = bito.wait_with_output().expect("Failed to read stdout");
    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        println!(
            "Command executed successfully: ---------{} ----------",
            result
        );
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        println!("Command failed: {}", error);
    }
}

#[test]
fn test_encoding(){
    // UTF-8 to GBK
    let msg = utf8_to_gbk("你好啊!nihao");
    println!("msg:{msg}");
    let msg = gbk_to_utf8(&msg);
    println!("msg:{msg}");

    let (gbk_bytes,_,_) = encoding_rs::GBK.encode("你好");
    println!("gbk:{}",String::from_utf8_lossy(&gbk_bytes));
    let (utf8_bytes,_,_) = encoding_rs::GBK.decode(&gbk_bytes);
    println!("utf8:{}",utf8_bytes.to_string());

    // GBK to UTF-8
    // let gbk_string = "你好";
    // let (utf8_bytes,_,_) = encoding_rs::GBK.decode(gbk_string.as_bytes());
    // let utf8_string = String::from_utf8_lossy(&utf8_bytes);
    // println!("UTF-8 decoded string: {}", utf8_string);
}
fn utf8_to_gbk(s:&str)->String{
    // let utf8_string = "你好";
    let (gbk_bytes,_,_) = encoding_rs::GBK.encode(&s);
    let gbk_string = String::from_utf8_lossy(&gbk_bytes);
    return gbk_string.to_string();
}
fn gbk_to_utf8(s:&str)->String{
    let (utf8_bytes,_,_) = encoding_rs::GBK.decode(s.as_bytes());
    println!("{utf8_bytes}");
    // let utf8_string = String::from_utf8_lossy(&utf8_bytes);
    // println!("UTF-8 decoded string: {}", utf8_string);
    // return utf8_string.to_string();
    return utf8_bytes.to_string();
}
fn gbk_to_utf8_by_bytes(s:&[u8])->String{
    let (utf8_bytes,_,_) = encoding_rs::GBK.decode(s);
    println!("{utf8_bytes}");
    // let utf8_string = String::from_utf8_lossy(&utf8_bytes);
    // println!("UTF-8 decoded string: {}", utf8_string);
    // return utf8_string.to_string();
    return utf8_bytes.to_string();
}