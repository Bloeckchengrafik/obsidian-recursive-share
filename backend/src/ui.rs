use crate::storage::Storage;
use rocket::State;
use rocket::http::ContentType;
use serde::Serialize;
use tera::Tera;

#[derive(Serialize)]
struct Context {
    title: String,
    raw: String,
    md: String,
}

fn render_template(cx: Context) -> String {
    Tera::one_off(
        include_str!("./ui/template.html"),
        &tera::Context::from_serialize(cx).unwrap(),
        true,
    )
    .unwrap()
}

fn render_md(source: &str) -> String {
    markdown::to_html_with_options(
        &source
            .replace("\n", "\n\n"),
        &markdown::Options::gfm(),
    ).unwrap_or(source.to_string())
}

#[rocket::get("/<id>")]
pub async fn show(id: &str, storage: &State<Storage>) -> (ContentType, Option<String>) {
    if let Some(context) = storage.inner().get_existing(&id) {
        (
            ContentType::HTML,
            Some(render_template(
                Context {
                    title: context.meta().title.clone(),
                    raw: context.get_content(),
                    md: render_md(&context.get_content()),
                },
            )),
        )
    } else {
        (ContentType::Plain, None)
    }
}
