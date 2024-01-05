use axum::{
    Json,
    response::{Html, IntoResponse},
    http::{StatusCode, Uri, header::{self, HeaderMap, HeaderName}},
};
use std::path::PathBuf;
use std::io::{BufWriter, Cursor};
use image::ImageFormat;

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
    let img_path = PathBuf::from("assets/").join("a_logo.png");
    let image = image::io::Reader::open(&img_path).unwrap().decode().unwrap();
    let mut buffer = BufWriter::new(Cursor::new(Vec::new()));
    image.write_to(&mut buffer, ImageFormat::Png).unwrap();
    let bytes: Vec<u8> = buffer.into_inner().unwrap().into_inner();
    (
        axum::response::AppendHeaders([(header::CONTENT_TYPE, "image/png")]),
        bytes
        // image.into_bytes()
    )
}
// roadmap - show webcam on laptop screen.
