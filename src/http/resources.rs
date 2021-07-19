use rust_embed::RustEmbed;
use warp::{http::header::HeaderValue, path::Tail, reply::Response, Rejection, Reply};

#[derive(RustEmbed)]
#[folder = "gen/devtools/front_end"]
pub struct DevtoolsFrontendDir;

pub async fn serve_index_devtools() -> Result<impl Reply, Rejection> {
    serve_impl_devtools("index.html")
}

pub async fn serve_devtools(path: Tail) -> Result<impl Reply, Rejection> {
    serve_impl_devtools(path.as_str())
}

fn serve_impl_devtools(path: &str) -> Result<impl Reply, Rejection> {
    let asset = DevtoolsFrontendDir::get(path).ok_or_else(warp::reject::not_found)?;
    let mime = mime_guess::from_path(path).first_or_octet_stream();

    let mut res = Response::new(asset.into());
    res.headers_mut().insert(
        "content-type",
        HeaderValue::from_str(mime.as_ref()).unwrap(),
    );
    Ok(res)
}
