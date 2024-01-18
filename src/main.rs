use axum::{routing::get, Json, Router};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::Serialize;

#[derive(Serialize)]
#[serde(untagged)]
enum Response {
    Valid { unix: i64, utc: String },
    Invalid,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/api/timestamp/:date_string", get(handle_timestamp));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:81").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello! Send any timestamps to the API endpoint."
}

async fn handle_timestamp(
    axum::extract::Path(date_string): axum::extract::Path<String>,
) -> Json<Response> {
    let response = if date_string.is_empty() {
        let now = Utc::now();
        Response::Valid {
            unix: now.timestamp_millis(),
            utc: now.to_rfc2822(),
        }
    } else {
        match parse_date_string(&date_string) {
            Some(utc_date) => Response::Valid {
                unix: utc_date.timestamp_millis(),
                utc: utc_date.to_rfc2822(),
            },
            None => Response::Invalid,
        }
    };
    Json(response)
}

fn parse_date_string(date_string: &str) -> Option<DateTime<Utc>> {
    if let Ok(date) = DateTime::parse_from_rfc3339(date_string) {
        // RFC 3339 & ISO 8601
        return Some(date.with_timezone(&Utc));
    }

    if let Ok(unix_timestamp) = date_string.parse::<i64>() {
        // Check length to determine if the timestamp is in seconds or milliseconds
        let (seconds, nanoseconds) = if date_string.len() <= 10 {
            (unix_timestamp, 0)
        } else {
            (
                unix_timestamp / 1000,
                (unix_timestamp % 1000) as u32 * 1_000_000,
            )
        };

        if let Some(naive_datetime) = NaiveDateTime::from_timestamp_opt(seconds, nanoseconds) {
            return Some(Utc.from_utc_datetime(&naive_datetime));
        }
    }

    None
}
