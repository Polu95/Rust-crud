use axum::{
    routing::{get, post, put, delete},
    Router,
    Json,
    extract::Path,
};

use mongodb::bson::{doc, oid::ObjectId};
use futures::stream::TryStreamExt;

use crate::{models::User, db::connect_db};

pub fn user_routes() -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/users", get(get_users))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
}

async fn create_user(Json(user): Json<User>) -> Json<User> {
    let db = connect_db().await;
    let collection = db.collection::<User>("users");

    let mut new_user = user;
    new_user.id = None;

    let result = collection
        .insert_one(new_user, None)
        .await
        .unwrap();

    Json(User {
        id: result.inserted_id.as_object_id(),
        name: "".to_string(),
        email: "".to_string(),
    })
}

async fn get_users() -> Json<Vec<User>> {
    let db = connect_db().await;
    let collection = db.collection::<User>("users");

    let mut cursor = collection
        .find(None, None)
        .await
        .unwrap();

    let mut users: Vec<User> = Vec::new();

    while let Some(user) = cursor.try_next().await.unwrap() {
        users.push(user);
    }

    Json(users)
}

async fn update_user(
    Path(id): Path<String>,
    Json(user): Json<User>,
) -> Json<&'static str> {
    let db = connect_db().await;
    let collection = db.collection::<User>("users");

    let obj_id = ObjectId::parse_str(id).unwrap();

    collection
        .update_one(
            doc! {"_id": obj_id},
            doc! {
                "$set": {
                    "name": user.name,
                    "email": user.email
                }
            },
            None,
        )
        .await
        .unwrap();

    Json("updated")
}

async fn delete_user(
    Path(id): Path<String>,
) -> Json<&'static str> {
    let db = connect_db().await;
    let collection = db.collection::<User>("users");

    let obj_id = ObjectId::parse_str(id).unwrap();

    collection
        .delete_one(doc! {"_id": obj_id}, None)
        .await
        .unwrap();

    Json("deleted")
}