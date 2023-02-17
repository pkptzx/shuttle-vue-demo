use poem::{
    error::NotFoundError, get,post ,handler, http::StatusCode, listener::TcpListener, web::Html,
    endpoint::StaticFilesEndpoint,
    web::{Path, Json, Multipart}, EndpointExt, Response, Route, Request,
};
use anyhow::anyhow;
use shuttle_secrets::SecretStore;
use tracing::{error, info};
use poem_openapi::{param::Query, payload::PlainText, OpenApi, OpenApiService, Tags};

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
 

#[shuttle_service::main]
async fn main(#[shuttle_static_folder::StaticFolder(folder = "public")] public_folder: std::path::PathBuf,#[shuttle_secrets::Secrets] secret_store: SecretStore,) -> shuttle_service::ShuttlePoem<impl poem::Endpoint> {
    let token = if let Some(token) = secret_store.get("XXX_TOKEN") {
        token
    } else {
        return Err(anyhow!("'TOKEN' was not found").into());
    };
    info!("token: {token}");

    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("https://myqr.shuttleapp.rs/api");
    let ui = api_service.swagger_ui();

    let app = Route::new().nest(
        "/",
        StaticFilesEndpoint::new(public_folder).index_file("index.html"),
    )
    .nest("/api", api_service)
    .nest("/docs", ui)
    .at("/hello", get(hello_world))
    .catch_error(|_: NotFoundError| async move {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .content_type("text/html; charset=utf-8")
            .body("404 <span style=\"color:red\">NOT_FOUND</span>!")
    });

    Ok(app)
}

//https://discord.com/api/oauth2/authorize?client_id=1076143538858889288&permissions=8&scope=bot

// fn qr_decode() -> Result<(), Box<dyn std::error::Error>> {
//     // Load on image to search, convert it to grayscale
//     let img = image::open("tests/data/github.gif")?.to_luma8();
//     // Prepare for detection
//     let mut img = rqrr::PreparedImage::prepare(img);
//     // Search for grids, without decoding
//     let grids = img.detect_grids();
//     assert_eq!(grids.len(), 1);
//     // Decode the grid
//     let (meta, content) = grids[0].decode()?;
//     assert_eq!(meta.ecc_level, 0);
//     assert_eq!(content, "https://github.com/WanzenBug/rqrr");
//     Ok(())
// }
