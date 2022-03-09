use crate::dbaccesses::course::*;
use crate::errors::MyError;
use crate::models::course::{CourseCreation, CourseUpdate};
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn create_one_course(
    app_state: web::Data<AppState>,
    new_course: web::Json<CourseCreation>,
) -> Result<HttpResponse, MyError> {
    create_one_course_db(&app_state.db, new_course.try_into()?)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id,) = params.into_inner();
    get_courses_for_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_one_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    get_one_course_detail_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn delete_one_course(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    delete_one_course_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
}

pub async fn update_one_course_detail(
    app_state: web::Data<AppState>,
    course_update: web::Json<CourseUpdate>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    update_one_course_detail_db(&app_state.db, teacher_id, course_id, course_update.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::{http::StatusCode, ResponseError};
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
        let course = web::Json(CourseCreation {
            teacher_id: 1,
            name: "Test course".into(),
            description: Some("This is a course".into()),
            format: None,
            structure: None,
            duration: None,
            price: None,
            language: Some("English".into()),
            level: Some("Beginner".into()),
        });
        let resp = create_one_course(app_state, course).await.unwrap();
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
        let teacher_id: web::Path<(i32,)> = web::Path::from((1,));
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
        let params: web::Path<(i32, i32)> = web::Path::from((1, 5));
        let resp = get_one_course_detail(app_state, params).await.unwrap();
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
        let course = web::Json(CourseCreation {
            teacher_id: 1,
            name: "Test course".into(),
            description: None,
            format: None,
            structure: None,
            duration: None,
            price: None,
            language: Some("English".into()),
            level: Some("Beginner".into()),
        });
        let resp = create_one_course(app_state, course).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn get_one_course_should_fail() {
        dotenv().ok();
        let url_db = env::var("DATABASE_URL").expect("DATABASE_URL is missing");
        let pool_db = PgPoolOptions::new().connect(&url_db).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_owned(),
            visit_count: Mutex::new(0),
            db: pool_db,
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1, 100));
        let resp = get_one_course_detail(app_state, params).await;
        match resp {
            Ok(_) => println!("Something went wrong..."),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }
}
