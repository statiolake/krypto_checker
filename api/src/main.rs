use anyhow::anyhow;
use anyhow::Context;
use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use itertools::Itertools;
use krypto_checker::AssignedFormula;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(tag = "type", rename = "error")]
pub struct Error {
    reason: String,
}

#[derive(Deserialize)]
pub struct SolveParams {
    cards: Option<String>,
    target: Option<String>,
    limit: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "type", rename = "success")]
pub struct Answer {
    formula: AssignedFormula,
    display: String,
}

struct BadRequest(anyhow::Error);

impl IntoResponse for BadRequest {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::BAD_REQUEST,
            Json(Error {
                reason: self.0.to_string(),
            }),
        )
            .into_response()
    }
}

#[axum::debug_handler]
async fn solve_get(Query(params): Query<SolveParams>) -> Result<Json<Vec<Answer>>, BadRequest> {
    let cards = params
        .cards
        .as_ref()
        .map(|x| x.trim())
        .ok_or_else(|| anyhow!("cards are not specified"))
        .map_err(BadRequest)?
        .split(',')
        .map(|s| s.trim().parse())
        .collect::<Result<Vec<_>, _>>()
        .context("invalid cards")
        .map_err(BadRequest)?;
    let target = params
        .target
        .as_ref()
        .map(|s| s.trim())
        .ok_or_else(|| anyhow!("target is not specified"))
        .map_err(BadRequest)?
        .parse()
        .context("invalid target")
        .map_err(BadRequest)?;
    let limit = params
        .limit
        .as_ref()
        .map(|s| s.trim())
        .unwrap_or("1")
        .parse()
        .context("invalid limit")
        .map_err(BadRequest)?;

    let answers = krypto_checker::find_answers(&cards, target)
        .take(limit)
        .map(|formula| Answer {
            display: formula.format(),
            formula,
        })
        .collect_vec();

    Ok(Json(answers))
}

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    let api = Router::new().route("/solve", get(solve_get));
    let app = Router::new().nest("/api/v1", api);

    let app = lambda_http::tower::ServiceBuilder::new()
        .layer(axum_aws_lambda::LambdaLayer::default())
        .service(app);

    lambda_http::run(app).await
}
