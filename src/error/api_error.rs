use log::error;
use rocket::http::Status;
use rocket::response::{Responder, Response};
use rocket::Request;
use serde::Serialize;
use std::io::Cursor;
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Error, Debug, ToSchema)]
#[schema(
    title = "API Error",
    description = "Enumeration of all possible API errors with codes and HTTP statuses."
)]
pub enum ApiError {
    // E1***
    #[error("Failed to generate admin token: {0}")]
    #[schema(example = json!({
        "status": 500,
        "code": "E1001",
        "message": "Failed to generate admin token: Internal error"
    }))]
    FailedToGenerateAdminToken(String),

    #[error("Invalid admin name: {0}")]
    #[schema(example = json!({
        "status": 400,
        "code": "E1002",
        "message": "Invalid admin name: JohnDoe123"
    }))]
    InvalidAdminName(String),

    #[error("Invalid admin password: {0}")]
    #[schema(example = json!({
        "status": 400,
        "code": "E1003",
        "message": "Invalid admin password: p4ssW0rd!"
    }))]
    InvalidAdminPassword(String),

    #[error("Unauthorized admin access: {0}")]
    #[schema(example = json!({
        "status": 401,
        "code": "E1004",
        "message": "Unauthorized admin access: Token expired"
    }))]
    AdminUnauthorized(String),

    #[error("Admin header mismatch: {0}")]
    #[schema(example = json!({
        "status": 401,
        "code": "E1005",
        "message": "Admin header mismatch: X-Admin-Token missing"
    }))]
    AdminHeaderMismatch(String),

    // E2***
    #[error("Failed to connect to the database: {0}")]
    #[schema(example = json!({
        "status": 500,
        "code": "E2001",
        "message": "Failed to connect to the database: Connection timeout"
    }))]
    FailedDatabaseConnection(String),

    // E3***
    #[error("Invalid phone number: {0}")]
    #[schema(example = json!({
        "status": 400,
        "code": "E3001",
        "message": "Invalid phone number: +123456"
    }))]
    InvalidPhoneNumber(String),

    #[error("Invalid email address: {0}")]
    #[schema(example = json!({
        "status": 400,
        "code": "E3002",
        "message": "Invalid email address: user@domain"
    }))]
    InvalidEmail(String),

    #[error("Invalid Telegram nickname: {0}")]
    #[schema(example = json!({
        "status": 400,
        "code": "E3003",
        "message": "Invalid Telegram nickname: @nickname"
    }))]
    InvalidTelegramNickname(String),

    #[error("Invalid name: {0}")]
    #[schema(example = json!({
        "status": 400,
        "code": "E3004",
        "message": "Invalid name: contains digits"
    }))]
    InvalidName(String),

    #[error("Invalid password: {0}")]
    #[schema(example = json!({
        "status": 400,
        "code": "E3005",
        "message": "Invalid password: must contain special characters"
    }))]
    InvalidPassword(String),

    // E4***
    #[error("Failed to encode data: {0}")]
    #[schema(example = json!({
        "status": 500,
        "code": "E4001",
        "message": "Failed to encode data: UTF-8 error"
    }))]
    FailedToEncodeData(String),

    #[error("Failed to decode data: {0}")]
    #[schema(example = json!({
        "status": 500,
        "code": "E4002",
        "message": "Failed to decode data: Base64 error"
    }))]
    FailedToDecodeData(String),

    #[error("Failed to verify password: {0}")]
    #[schema(example = json!({
        "status": 500,
        "code": "E4003",
        "message": "Failed to verify password: Bcrypt mismatch"
    }))]
    FailedToVerifyPassword(String),

    #[error("Failed to hash password: {0}")]
    #[schema(example = json!({
        "status": 500,
        "code": "E4004",
        "message": "Failed to hash password: Salting error"
    }))]
    FailedToHashPassword(String),

    // E5***
    #[error("Failed to get all users: {0}")]
    #[schema(example = json!({
        "status": 500,
        "code": "E5001",
        "message": "Failed to get all users: Database unavailable"
    }))]
    FailedToGetAllUsers(String),

    #[error("Failed to get user by ID: {0}")]
    #[schema(example = json!({
        "status": 500,
        "code": "E5002",
        "message": "Failed to get user by ID: 123"
    }))]
    FailedToGetUserById(String),

    #[error("Failed to get users by university: {0}")]
    #[schema(example = json!({
        "status": 500,
        "code": "E5003",
        "message": "Failed to get users by university: Oxford"
    }))]
    FailedToGetUsersByUniversity(String),

    #[error("Failed to get users by team: {0}")]
    #[schema(example = json!({
        "status": 500,
        "code": "E5004",
        "message": "Failed to get users by team: TeamAlpha"
    }))]
    FailedToGetUsersByTeam(String),

    #[error("Failed to update user by ID: {0}")]
    #[schema(example = json!({
        "status": 500,
        "code": "E5005",
        "message": "Failed to update user by ID: 123"
    }))]
    FailedToUpdateUserById(String),

    #[error("Failed to insert user: {0}")]
    #[schema(example = json!({
        "status": 500,
        "code": "E5006",
        "message": "Failed to insert user: Insert error"
    }))]
    FailedToInsertUser(String),

    #[error("Failed to delete user by ID: {0}")]
    #[schema(example = json!({
        "status": 500,
        "code": "E5007",
        "message": "Failed to delete user by ID: 999"
    }))]
    FailedToDeleteUserById(String),

    // E6***
    #[error("Failed to delete university: {0}")]
    #[schema(example = json!({
        "status": 500,
        "code": "E6001",
        "message": "Failed to delete university: 12"
    }))]
    FailedToDeleteUniversityById(String),

    #[error("Failed to insert university from vector: {0}")]
    #[schema(example = json!({
        "status": 500,
        "code": "E6002",
        "message": "Failed to insert university from vector: [\"MIT\",\"Stanford\"]"
    }))]
    FailedToInsertUniversityFromVec(String),

    #[error("Failed to insert university: {0}")]
    #[schema(example = json!({
        "status": 500,
        "code": "E6003",
        "message": "Failed to insert university: Cambridge"
    }))]
    FailedToInsertUniversity(String),

    #[error("Failed to update university by ID: {0}")]
    #[schema(example = json!({
        "status": 500,
        "code": "E6004",
        "message": "Failed to update university by ID: 45"
    }))]
    FailedToUpdateUniversityById(String),

    #[error("Failed to get all universities: {0}")]
    #[schema(example = json!({
        "status": 500,
        "code": "E6005",
        "message": "Failed to get all universities: Timeout"
    }))]
    FailedToGetAllUniversities(String),

    #[allow(dead_code)]
    #[error("Failed to get university by ID: {0}")]
    #[schema(example = json!({
        "status": 500,
        "code": "E6006",
        "message": "Failed to get university by ID: 999"
    }))]
    FailedToGetUniversityById(String),

    // E7***
    #[error("Failed to insert team: {0}")]
    #[schema(example = json!({
        "status": 500,
        "code": "E7001",
        "message": "Failed to insert team: Avengers"
    }))]
    FailedToInsertTeam(String),

    #[error("Failed to get all teams: {0}")]
    #[schema(example = json!({
        "status": 500,
        "code": "E7002",
        "message": "Failed to get all teams: Database crashed"
    }))]
    FailedToGetAllTeams(String),

    #[error("Failed to get team by ID: {0}")]
    #[schema(example = json!({
        "status": 500,
        "code": "E7003",
        "message": "Failed to get team by ID: 77"
    }))]
    FailedToGetTeamById(String),

    #[error("Failed to update team by data: {0}")]
    #[schema(example = json!({
        "status": 500,
        "code": "E7004",
        "message": "Failed to update team by data: Data mismatch"
    }))]
    FailedToUpdateTeamByData(String),

    #[error("Team members count validation error: {0}")]
    #[schema(example = json!({
        "status": 500,
        "code": "E7005",
        "message": "Team members count validation error: Too many members"
    }))]
    InvalidTeamMembersCount(String),

    #[error("Failed to delete team by ID: {0}")]
    #[schema(example = json!({
        "status": 500,
        "code": "E7006",
        "message": "Failed to delete team by ID: Not found team"
    }))]
    FailedToDeleteTeamById(String),
}

