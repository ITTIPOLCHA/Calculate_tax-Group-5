use serde::{Deserialize, Serialize};

// get
#[derive(Serialize, Deserialize)]
pub struct TaxMenu {
    pub user_id: i32,
    pub right: String,
    pub choices: Vec<TaxChoice>,
}

#[derive(Serialize, Deserialize)]
pub struct TaxChoice {
    pub choice: String,
    pub link: String,
}

// patch
/// request
#[derive(Debug, Deserialize)]
pub struct UpdateRequest {
    pub user_id: i32,
    pub right: String,
    pub update: Vec<UpdateDetail>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDetail {
    pub tax: String,
    pub detail: Vec<Detail>,
}

#[derive(Debug, Deserialize)]
pub struct Detail {
    pub vat: String,
}

#[derive(Debug, Deserialize)]
pub struct PatchResponse {
    pub state: String,
    pub update: Vec<UpdateDetail>,
    pub response_at: String,
}

// delete
/// request
#[derive(Debug, Deserialize, Serialize)]
pub struct TaxResponse {
    pub user_id: i32,
    pub tax: String,
    pub response_at: String,
}