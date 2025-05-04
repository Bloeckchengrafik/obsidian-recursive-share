mod api;
pub mod api_auth;
pub mod config;
pub mod storage;
mod ui;

use crate::config::Config;
use rocket::data::{Limits, ToByteUnit};
use rocket::http::Method;
use rocket::{launch, routes};
use rocket_cors::{AllowedOrigins, CorsOptions};

#[launch]
async fn rocket() -> _ {
    let config = Config::new().await;
    let port = config.get_port().await;
    let storage = storage::Storage::new(&config).await;

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Put]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    std::fs::create_dir_all("tmp").unwrap();
    unsafe { std::env::set_var("TMPDIR", "tmp") };

    rocket::build()
        .manage(config)
        .manage(storage)
        .attach(cors.to_cors().unwrap())
        .configure(rocket::Config {
            port,
            limits: Limits::default()
                .limit("file", 1.gibibytes())
                .limit("data-form", 1.gibibytes()),
            ..Default::default()
        })
        .mount(
            "/api",
            routes![
                api::get_meta,
                api::create_new,
                api::upload_file,
                api::download_file,
                api::download_bom,
            ],
        )
        .mount("/view", routes![ui::show,])
}
