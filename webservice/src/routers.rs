use crate::handlers::{course::*, general::*};
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routers(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("/", web::post().to(create_one_course))
            .route("/{teacher_id}", web::get().to(get_courses_for_teacher))
            .route(
                "/{teacher_id}/{course_id}",
                web::get().to(get_one_course_detail),
            )
            .route(
                "/{teacher_id}/{course_id}",
                web::delete().to(delete_one_course),
            )
            .route(
                "/{teacher_id}/{course_id}",
                web::put().to(update_one_course_detail),
            ),
    );
}
