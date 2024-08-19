use gloo_net::http::Request;
use gloo_net::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginResponse {
    pub user_id: Uuid,
    pub email: String,
    pub access_token: String,
    pub token_type: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct UserProfileResponse {
    pub user_id: Uuid,
    pub email: String,
}

pub async fn login(email: String, password: String) -> Result<LoginResponse, Error> {
    const ENDPOINT: &str = "/api/v1/auth/sign-in";
    let json_value = json!({"email": email, "password": password});
    log::info!("Doing login, endpoint={ENDPOINT}");
    let request_body = serde_json::to_string(&json_value).expect("Serialize should not fail");
    let response = Request::post(ENDPOINT)
        .header("Content-Type", "application/json")
        .body(request_body)?
        .send()
        .await?
        .json::<LoginResponse>()
        .await?;
    log::info!("Api auth login, email={email}");
    Ok(response)
}

pub async fn get_user_profile(token: String) -> Result<UserProfileResponse, Error> {
    const ENDPOINT: &str = "/api/v1/auth/user-profile";
    let response = Request::get(ENDPOINT)
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await?
        .json::<UserProfileResponse>()
        .await?;
    log::info!("Api get user profile, token={}", token);
    Ok(response)
}
