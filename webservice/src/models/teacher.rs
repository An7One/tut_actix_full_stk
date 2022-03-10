use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Teacher {
    pub id: i32,
    pub name: String,
    pub url_picture: String,
    pub profile: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TeacherCreation {
    pub name: String,
    pub url_picture: String,
    pub profile: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TeacherUpdate {
    pub name: Option<String>,
    pub url_picture: Option<String>,
    pub profile: Option<String>,
}

impl From<web::Json<TeacherCreation>> for TeacherCreation {
    fn from(new_teacher: web::Json<TeacherCreation>) -> Self {
        TeacherCreation {
            name: new_teacher.name.clone(),
            url_picture: new_teacher.url_picture.clone(),
            profile: new_teacher.profile.clone(),
        }
    }
}

impl From<web::Json<TeacherUpdate>> for TeacherUpdate {
    fn from(teacher_update: web::Json<TeacherUpdate>) -> Self {
        TeacherUpdate {
            name: teacher_update.name.clone(),
            url_picture: teacher_update.url_picture.clone(),
            profile: teacher_update.profile.clone(),
        }
    }
}
