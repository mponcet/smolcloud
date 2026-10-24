use askama::Template;
use axum::http::header;
use axum::response::{Html, IntoResponse, Response};

const STYLE_CSS: &str = include_str!("../../static/css/style.css");

pub async fn style_css() -> Response {
    (
        [(header::CONTENT_TYPE, "text/css; charset=utf-8")],
        STYLE_CSS,
    )
        .into_response()
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

impl HomeTemplate {
    fn name(&self) -> &'static str {
        "home"
    }
}

pub async fn home() -> Html<String> {
    HomeTemplate {}.render().unwrap().into()
}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate;

impl AboutTemplate {
    fn name(&self) -> &'static str {
        "about"
    }
}

pub async fn about() -> Html<String> {
    AboutTemplate {}.render().unwrap().into()
}
