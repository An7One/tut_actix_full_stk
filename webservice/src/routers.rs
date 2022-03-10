use crate::handlers::{course::*, general::*, teacher::*};

use actix_web::web;

pub fn routes_general(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn routes_course(cfg: &mut web::ServiceConfig) {
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

pub fn routes_teacher(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/teachers")
            .route("/", web::post().to(create_one_teacher))
            .route("/", web::get().to(get_all_teachers))
            .route("/{teacher_id}", web::get().to(get_one_teacher_detail))
            .route("/{teacher_id}", web::put().to(update_one_teacher_detail))
            .route("/{teacher_id}", web::delete().to(delete_one_teacher)),
    );
}
