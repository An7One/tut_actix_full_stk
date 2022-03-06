use super::db_access::*;
use super::errors::MyError;
use super::models::Course;
use super::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn new_course(
    app_state: web::Data<AppState>,
    new_course: web::Json<Course>,
) -> Result<HttpResponse, MyError> {
    create_one_course_db(&app_state.db, new_course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<(usize,)>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = i32::try_from(params.into_inner().0).unwrap();
    get_courses_for_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_course_details(
    app_state: web::Data<AppState>,
    params: web::Path<(usize, usize)>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = i32::try_from(params.0).unwrap();
    let course_id = i32::try_from(params.1).unwrap();
    get_one_course_detail_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use std::sync::Mutex;
    #[actix_rt::test]
    async fn add_one_course_should_succeed() {
        dotenv().ok();
        let url_db = env::var("DATABASE_URL").expect("DATABASE_URL is missing");
        let pool_db = PgPoolOptions::new().connect(&url_db).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_owned(),
            visit_count: Mutex::new(0),
            db: pool_db,
        });
        let course = web::Json(Course {
            teacher_id: 1,
            name: "Test course".into(),
            id: Some(1),
            time: None,
        });
        let resp = new_course(app_state, course).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn get_courses_by_teacher_should_succeed() {
        dotenv().ok();
        let url_db = env::var("DATABASE_URL").expect("DATABASE_URL is missing");
        let pool_db = PgPoolOptions::new().connect(&url_db).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_owned(),
            visit_count: Mutex::new(0),
            db: pool_db,
        });
        let teacher_id: web::Path<(usize,)> = web::Path::from((1,));
        let resp = get_courses_for_teacher(app_state, teacher_id)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn get_one_course_should_succeed() {
        dotenv().ok();
        let url_db = env::var("DATABASE_URL").expect("DATABASE_URL is missing");
        let pool_db = PgPoolOptions::new().connect(&url_db).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_owned(),
            visit_count: Mutex::new(0),
            db: pool_db,
        });
        let params: web::Path<(usize, usize)> = web::Path::from((1, 1));
        let resp = get_course_details(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn create_one_course_should_succeed() {
        dotenv().ok();
        let url_db = env::var("DATABASE_URL").expect("DATABASE_URL is missing");
        let pool_db = PgPoolOptions::new().connect(&url_db).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_owned(),
            visit_count: Mutex::new(0),
            db: pool_db,
        });
        let course = web::Json(Course {
            teacher_id: 1,
            name: "Test course".into(),
            id: Some(3),
            time: None,
        });
        let resp = new_course(app_state, course).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
