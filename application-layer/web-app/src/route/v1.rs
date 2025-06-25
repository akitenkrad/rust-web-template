use super::health::build_health_check_router;
use axum::Router;
use registry::AppRegistry;

pub fn routes() -> Router<AppRegistry> {
    let routers = Router::new().merge(build_health_check_router());
    Router::new().nest("/api/v1", routers)
}
