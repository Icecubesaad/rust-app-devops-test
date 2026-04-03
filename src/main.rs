use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Item {
    id: Option<u32>,
    name: String,
    description: Option<String>,
    price: f64,
    quantity: u32,
    created_at: Option<String>,
}

#[derive(Debug, Serialize)]
struct ApiResponse<T> {
    message: String,
    data: Option<T>,
}

struct AppState {
    items: Mutex<Vec<Item>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting Rust Test Application");
    log::info!("Server will listen on 0.0.0.0:8080");

    let app_state = web::Data::new(AppState {
        items: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health))
            .route("/items", web::get().to(get_items))
            .route("/items", web::post().to(create_item))
            .route("/items/{id}", web::get().to(get_item))
            .route("/stats", web::get().to(get_stats))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

async fn index() -> impl Responder {
    log::info!("Root endpoint accessed");
    
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Rust Test Application",
        "status": "running",
        "timestamp": Utc::now().to_rfc3339(),
        "framework": "Actix-web",
        "endpoints": {
            "health": "/health",
            "items": "/items",
            "stats": "/stats"
        }
    }))
}

async fn health() -> impl Responder {
    log::info!("Health check endpoint accessed");
    
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": Utc::now().to_rfc3339(),
        "service": "rust-app",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

async fn get_items(data: web::Data<AppState>) -> impl Responder {
    log::info!("Get items endpoint accessed");
    
    let items = data.items.lock().unwrap();
    
    HttpResponse::Ok().json(serde_json::json!({
        "items": items.clone(),
        "count": items.len()
    }))
}

async fn create_item(
    item: web::Json<Item>,
    data: web::Data<AppState>,
) -> impl Responder {
    log::info!("Create item endpoint accessed");
    
    let mut items = data.items.lock().unwrap();
    let id = items.len() as u32 + 1;
    
    let new_item = Item {
        id: Some(id),
        name: item.name.clone(),
        description: item.description.clone(),
        price: item.price,
        quantity: item.quantity,
        created_at: Some(Utc::now().to_rfc3339()),
    };
    
    items.push(new_item.clone());
    
    log::info!("Item created with ID: {}", id);
    
    HttpResponse::Created().json(ApiResponse {
        message: "Item created successfully".to_string(),
        data: Some(new_item),
    })
}

async fn get_item(
    path: web::Path<u32>,
    data: web::Data<AppState>,
) -> impl Responder {
    let item_id = path.into_inner();
    log::info!("Get item endpoint accessed for ID: {}", item_id);
    
    let items = data.items.lock().unwrap();
    
    match items.iter().find(|item| item.id == Some(item_id)) {
        Some(item) => HttpResponse::Ok().json(item),
        None => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Item not found",
            "id": item_id
        })),
    }
}

async fn get_stats(data: web::Data<AppState>) -> impl Responder {
    log::info!("Stats endpoint accessed");
    
    let items = data.items.lock().unwrap();
    
    HttpResponse::Ok().json(serde_json::json!({
        "total_items": items.len(),
        "timestamp": Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION")
    }))
}
