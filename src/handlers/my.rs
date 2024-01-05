use axum::{
    Json,
    response::{Html, IntoResponse},
    http::{StatusCode, Uri, header::{self, HeaderMap, HeaderName}},
};

// this is a handler, a function that is used in a Router
// A handler is an async function that accepts zero or more “extractors” as arguments and returns something that can be converted into a response.
pub async fn say_hello() -> String {
    // integrate new functionality here:
   return "Hello!!!".to_string();
}

pub async fn html() -> Html<&'static str> {
    Html("<p>Hello, World!</p>")
}
// stream.
pub async fn get_image() -> impl axum::response::IntoResponse {
    //let img_path = PathBuf::from("<path/to/image>").join("ferris.png");
    //let image = image::io::Reader::open(&img_path).unwrap().decode().unwrap();

    (
        axum::response::AppendHeaders([(header::CONTENT_TYPE, "image/png")]),
        "Hello!!!".to_string()
        // image.into_bytes()
    )
}
// roadmap - show webcam on laptop screen.
