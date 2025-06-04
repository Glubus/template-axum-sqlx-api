//! # Help Routes Module
//!
//! Ce module contient les routes d'aide et de diagnostic de l'API.
//! Ces routes permettent de vérifier l'état de santé du système et d'obtenir
//! des informations utiles pour le debugging et le monitoring.

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sysinfo::{Disks, System};
use std::time::Instant;

use crate::db::DatabaseManager;

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub version: String,
    pub database: DatabaseStatus,
    pub system: SystemMetrics,
    pub performance: PerformanceMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseStatus {
    pub connected: bool,
    pub response_time_ms: Option<u64>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub cpu_count: usize,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub memory_usage_percent: f32,
    pub disk_usage_percent: f32,
    pub uptime: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub response_time_ms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoResponse {
    pub name: String,
    pub version: String,
    pub description: String,
    pub authors: Vec<String>,
    pub endpoints: Vec<EndpointInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndpointInfo {
    pub path: String,
    pub method: String,
    pub description: String,
}

/// Route de santé générale du système
pub async fn health_check(State(db): State<DatabaseManager>) -> Result<Json<HealthResponse>, StatusCode> {
    let start_time = Instant::now();
    
    // Vérification de la base de données
    let db_status = check_database_health(&db).await;
    
    // Métriques système
    let system_metrics = get_system_metrics();
    
    // Métriques de performance
    let response_time = start_time.elapsed().as_millis() as u64;
    let performance_metrics = PerformanceMetrics {
        response_time_ms: response_time,
    };
    
    let health_response = HealthResponse {
        status: if db_status.connected { "healthy".to_string() } else { "unhealthy".to_string() },
        timestamp: Utc::now(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: db_status,
        system: system_metrics,
        performance: performance_metrics,
    };
    
    if health_response.status == "healthy" {
        Ok(Json(health_response))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

/// Route d'informations sur l'API
pub async fn info() -> Json<InfoResponse> {
    Json(InfoResponse {
        name: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        description: env!("CARGO_PKG_DESCRIPTION").to_string(),
        authors: env!("CARGO_PKG_AUTHORS").split(':').map(|s| s.trim().to_string()).collect(),
        endpoints: vec![
            EndpointInfo {
                path: "/help/health".to_string(),
                method: "GET".to_string(),
                description: "Vérification complète de l'état de santé du système".to_string(),
            },
            EndpointInfo {
                path: "/help/health-light".to_string(),
                method: "GET".to_string(),
                description: "Vérification rapide (DB + performance seulement)".to_string(),
            },
            EndpointInfo {
                path: "/help/info".to_string(),
                method: "GET".to_string(),
                description: "Informations sur l'API".to_string(),
            },
            EndpointInfo {
                path: "/help/ping".to_string(),
                method: "GET".to_string(),
                description: "Test de connectivité simple".to_string(),
            },
        ],
    })
}

/// Route de ping simple
pub async fn ping() -> &'static str {
    "pong"
}

/// Route de health légère (juste DB + performance)
pub async fn health_light(State(db): State<DatabaseManager>) -> Result<Json<HealthResponse>, StatusCode> {
    let start_time = Instant::now();
    
    // Vérification de la base de données seulement
    let db_status = check_database_health(&db).await;
    
    // Métriques système minimales
    let system_metrics = SystemMetrics {
        cpu_usage: 0.0, // Skip CPU check for speed
        cpu_count: 0,
        memory_used_mb: 0,
        memory_total_mb: 0,
        memory_usage_percent: 0.0,
        disk_usage_percent: 0.0,
        uptime: System::uptime(),
    };
    
    // Métriques de performance
    let response_time = start_time.elapsed().as_millis() as u64;
    let performance_metrics = PerformanceMetrics {
        response_time_ms: response_time,
    };
    
    let health_response = HealthResponse {
        status: if db_status.connected { "healthy".to_string() } else { "unhealthy".to_string() },
        timestamp: Utc::now(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        database: db_status,
        system: system_metrics,
        performance: performance_metrics,
    };
    
    if health_response.status == "healthy" {
        Ok(Json(health_response))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}

/// Vérification de l'état de la base de données
async fn check_database_health(db: &DatabaseManager) -> DatabaseStatus {
    let start_time = Instant::now();
    
    match sqlx::query("SELECT 1 as test")
        .fetch_one(db.get_pool())
        .await
    {
        Ok(_) => DatabaseStatus {
            connected: true,
            response_time_ms: Some(start_time.elapsed().as_millis() as u64),
            error: None,
        },
        Err(e) => DatabaseStatus {
            connected: false,
            response_time_ms: None,
            error: Some(e.to_string()),
        },
    }
}

/// Collecte des métriques système (optimisée)
fn get_system_metrics() -> SystemMetrics {
    let mut sys = System::new();
    
    // Refresh seulement la mémoire et CPU (plus rapide)
    sys.refresh_cpu_usage();
    sys.refresh_memory();
    
    // CPU usage (moyenne de tous les cœurs)
    let cpu_usage = if !sys.cpus().is_empty() {
        sys.cpus().iter()
            .map(|cpu| cpu.cpu_usage())
            .sum::<f32>() / sys.cpus().len() as f32
    } else {
        0.0
    };
    
    let cpu_count = sys.cpus().len();
    
    // Mémoire
    let memory_used = sys.used_memory() / 1024 / 1024; // Convert to MB
    let memory_total = sys.total_memory() / 1024 / 1024; // Convert to MB
    let memory_usage_percent = if memory_total > 0 {
        (memory_used as f32 / memory_total as f32) * 100.0
    } else {
        0.0
    };
    
    // Disques - seulement pour le premier disque (plus rapide)
    let disks = Disks::new_with_refreshed_list();
    let disk_usage_percent = if let Some(disk) = disks.first() {
        let total = disk.total_space();
        let available = disk.available_space();
        if total > 0 {
            let used = total - available;
            (used as f32 / total as f32) * 100.0
        } else {
            0.0
        }
    } else {
        0.0
    };
    
    SystemMetrics {
        cpu_usage,
        cpu_count,
        memory_used_mb: memory_used,
        memory_total_mb: memory_total,
        memory_usage_percent,
        disk_usage_percent,
        uptime: System::uptime(),
    }
}

/// Créer le routeur pour les routes d'aide
pub fn router() -> Router<DatabaseManager> {
    Router::new()
        .route("/help/health", get(health_check))
        .route("/help/info", get(info))
        .route("/help/ping", get(ping))
        .route("/help/health-light", get(health_light))
} 