use activitypub_federation::{
    axum::json::FederationJson,
    config::Data,
    fetch::webfinger::{Webfinger, build_webfinger_response, extract_webfinger_name},
    protocol::context::WithContext,
    traits::Object,
};
use axum::{
    Json, debug_handler,
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;

use nougat::{database::DatabaseHandle, objects::Person, utils::Error};

#[derive(Deserialize)]
pub struct WebfingerQuery {
    resource: String,
}

#[debug_handler]
pub async fn webfinger(
    Query(query): Query<WebfingerQuery>,
    data: Data<DatabaseHandle>,
) -> Result<Json<Webfinger>, Error> {
    let name = extract_webfinger_name(&query.resource, &data)?;
    let db_user = data.read_user(name)?;
    Ok(Json(build_webfinger_response(
        query.resource,
        db_user.ap_id.into_inner(),
    )))
}

#[debug_handler]
pub async fn http_get_user(
    Path(name): Path<String>,
    data: Data<DatabaseHandle>,
) -> Result<FederationJson<WithContext<Person>>, Error> {
    println!("Was requested: {}", &name);
    let db_user = data.read_user(&name)?;
    let json_user = db_user.into_json(&data).await?;
    Ok(FederationJson(WithContext::new_default(json_user)))
}
