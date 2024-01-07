#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![deny(unreachable_pub)]

//! efected-coto-emmory

pub mod docs;
pub mod error;
pub mod extract;
pub mod headers;
pub mod metrics;
pub mod middleware;
pub mod router;
pub mod routes;
pub mod settings;
pub mod tracer;
pub mod tracing_layers;

use axum::{
    Json,
    response::{Html, IntoResponse},
    http::{StatusCode, Uri, header::{self, HeaderMap, HeaderName}},
};
use std::path::PathBuf;
use std::io::{BufWriter, Cursor};
use image::ImageFormat;

/// Test utilities.
#[cfg(any(test, feature = "test_utils"))]
#[cfg_attr(docsrs, doc(cfg(feature = "test_utils")))]
pub mod test_utils;
/// Add two integers together.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
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

pub async fn html() -> Html<&'static str> {
    Html("<p>Hello, World!</p>")
}
// roadmap - show webcam on laptop screen.
