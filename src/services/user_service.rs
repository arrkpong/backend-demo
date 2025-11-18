use crate::models::user::{
    ActiveModel as UserActiveModel, Entity as UserEntity, Model as UserModel,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

/// Fetches a user by username from Postgres.
pub async fn find_user_by_username(
    db: &DatabaseConnection,
    username: &str,
) -> Result<Option<UserModel>, sea_orm::DbErr> {
    UserEntity::find()
        .filter(<UserEntity as EntityTrait>::Column::Username.eq(username.to_owned()))
        .one(db)
        .await
}

/// Fetches a user by primary key.
pub async fn find_user_by_id(
    db: &DatabaseConnection,
    user_id: i32,
) -> Result<Option<UserModel>, sea_orm::DbErr> {
    UserEntity::find_by_id(user_id).one(db).await
}

/// Inserts a new user record.
pub async fn create_user(
    db: &DatabaseConnection,
    username: String,
    password: String,
) -> Result<UserModel, sea_orm::DbErr> {
    let new_user = UserActiveModel {
        username: Set(username),
        password: Set(password),
        ..Default::default()
    };

    new_user.insert(db).await
}
