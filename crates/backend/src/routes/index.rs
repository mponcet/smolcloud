use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate<'a> {
    title: &'a str,
    message: &'a str,
}

pub async fn index() -> axum::response::Html<String> {
    let hello = HelloTemplate {
        title: "Hello World",
        message: "Hello World",
    };
    hello.render().unwrap().into()
}
