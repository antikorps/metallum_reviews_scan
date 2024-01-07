use axum::response::Html;

pub async fn index() -> Html<&'static str> {
    return Html(include_str!("html/index.html"));
}
