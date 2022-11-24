use std::collections::HashMap;

use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};

async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let mut labels = HashMap::new();
    labels.insert("application".to_string(), "kfcore".to_string());
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .const_labels(labels)
        .build()
        .unwrap();

        HttpServer::new(move || {
            App::new()
                .wrap(prometheus.clone())
                .service(web::resource("/health").to(health))
        })
        // curl -i localhost:8080/health
        .bind("127.0.0.1:8080")?
        .run()
        .await?;
    Ok(())
}
