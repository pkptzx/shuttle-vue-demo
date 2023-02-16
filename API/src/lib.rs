use poem::{get, handler, Route, endpoint::StaticFilesEndpoint};

#[handler]
fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_service::main]
async fn main(#[shuttle_static_folder::StaticFolder(folder = "public")] public_folder: std::path::PathBuf) -> shuttle_service::ShuttlePoem<impl poem::Endpoint> {
    let app = Route::new().nest(
        "/",
        StaticFilesEndpoint::new(public_folder).index_file("index.html"),
    ).at("/hello", get(hello_world));

    Ok(app)
}

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
