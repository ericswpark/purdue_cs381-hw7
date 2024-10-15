use crate::structs::*;
use std::path::PathBuf;

use anyhow::Result;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::Json;
use cs381_hw7::*;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

fn do_question_two(s: Vec<u32>, e: Vec<u32>) -> Result<u32, AppError> {
    Ok(vip_scheduler(&s, &e)?)
}

pub async fn question_two(Json(payload): Json<QuestionTwo>) -> impl IntoResponse {
    match do_question_two(payload.s, payload.e) {
        Ok(result) => (StatusCode::OK, Json(QuestionTwoAnswer { answer: result })).into_response(),
        Err(e) => e.into_response(),
    }
}

pub async fn question_two_test_cases() -> impl IntoResponse {
    let file = PathBuf::from("q2_test_cases.json");

    match load_file(file).await {
        Ok(content) => Html(content).into_response(),
        Err(err) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error loading test cases: {}", err),
        )
            .into_response(),
    }
}

async fn load_file(path: PathBuf) -> Result<String, std::io::Error> {
    let mut file = File::open(path).await?;
    let mut content = String::new();
    file.read_to_string(&mut content).await?;
    Ok(content)
}
