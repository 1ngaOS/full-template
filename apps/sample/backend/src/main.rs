use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tower_http::cors::CorsLayer;

#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: String,
    name: String,
    email: String,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    timestamp: String,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
    message: Option<String>,
}

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

// In-memory storage (replace with database in production)
type UserStore = HashMap<String, User>;

async fn health() -> Json<HealthResponse> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();
    
    Json(HealthResponse {
        status: "ok".to_string(),
        timestamp,
    })
}

async fn get_users() -> Json<ApiResponse<Vec<User>>> {
    let users = vec![
        User {
            id: "1".to_string(),
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        },
        User {
            id: "2".to_string(),
            name: "Jane Smith".to_string(),
            email: "jane@example.com".to_string(),
        },
    ];

    Json(ApiResponse {
        success: true,
        data: Some(users),
        error: None,
        message: None,
    })
}

async fn get_user(Path(id): Path<String>) -> Result<Json<ApiResponse<User>>, StatusCode> {
    // Mock user lookup
    if id == "1" {
        Ok(Json(ApiResponse {
            success: true,
            data: Some(User {
                id: "1".to_string(),
                name: "John Doe".to_string(),
                email: "john@example.com".to_string(),
            }),
            error: None,
            message: None,
        }))
    } else if id == "2" {
        Ok(Json(ApiResponse {
            success: true,
            data: Some(User {
                id: "2".to_string(),
                name: "Jane Smith".to_string(),
                email: "jane@example.com".to_string(),
            }),
            error: None,
            message: None,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn create_user(
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<ApiResponse<User>>), StatusCode> {
    if payload.name.is_empty() || payload.email.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let user = User {
        id: "3".to_string(),
        name: payload.name,
        email: payload.email,
    };

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse {
            success: true,
            data: Some(user),
            error: None,
            message: Some("User created".to_string()),
        }),
    ))
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/users", get(get_users))
        .route("/api/users/:id", get(get_user))
        .route("/api/users", post(create_user))
        .layer(CorsLayer::permissive());

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind to address");

    tracing::info!("🚀 API server running on http://localhost:{}", port);
    tracing::info!("📚 Available endpoints:");
    tracing::info!("   GET  /health");
    tracing::info!("   GET  /api/users");
    tracing::info!("   GET  /api/users/:id");
    tracing::info!("   POST /api/users");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

