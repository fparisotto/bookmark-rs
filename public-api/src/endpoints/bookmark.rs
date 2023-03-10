use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::{routing::get, routing::post, Extension, Router};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::auth::Claims;
use crate::database::bookmark::{BookmarkTable, BookmarkWithUserData, TagOperation};
use crate::database::task::{BookmarkTask, BookmarkTaskTable};
use crate::error::Result;
use crate::AppContext;

use crate::endpoints::Error;

pub fn routes() -> Router {
    Router::new()
        .route("/tags", get(get_all_tags))
        .route("/tags/:tag", get(get_bookmarks_by_tag))
        .route("/bookmarks", get(get_bookmarks).post(new_bookmark))
        .route("/bookmarks/:id", get(get_bookmark))
        .route("/bookmarks/:id/tags", post(set_tags).patch(append_tags))
}

#[derive(Debug, Serialize, Deserialize)]
struct TagCount {
    tag: String,
    count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct TagsWithCounters {
    tags: Vec<TagCount>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Tags {
    tags: Vec<String>,
}

#[derive(Debug, Serialize)]
struct Bookmarks {
    bookmarks: Vec<BookmarkWithUserData>,
}

#[derive(Debug, Deserialize)]
struct NewBookmark {
    url: Url,
    tags: Option<Vec<String>>,
}

#[debug_handler()]
async fn get_bookmarks(
    claims: Claims,
    Extension(app_context): Extension<AppContext>,
) -> Result<Json<Bookmarks>> {
    let bookmarks = BookmarkTable::get_bookmarks_by_user(&app_context.db, &claims.user_id).await?;
    Ok(Json(Bookmarks { bookmarks }))
}

#[debug_handler()]
async fn get_all_tags(
    claims: Claims,
    Extension(app_context): Extension<AppContext>,
) -> Result<Json<TagsWithCounters>> {
    let tags = BookmarkTable::get_tag_count_by_user(&app_context.db, &claims.user_id).await?;
    let tags = tags
        .into_iter()
        .map(|(tag, count)| TagCount { tag, count })
        .collect::<Vec<_>>();
    Ok(Json(TagsWithCounters { tags }))
}

#[debug_handler()]
async fn get_bookmarks_by_tag(
    claims: Claims,
    Extension(app_context): Extension<AppContext>,
    Path(tag): Path<String>,
) -> Result<Json<Bookmarks>> {
    let bookmarks =
        BookmarkTable::get_bookmarks_by_tag(&app_context.db, &claims.user_id, &tag).await?;
    Ok(Json(Bookmarks { bookmarks }))
}

#[debug_handler()]
async fn get_bookmark(
    claims: Claims,
    Extension(app_context): Extension<AppContext>,
    Path(id): Path<String>,
) -> Result<Json<BookmarkWithUserData>> {
    let maybe_bookmark =
        BookmarkTable::get_bookmark_with_user_data(&app_context.db, &claims.user_id, &id).await?;
    match maybe_bookmark {
        Some(bookmark) => Ok(Json(bookmark)),
        None => Err(Error::NotFound),
    }
}

#[debug_handler()]
async fn new_bookmark(
    claims: Claims,
    Extension(app_context): Extension<AppContext>,
    Json(input): Json<NewBookmark>,
) -> Result<(StatusCode, Json<BookmarkTask>)> {
    // FIXME put this validation in a better place
    let mut tags = input.tags.clone().unwrap_or_default();
    tags.retain(|t| !t.trim().is_empty());
    let mut tx = app_context.db.begin().await?;
    let response = BookmarkTaskTable::create(&mut tx, &claims.user_id, &input.url, &tags).await?;
    tx.commit().await?;
    Ok((StatusCode::CREATED, Json(response)))
}

#[debug_handler()]
async fn set_tags(
    claims: Claims,
    Extension(app_context): Extension<AppContext>,
    Path(bookmark_id): Path<String>,
    Json(tags): Json<Tags>,
) -> Result<Json<BookmarkWithUserData>> {
    let mut tx = app_context.db.begin().await?;
    let updated = BookmarkTable::update_tags(
        &mut tx,
        &claims.user_id,
        &bookmark_id,
        TagOperation::Set(tags.tags),
    )
    .await?;
    tx.commit().await?;
    Ok(Json(updated))
}

#[debug_handler()]
async fn append_tags(
    claims: Claims,
    Extension(app_context): Extension<AppContext>,
    Path(bookmark_id): Path<String>,
    Json(tags): Json<Tags>,
) -> Result<Json<BookmarkWithUserData>> {
    let mut tx = app_context.db.begin().await?;
    let updated = BookmarkTable::update_tags(
        &mut tx,
        &claims.user_id,
        &bookmark_id,
        TagOperation::Append(tags.tags),
    )
    .await?;
    tx.commit().await?;
    Ok(Json(updated))
}
