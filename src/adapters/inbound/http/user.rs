use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
    Json,
};
use serde::Deserialize;
use tracing::info;
use validator::Validate;

use crate::adapters::inbound::response::validation::{format_validation_errors};
use crate::app::App;
use crate::adapters::inbound::response::validation::USERNAME_RE;
use crate::application::user::change_password;
use crate::domain::user::entity::UserId;
use crate::adapters::inbound::response::response::{created, error, validation_error};


use crate::application::user::{
    register_user::RegisterUser,
    login_user::LoginUser,
    forgot_password::ForgotPassword,
    inactive_user::InactiveUser,
};


pub fn routes(state: App) -> Router {
    Router::new()
        .route("/users/register", post(register))
        .route("/users/login", post(login))
        .route("/users/forgot-password", post(forgot_password))
        .route("/users/inactive", post(inactive_user))
        .with_state(state)
}

// Request Structs
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterUserRequest {

    #[validate(required, length(min = 1), email)]
    pub email: Option<String>,

    #[validate(required, length(min = 3, max = 20), regex(path = "USERNAME_RE"))]
    pub username: Option<String>,

    #[validate(required, length(min = 1))]
    pub name: Option<String>,

    #[validate(required, length(min = 6))]
    pub password: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginUserRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Deserialize)]
pub struct InactiveUserRequest {
    pub user_id: String,
}


pub async fn register(
    State(app): State<App>,
    Json(payload): Json<RegisterUserRequest>,
) -> impl IntoResponse {

    if let Err(e) = payload.validate() {
        let errors = format_validation_errors(e);

        return (
            StatusCode::BAD_REQUEST,
            Json(validation_error(errors)),
        );
    }

    let use_case = RegisterUser::new(&app.user_repo);

    match use_case.execute(
        payload.email.unwrap(),
        payload.username.unwrap(),
        payload.name.unwrap(),
        payload.password.unwrap(),
    ).await {
        Ok(_) => (
            StatusCode::CREATED,
            Json(created((), "user registered")),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(error(e)),
        ),
    }
}


pub async fn login(
    State(app): State<App>,
    Json(payload): Json<LoginUserRequest>,
) -> impl IntoResponse {
    info!("login user endpoint hit");

    let use_case = LoginUser::new(&app.user_repo);
    match use_case.execute(&payload.email, &payload.password).await {
        Ok(_) => (
            StatusCode::OK,
             Json(created((), "user logged in" ))
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(error(e.to_string())),
        ),
    }
}

pub async fn forgot_password(
    Json(payload): Json<ForgotPasswordRequest>,
) -> impl IntoResponse {
    let use_case = ForgotPassword::new();

    match use_case.execute(&payload.email).await {
        Ok(_) => (
            StatusCode::CREATED,
            Json(created((), "password reset email sent")),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(error(e.to_string())),
        ),
    }
}


pub async fn inactive_user(
    State(app): State<App>,
    Json(payload): Json<InactiveUserRequest>,
) -> impl IntoResponse {
    info!("inactive user endpoint hit");

    let use_case = InactiveUser::new(&app.user_repo);
    match use_case.execute(UserId(payload.user_id)).await {
       Ok(_) => (
            StatusCode::CREATED,
            Json(created((), "user deactivated" ))
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(error(e.to_string())),
        ),
    }
}