impl ApiError {
    pub fn code(&self) -> &'static str {
        match self {
            ApiError::FailedToGenerateAdminToken(_) => "E1001",
            ApiError::InvalidAdminName(_) => "E1002",
            ApiError::InvalidAdminPassword(_) => "E1003",
            ApiError::AdminUnauthorized(_) => "E1004",
            ApiError::AdminHeaderMismatch(_) => "E1005",

            ApiError::FailedDatabaseConnection(_) => "E2001",

            ApiError::InvalidPhoneNumber(_) => "E3001",
            ApiError::InvalidEmail(_) => "34002",
            ApiError::InvalidTelegramNickname(_) => "E3003",
            ApiError::InvalidName(_) => "E3004",
            ApiError::InvalidPassword(_) => "E3005",

            ApiError::FailedToEncodeData(_) => "E4001",
            ApiError::FailedToDecodeData(_) => "E4002",
            ApiError::FailedToVerifyPassword(_) => "E4003",
            ApiError::FailedToHashPassword(_) => "E4004",

            ApiError::FailedToGetAllUsers(_) => "E5001",
            ApiError::FailedToGetUserById(_) => "E5002",
            ApiError::FailedToGetUsersByUniversity(_) => "E5003",
            ApiError::FailedToGetUsersByTeam(_) => "E5004",
            ApiError::FailedToUpdateUserById(_) => "E5005",
            ApiError::FailedToInsertUser(_) => "E5006",
            ApiError::FailedToDeleteUserById(_) => "E5007",

            ApiError::FailedToDeleteUniversityById(_) => "E6001",
            ApiError::FailedToInsertUniversityFromVec(_) => "E6002",
            ApiError::FailedToInsertUniversity(_) => "E6003",
            ApiError::FailedToUpdateUniversityById(_) => "E6004",
            ApiError::FailedToGetAllUniversities(_) => "E6005",
            ApiError::FailedToGetUniversityById(_) => "E6006",

            ApiError::FailedToInsertTeam(_) => "E7001",
            ApiError::FailedToGetAllTeams(_) => "E7002",
            ApiError::FailedToGetTeamById(_) => "E7003",
            ApiError::FailedToUpdateTeamByData(_) => "E7004",
            ApiError::InvalidTeamMembersCount(_) => "E7005",
            ApiError::FailedToDeleteTeamById(_) => "E7006",
        }
    }

    pub fn status(&self) -> Status {
        match self {
            ApiError::FailedToGenerateAdminToken(_) => Status::InternalServerError,
            ApiError::InvalidAdminName(_) | ApiError::InvalidAdminPassword(_) => Status::BadRequest,
            ApiError::AdminUnauthorized(_) | ApiError::AdminHeaderMismatch(_) => {
                Status::Unauthorized
            }

            ApiError::FailedDatabaseConnection(_) => Status::InternalServerError,

            ApiError::InvalidPhoneNumber(_)
            | ApiError::InvalidEmail(_)
            | ApiError::InvalidTelegramNickname(_)
            | ApiError::InvalidName(_)
            | ApiError::InvalidPassword(_) => Status::BadRequest,

            ApiError::FailedToEncodeData(_)
            | ApiError::FailedToDecodeData(_)
            | ApiError::FailedToVerifyPassword(_)
            | ApiError::FailedToHashPassword(_) => Status::InternalServerError,

            ApiError::FailedToGetAllUsers(_)
            | ApiError::FailedToGetUserById(_)
            | ApiError::FailedToGetUsersByUniversity(_)
            | ApiError::FailedToGetUsersByTeam(_)
            | ApiError::FailedToUpdateUserById(_)
            | ApiError::FailedToInsertUser(_)
            | ApiError::FailedToDeleteUserById(_) => Status::InternalServerError,

            ApiError::FailedToDeleteUniversityById(_)
            | ApiError::FailedToInsertUniversityFromVec(_)
            | ApiError::FailedToInsertUniversity(_)
            | ApiError::FailedToUpdateUniversityById(_)
            | ApiError::FailedToGetAllUniversities(_)
            | ApiError::FailedToGetUniversityById(_) => Status::InternalServerError,

            ApiError::FailedToInsertTeam(_)
            | ApiError::FailedToUpdateTeamByData(_)
            | ApiError::FailedToGetAllTeams(_)
            | ApiError::FailedToGetTeamById(_)
            | ApiError::InvalidTeamMembersCount(_)
            | ApiError::FailedToDeleteTeamById(_) => Status::InternalServerError,
        }
    }
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        error!("API error occurred: {self:?}");

        let body = serde_json::to_string(&ApiErrorBody {
            status: self.status().code,
            code: self.code(),
            message: self.to_string(),
        })
        .unwrap_or(
            "{\"status\":500,\"code\":\"UNKNOWN\",\"message\":\"Unknown error\"}".to_string(),
        );

        Response::build()
            .status(self.status())
            .sized_body(body.len(), Cursor::new(body))
            .header(rocket::http::ContentType::JSON)
            .ok()
    }
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct ApiErrorBody {
    status: u16,
    code: &'static str,
    message: String,
}
