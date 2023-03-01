use std::sync::Mutex;
use std::{collections::HashMap, str::FromStr};

use anyhow::anyhow;
use anyhow::Result;
use poem::{
    endpoint::StaticFilesEndpoint,
    error::NotFoundError,
    get, handler,
    http::StatusCode,
    listener::TcpListener,
    post,
    web::Html,
    web::{Json, Multipart, Path},
    EndpointExt, Request, Response, Route,
};
use poem_openapi::types::ToJSON;
use poem_openapi::{param::Query, payload::PlainText, OpenApi, OpenApiService, Tags};
use reqwest::{Body, Method};
use serde::__private::de::IdentifierDeserializer;
use shuttle_secrets::SecretStore;
use tracing::{error, info};
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
        name: Query<Option<String>>,
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
   pub static ref spec_yaml:String = String::new();
   static ref spec_yaml2: Mutex<Vec<u8>> = Mutex::new(vec![]);
}

#[handler]
async fn api_docs() -> Json<serde_json::Value> {
    Json(serde_json::from_str(&String::from_utf8(spec_yaml2.lock().unwrap().to_vec()).unwrap()).unwrap())
}

#[handler]
async fn translation(poem::web::Query(params): poem::web::Query<HashMap<String,String>>) -> Json<translate::TranslateResult> {
    let txt = params.get("txt").expect("txt必传");
    let to = params.get("to");
    info!("txt: {} to: {:?}",txt,to);
    match to {
        Some(t)=>{
               Json(translate::translate(vec![txt.to_owned()], InputLang::Auto, t).await.unwrap())
        },
        None =>{
            Json(translate::translate(vec![txt.to_owned()], InputLang::Auto, OutputLang::SimplifiedChinese).await.unwrap())
        }
    }

    // let t = translate::translate_one_line(txt, InputLang::Auto, OutputLang::SimplifiedChinese).await;
    // let r =  match t {
    //     Ok(r1) => r1,
    //     Err(r2) => r2,
    // };
    // r
    // format!("txt: {} to: {}",txt,to)
}

#[shuttle_service::main]
async fn main(
    #[shuttle_static_folder::StaticFolder(folder = "public")] public_folder: std::path::PathBuf,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_service::ShuttlePoem<impl poem::Endpoint> {
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
    // println!("{}",api_service.spec_yaml());
    spec_yaml2
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
        .at("/v3/api-docs/swagger-config", get(api_docs).with(poem::middleware::Cors::new()))
        .at("/v2/api-docs", get(api_docs).with(poem::middleware::Cors::new()))
        .at("/hello", get(hello_world))
        .at("/upload", post(upload))
        .at("/translate", get(translation).with(poem::middleware::Cors::new()))
        .catch_error(|_: NotFoundError| async move {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .content_type("text/html; charset=utf-8")
                .body("404 <span style=\"color:red\">NOT_FOUND</span>!")
        });

    Ok(app)
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

