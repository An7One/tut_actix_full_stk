use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

use crate::errors::MyError;

#[derive(sqlx::FromRow, Clone, Debug, Serialize)]
pub struct Course {
    pub teacher_id: i32,
    pub id: i32,
    pub name: String,
    pub time: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CourseCreation {
    pub teacher_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CourseUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

// impl From<web::Json<CourseCreation>> for CourseCreation {
//     fn from(course: web::Json<CourseCreation>) -> Self {
//         CourseCreation {
//             teacher_id: course.teacher_id,
//             name: course.name.clone(),
//             description: course.description.clone(),
//             format: course.format.clone(),
//             structure: course.structure.clone(),
//             duration: course.duration.clone(),
//             price: course.price,
//             language: course.language.clone(),
//             level: course.level.clone(),
//         }
//     }
// }

impl TryFrom<web::Json<CourseCreation>> for CourseCreation {
    type Error = MyError;
    fn try_from(course: web::Json<CourseCreation>) -> Result<Self, Self::Error> {
        Ok(CourseCreation {
            teacher_id: course.teacher_id,
            name: course.name.clone(),
            description: course.description.clone(),
            format: course.format.clone(),
            structure: course.structure.clone(),
            duration: course.duration.clone(),
            price: course.price,
            language: course.language.clone(),
            level: course.level.clone(),
        })
    }
}

impl From<web::Json<CourseUpdate>> for CourseUpdate {
    fn from(course: web::Json<CourseUpdate>) -> Self {
        CourseUpdate {
            name: course.name.clone(),
            description: course.description.clone(),
            format: course.format.clone(),
            structure: course.structure.clone(),
            duration: course.duration.clone(),
            price: course.price,
            language: course.language.clone(),
            level: course.level.clone(),
        }
    }
}
