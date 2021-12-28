use crate::controllers::index;
use actix_web::web;
pub fn routes(cfg: &mut web::ServiceConfig) {
  cfg.route("/", web::get().to(index::index));
}
