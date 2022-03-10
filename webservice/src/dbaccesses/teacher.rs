use crate::errors::MyError;
use crate::models::teacher::{Teacher, TeacherCreation, TeacherUpdate};
use sqlx::postgres::PgPool;

pub async fn get_all_teachers_db(pool: &PgPool) -> Result<Vec<Teacher>, MyError> {
    let rows = sqlx::query!("SELECT * FROM teacher")
        .fetch_all(pool)
        .await?;
    let teachers: Vec<Teacher> = rows
        .iter()
        .map(|r| Teacher {
            id: r.id,
            name: r.name.clone().unwrap_or_default(),
            url_picture: r.url_picture.clone().unwrap_or_default(),
            profile: r.profile.clone().unwrap_or_default(),
        })
        .collect();
    match teachers.len() {
        0 => Err(MyError::NotFound("No teachers found".into())),
        _ => Ok(teachers),
    }
}

pub async fn get_one_teacher_detail_db(pool: &PgPool, teacher_id: i32) -> Result<Teacher, MyError> {
    let row = sqlx::query!(
        r#"
        SELECT * FROM teacher WHERE id = $1
        "#,
        teacher_id
    )
    .fetch_one(pool)
    .await
    .map(|r| Teacher {
        id: r.id,
        name: r.name.unwrap_or_default(),
        url_picture: r.url_picture.unwrap_or_default(),
        profile: r.profile.unwrap_or_default(),
    })
    .map_err(|_err| MyError::NotFound("Teacher ID not found".into()))?;
    Ok(row)
}

pub async fn create_one_teacher_db(
    pool: &PgPool,
    new_teacher: TeacherCreation,
) -> Result<Teacher, MyError> {
    let row = sqlx::query!(
        r#"
        INSERT INTO teacher (name, url_picture, profile)
        VALUES ($1, $2, $3)
        RETURNING id, name, url_picture, profile
        "#,
        new_teacher.name,
        new_teacher.url_picture,
        new_teacher.profile
    )
    .fetch_one(pool)
    .await?;
    Ok(Teacher {
        id: row.id,
        name: row.name.unwrap_or_default(),
        url_picture: row.url_picture.unwrap_or_default(),
        profile: row.profile.unwrap_or_default(),
    })
}

pub async fn update_one_teacher_detail_db(
    pool: &PgPool,
    teacher_id: i32,
    teacher_update: TeacherUpdate,
) -> Result<Teacher, MyError> {
    let row = sqlx::query!(
        r#"
        SELECT * FROM teacher WHERE id = $1
        "#,
        teacher_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| MyError::NotFound("Teacher ID not found".into()))?;
    let tmp = Teacher {
        id: row.id,
        name: if let Some(name) = teacher_update.name {
            name
        } else {
            row.name.unwrap_or_default()
        },
        url_picture: if let Some(url) = teacher_update.url_picture {
            url
        } else {
            row.url_picture.unwrap_or_default()
        },
        profile: if let Some(profile) = teacher_update.profile {
            profile
        } else {
            row.profile.unwrap_or_default()
        },
    };
    let updated_row = sqlx::query!(
        r#"
        UPDATE teacher SET name = $1, url_picture=$2, profile=$3 WHERE id = $4
        RETURNING id, name, url_picture, profile
        "#,
        tmp.name,
        tmp.url_picture,
        tmp.profile,
        teacher_id,
    )
    .fetch_one(pool)
    .await
    .map(|r| Teacher {
        id: r.id,
        name: r.name.unwrap_or_default(),
        url_picture: r.url_picture.unwrap_or_default(),
        profile: r.profile.unwrap_or_default(),
    })
    .map_err(|_err| MyError::NotFound("Teacher ID not found".into()))?;
    Ok(updated_row)
}

pub async fn delete_one_teacher_db(pool: &PgPool, teacher_id: i32) -> Result<String, MyError> {
    let row = sqlx::query!(
        r#"
        DELETE FROM teacher WHERE id = $1
        "#,
        teacher_id
    )
    .execute(pool)
    .await
    .map_err(|_err| MyError::DBError("Unable to delete the teacher".into()))?;
    Ok(format!("Deleted {:?} record(s)", row))
}
