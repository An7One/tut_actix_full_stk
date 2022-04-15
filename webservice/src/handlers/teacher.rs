use crate::dbaccesses::teacher::*;
use crate::errors::MyError;
use crate::models::teacher::{TeacherCreation, TeacherUpdate};
use crate::state::AppState;

use actix_web::{web, HttpResponse};

pub async fn get_all_teachers(app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {
    get_all_teachers_db(&app_state.db)
        .await
        .map(|teachers| HttpResponse::Ok().json(teachers))
}

pub async fn get_one_teacher_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id,) = params.into_inner();
    get_one_teacher_detail_db(&app_state.db, teacher_id)
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn create_one_teacher(
    app_state: web::Data<AppState>,
    new_teacher: web::Json<TeacherCreation>,
) -> Result<HttpResponse, MyError> {
    create_one_teacher_db(&app_state.db, TeacherCreation::from(new_teacher))
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn update_one_teacher_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
    teacher_update: web::Json<TeacherUpdate>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id,) = params.into_inner();
    update_one_teacher_detail_db(
        &app_state.db,
        teacher_id,
        TeacherUpdate::from(teacher_update),
    )
    .await
    .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn delete_one_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    delete_one_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
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
    async fn get_all_teachers_should_succeed() {
        dotenv().ok();
        let url_db = env::var("DATABASE_URL").expect("DATABASE_URL is missing");
        let pool_db = PgPoolOptions::new().connect(&url_db).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_owned(),
            visit_count: Mutex::new(0),
            db: pool_db,
        });
        let resp = get_all_teachers(app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn get_one_teacher_detail_should_succeed() {
        dotenv().ok();
        let url_db = env::var("DATABASE_URL").expect("DATABASE_URL is missing");
        let pool_db = PgPoolOptions::new().connect(&url_db).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_owned(),
            visit_count: Mutex::new(0),
            db: pool_db,
        });
        let params: web::Path<(i32,)> = web::Path::from((1,));
        let resp = get_one_teacher_detail(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn create_one_teacher_should_succeed() {
        dotenv().ok();
        let url_db = env::var("DATABASE_URL").expect("DATABASE_URL is missing");
        let pool_db = PgPoolOptions::new().connect(&url_db).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_owned(),
            visit_count: Mutex::new(0),
            db: pool_db,
        });
        let new_teacher = TeacherCreation {
            name: "Third Teacher".into(),
            url_picture: "http://yanglyu.pro".into(),
            profile: "A teacher".into(),
        };
        let params: web::Json<TeacherCreation> = web::Json(new_teacher);
        let resp = create_one_teacher(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn delete_one_teacher_should_succeed() {
        dotenv().ok();
        let url_db = env::var("DATABASE_URL").expect("DATABASE_URL is missing");
        let pool_db = PgPoolOptions::new().connect(&url_db).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_owned(),
            visit_count: Mutex::new(0),
            db: pool_db,
        });
        let params: web::Path<i32> = web::Path::from(1);
        let resp = delete_one_teacher(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
