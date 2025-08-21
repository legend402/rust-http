use actix_web::{web, HttpResponse, Responder, Scope};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, Iden, IntoActiveModel, QueryFilter, Set};
use serde::Deserialize;
use crate::utils::{Response};
use entity::{prelude::*, *};

pub fn create_user_service() -> Scope {
    web::scope("/user")
        .route("/delete/{id}", web::delete().to(user_delete))
        .route("/update", web::put().to(user_update))
        .route("/get", web::get().to(user_get))
        .route("/create", web::post().to(user_create))
}

// 定义 POST 请求的 Body 参数结构（Data Transfer Object）
#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    username: String,
    password: String,
    email: String,
}

async fn user_create(db: web::Data<DatabaseConnection>, data: web::Json<CreateUserRequest>) -> Response<String> {
    println!("create user data: {:?}", data);
    let user = user::ActiveModel {
        username: Set(data.username.clone()),
        password: Set(data.password.clone()),
        email: Set(data.email.clone()),
        ..Default::default()
    };
    match User::insert(user).exec(db.get_ref()).await {
        Ok(user) => {
            println!("create user success: {:?}", user);
            Response::success("create user success".to_string())
        }
        Err(e) => {
            println!("create user error:{:?}", e);
            Response::server_error(e.to_string())
        }
    }
}

async fn user_delete(db: web::Data<DatabaseConnection>, info: web::Path<i32>) -> Response<String> {
    let id = info.into_inner();
    match User::delete_by_id(id).exec(db.get_ref()).await {
        Ok(user) => {
            println!("delete user success: {:?}", user);
            Response::success("delete user success".to_string())
        }
        Err(e) => {
            println!("delete user error:{:?}", e);
            Response::server_error(e.to_string())
        }
    }
}

// 定义 POST 请求的 Body 参数结构（Data Transfer Object）
#[derive(Debug, Deserialize)]
struct UpdateUserRequest {
    id: i32,
    username: Option<String>,
    email: Option<String>,
}

async fn user_update(db: web::Data<DatabaseConnection>, data: web::Json<UpdateUserRequest>) -> Response<String> {
    // 1. 先查询用户是否存在
    let existing_user = match User::find_by_id(data.id).one(db.get_ref()).await {
        Ok(Some(user)) => user,  // 用户存在，转为 ActiveModel 准备更新
        Ok(None) => return Response::server_error("用户不存在".to_string()),
        Err(e) => return Response::server_error(e.to_string()),
    };

    let mut user = existing_user.into_active_model();
    // 3. 仅更新客户端提供的非 None 字段
    if let Some(username) = &data.username {
        user.username = Set(username.clone());  // Set 表示需要更新该字段
    }
    if let Some(email) = &data.email {
        user.email = Set(email.clone());
    }
    match User::update(user).exec(db.get_ref()).await {
        Ok(_) => Response::success("update user success".to_string()),
        Err(e) => Response::server_error(e.to_string()),
    }
}

// 定义 POST 请求的 Body 参数结构（Data Transfer Object）
#[derive(Debug, Deserialize)]
struct SearchUserRequest {
    id: i32,
}
async fn user_get(db: web::Data<DatabaseConnection>, query: web::Query<SearchUserRequest>) -> Response<
    Vec<user::Model>
> {
    match User::find()
        .filter(user::Column::Id.eq(query.id))
        .all(db.get_ref()).await {
        Ok(user) => {
            println!("get user: {:?}", user);
            Response::success(user)
        }
        Err(e) => {
            println!("get user error:{:?}", e);
            Response::server_error(e.to_string())
        }
    }
}
