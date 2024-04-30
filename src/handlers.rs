use axum::{
    extract::{Path, State},
    http::{status, HeaderMap, StatusCode},
    Json
};
use chrono::{DateTime, Utc};
use serde_json::{json, Value};

use sqlx::{Any, Decode, PgPool, Postgres};
use serde::{Serialize, Deserialize};
use sqlx::postgres::PgRow;
use sqlx::Error;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct NewWorkspace{

    name: String,
    description: String,
    status: String
}

pub async fn create_workspace(
    State(pool): State<PgPool>, 

    headers: HeaderMap,
    Json(new_workspace): Json<NewWorkspace>

) -> Result<Json<Value>, (StatusCode, String)> {

    let resp = sqlx::query(
        "INSERT INTO workspaces (name, description, owner_user_id, status) VALUES (
        $1, $2, $3, $4);"
    )
    .bind(&new_workspace.name)
    .bind(&new_workspace.description)
    .bind(headers.get("user_id").unwrap().to_str().unwrap().parse::<i32>().unwrap())
    .bind(&new_workspace.status)
    .execute(&pool)
    .await
    .map_err(|err| (StatusCode::UNPROCESSABLE_ENTITY, format!("Cannot insert new workspace: {}", err)))?;

    Ok(Json(json!(new_workspace)))
}


pub async fn get_workspaces(

    State(pool): State<PgPool>, 
    headers: HeaderMap

) -> Result<Json<Vec<Value>>, (StatusCode, String)> 
    {

    let result = sqlx::query_as::<_, NewWorkspace>("SELECT * FROM workspaces WHERE owner_user_id = $1;")
        .bind(headers.get("user_id").unwrap().to_str().unwrap().parse::<i32>().unwrap())
        .fetch_all(&pool)
        .await
        .map_err(|err| (StatusCode::NOT_FOUND, format!("Cannot get workspaces: {}", err)))?;

    Ok(Json(result.into_iter().map(|workspace| json!(workspace)).collect()))
    }


#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Workspace{
    
    name: String,
    description: String,
    status: String
}
    

pub async fn get_workspace(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    headers: HeaderMap
) -> Result<Json<Workspace>, (StatusCode, String)> {

    let workspace = sqlx::query_as::<_,Workspace>("SELECT * FROM workspaces WHERE id = $1 AND owner_user_id = $2;")
        .bind(id)
        .bind(headers.get("user_id").unwrap().to_str().unwrap().parse::<i32>().unwrap())
        .fetch_one(&pool)
        .await
        .map_err(|err| (StatusCode::NOT_FOUND, format!("Cannot get workspace: {}", err)))?;

    Ok(Json(workspace))
}

pub async fn update_workspace(
    State(pool): State<PgPool>,

    Path(id): Path<i32>,
    Json(status): Json<String>,

    headers: HeaderMap,
) -> Result<Json<Value>, (StatusCode, String)> {

    let resp = sqlx::query(
        "UPDATE workspaces SET status = $1 WHERE id = $2 AND owner_user_id = $2;"
    )
    .bind(&status)
    .bind(&id)
    .bind(headers.get("user_id").unwrap().to_str().unwrap().parse::<i32>().unwrap())
    .execute(&pool)
    .await
    .map_err(|err| (StatusCode::UNPROCESSABLE_ENTITY, format!("Cannot update workspace status: {}", err)))?;

    Ok(Json(json!({"msg": "Workspace status updated successfully"})))
}