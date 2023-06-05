use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use anyhow::anyhow;
use anyhow::Result;
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
use shuttle_secrets::SecretStore;
use tracing::info;
use translate::{InputLang, OutputLang};
pub mod lang;
pub mod translate;

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
}

#[handler]
async fn api_docs() -> Json<serde_json::Value> {
    Json(
        serde_json::from_str(&String::from_utf8(SPEC_JSON.lock().unwrap().to_vec()).unwrap())
            .unwrap(),
    )
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
async fn poem(
    #[shuttle_static_folder::StaticFolder(folder = "public")] public_folder: std::path::PathBuf,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_poem::ShuttlePoem<impl poem::Endpoint> {
    let token = if let Some(token) = secret_store.get("XXX_TOKEN") {
        token
    } else {
        return Err(anyhow!("'TOKEN' was not found").into());
    };
    info!("token: {token}");

    let api_service = OpenApiService::new(Api, "Hello World", "1.0")
        .server("https://myqr.shuttleapp.rs/api")
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
            StaticFilesEndpoint::new(public_folder).index_file("index.html"),
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
            "/bili_video_info",
            get(get_bili_video_info).with(poem::middleware::Cors::new()),
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
        let cookie = "SESSDATA=ccaf6e65%2C1701286775%2C1e59a%2A62;";
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
