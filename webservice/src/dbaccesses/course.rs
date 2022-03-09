use crate::errors::MyError;
use crate::models::course::{Course, CourseCreation, CourseUpdate};
use sqlx::postgres::PgPool;

pub async fn get_courses_for_teacher_db(
    pool: &PgPool,
    teacher_id: i32,
) -> Result<Vec<Course>, MyError> {
    let rows: Vec<Course> = sqlx::query_as!(
        Course,
        "SELECT * FROM course WHERE teacher_id = $1",
        teacher_id
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_one_course_detail_db(
    pool: &PgPool,
    teacher_id: i32,
    course_id: i32,
) -> Result<Course, MyError> {
    let row: Option<Course> = sqlx::query_as!(
        Course,
        r#"SELECT * FROM course
        WHERE teacher_id = $1 and id = $2"#,
        teacher_id,
        course_id
    )
    .fetch_optional(pool)
    .await?;
    if let Some(course) = row {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course not found".into()))
    }
}

pub async fn create_one_course_db(
    pool: &PgPool,
    new_course: CourseCreation,
) -> Result<Course, MyError> {
    let row = sqlx::query_as!(
        Course,
        r#"INSERT INTO course (teacher_id, name, description, format, structure, duration, price, language, level)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id, teacher_id, name, time, description, format, structure, duration, price, language, level"#,
        new_course.teacher_id,
        new_course.name,
        new_course.description,
        new_course.format,
        new_course.structure,
        new_course.duration,
        new_course.price,
        new_course.language,
        new_course.level,
    )
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn delete_one_course_db(
    pool: &PgPool,
    teacher_id: i32,
    id: i32,
) -> Result<String, MyError> {
    let row = sqlx::query!(
        "DELETE FROM course WHERE teacher_id = $1 and id = $2",
        teacher_id,
        id,
    )
    .execute(pool)
    .await?;
    Ok(format!("Deleted {:?} record", row))
}

pub async fn update_one_course_detail_db(
    pool: &PgPool,
    teacher_id: i32,
    id: i32,
    course_update: CourseUpdate,
) -> Result<Course, MyError> {
    let current_course_row = sqlx::query_as!(
        Course,
        "SELECT * FROM course WHERE teacher_id = $1 and id = $2",
        teacher_id,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| MyError::NotFound("Course not found".into()))?;
    let name: String = if let Some(name) = course_update.name {
        name
    } else {
        current_course_row.name
    };
    let description: String = if let Some(description) = course_update.description {
        description
    } else {
        current_course_row.description.unwrap_or_default()
    };
    let language: String = if let Some(language) = course_update.language {
        language
    } else {
        current_course_row.language.unwrap_or_default()
    };
    let format: String = if let Some(format) = course_update.format {
        format
    } else {
        current_course_row.format.unwrap_or_default()
    };
    let structure: String = if let Some(structure) = course_update.structure {
        structure
    } else {
        current_course_row.structure.unwrap_or_default()
    };
    let duration: String = if let Some(duration) = course_update.duration {
        duration
    } else {
        current_course_row.duration.unwrap_or_default()
    };
    let level: String = if let Some(level) = course_update.level {
        level
    } else {
        current_course_row.level.unwrap_or_default()
    };
    let price = if let Some(price) = course_update.price {
        price
    } else {
        current_course_row.price.unwrap_or_default()
    };
    let course_row = sqlx::query_as!(
        Course,
        r#"
        UPDATE course SET name = $1, description = $2, format = $3, structure = $4, duration = $5, price = $6, language = $7, level = $8
        WHERE teacher_id = $9 and id = $10
        RETURNING id, teacher_id, name, time, description, format, structure, duration, price, language, level
        "#,
        name,
        description,
        format,
        structure,
        duration,
        price,
        language,
        level,
        teacher_id,
        id
    )
    .fetch_one(pool)
    .await;
    if let Ok(course) = course_row {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course not found".into()))
    }
}
