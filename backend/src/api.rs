use std::path::PathBuf;
use crate::api_auth::ApiKey;
use crate::storage::Storage;
use crate::storage::meta::Meta;
use rocket::form::Form;
use rocket::fs::{NamedFile, TempFile};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{FromForm, State};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateArgs {
    pub root_name: String,
}

#[rocket::post("/create", data = "<args>")]
pub async fn create_new(
    args: Json<CreateArgs>,
    storage: &State<Storage>,
    _api_key: ApiKey,
) -> Json<Meta> {
    Json(storage.inner().allocate_new(&args.root_name).meta())
}

#[rocket::get("/<id>")]
pub async fn get_meta(id: String, storage: &State<Storage>) -> Json<Option<Meta>> {
    if let Some(context) = storage.inner().get_existing(&id) {
        Json(Some(context.meta()))
    } else {
        Json(None)
    }
}

#[derive(FromForm)]
pub(crate) struct FileUpload<'r> {
    file: TempFile<'r>,
    name: String
}

#[rocket::put("/<id>", data = "<args>")]
pub async fn upload_file(
    id: &str,
    mut args: Form<FileUpload<'_>>,
    storage: &State<Storage>,
    _api_key: ApiKey,
) -> Status {
    let Some(ctx) = storage.inner().get_existing(&id) else {
        return Status::NotFound;
    };

    let name = args.name.clone();
    ctx.add_file(&mut args.file, &name).await;

    Status::Ok
}

#[rocket::get("/<id>/<file_name..>", rank = 2)]
pub async fn download_file(
    id: &str,
    file_name: PathBuf,
    storage: &State<Storage>,
) -> Option<NamedFile> {
    let Some(ctx) = storage.inner().get_existing(&id) else {
        return None;
    };

    let file_path = ctx.path.join(file_name);
    if file_path.exists() {
        Some(NamedFile::open(file_path).await.ok()?)
    } else {
        None
    }
}

#[rocket::get("/<id>/bom", rank = 1)]
pub async fn download_bom(
    id: &str,
    storage: &State<Storage>,
    _api_key: ApiKey,
) -> Option<Json<Vec<String>>> {
    let Some(ctx) = storage.inner().get_existing(&id) else {
        return None;
    };

    Some(Json(ctx.list_files_recursively()))
}