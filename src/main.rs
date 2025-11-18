use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;
use rand_core::OsRng;
use sea_orm::entity::prelude::*;
use sea_orm::{ColumnTrait, Database, DatabaseConnection, EntityTrait, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No relations defined")
    }
}

impl ActiveModelBehavior for ActiveModel {}

/// Create a hash of the password using Argon2 before storing it.
fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
}

/// Verify that the supplied password matches the stored Argon2 hash.
fn verify_password(hash: &str, password: &str) -> Result<(), argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    password: String,
}

// Simple health check endpoint to verify the service is reachable.
#[get("/")]
async fn index(_db: web::Data<DatabaseConnection>) -> impl Responder {
    "Hello, World!"
}

#[post("/auth/login")]
async fn login(
    db: web::Data<DatabaseConnection>,
    login_payload: web::Json<LoginRequest>,
) -> impl Responder {
    let existing_user = Entity::find()
        .filter(Column::Username.eq(login_payload.username.clone()))
        .one(db.get_ref())
        .await;

    match existing_user {
        Ok(Some(user)) => {
            if verify_password(&user.password, &login_payload.password).is_err() {
                HttpResponse::Unauthorized().body("Invalid password.")
            } else {
                HttpResponse::Ok().body("Login Successful.")
            }
        }
        Ok(None) => HttpResponse::Unauthorized().body("Invalid username or password."),
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("DB error on fetching user: {}", e)),
    }
}

#[post("/auth/register")]
async fn register(
    db: web::Data<DatabaseConnection>,
    register_payload: web::Json<RegisterRequest>,
) -> impl Responder {
    // Ensure username uniqueness before inserting.
    let existing_user = Entity::find()
        .filter(Column::Username.eq(register_payload.username.clone()))
        .one(db.get_ref())
        .await;

    match existing_user {
        Ok(Some(_)) => {
            return HttpResponse::BadRequest().body("Username already exists.");
        }
        Ok(None) => {}
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("DB error on checking username: {}", e));
        }
    }

    let hashed_password = match hash_password(&register_payload.password) {
        Ok(hash) => hash,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Password hashing failed: {}", e));
        }
    };

    let new_user = ActiveModel {
        username: Set(register_payload.username.clone()),
        password: Set(hashed_password),
        ..Default::default()
    };

    // Persist the new user and return the created record.
    let insert_result = new_user.insert(db.get_ref()).await;

    match insert_result {
        Ok(created_user) => {
            HttpResponse::Ok().json(created_user)
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("DB error on insert user: {}", e))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Open a shared database connection for all request handlers.
    let database_connection: DatabaseConnection = Database::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    let shared_connection = web::Data::new(database_connection);

    HttpServer::new(move || {
        App::new()
            .app_data(shared_connection.clone())
            .service(index)
            .service(login)
            .service(register)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
