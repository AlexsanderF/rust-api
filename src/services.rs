use crate::model::TaskModel;
use crate::{
    AppState,
    schema::{CreateTaskSchema, FilterOptions, UpdateTaskSchema},
};
use actix_web::{
    HttpResponse, Responder, delete, get, patch, post, web,
    web::{Data, Json, Path, Query, scope},
};
use serde_json::json;
use uuid::Uuid;

#[get("/health")]
async fn health() -> impl Responder {
    const MESSAGE: &str = "Health check: API is up and running...";

    HttpResponse::Ok().json(json!({"status": "ok", "message": MESSAGE}))
}

#[post("/tasks")]
async fn create_task(body: Json<CreateTaskSchema>, data: Data<AppState>) -> impl Responder {
    match sqlx::query_as!(
        TaskModel,
        "INSERT INTO tasks (title, content, status, priority) VALUES ($1, $2, $3, $4) RETURNING *",
        body.title.to_string(),
        body.content.to_string(),
        body.status.to_string(),
        body.priority.to_string()
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(task) => {
            let response = json!({
                "status": "success",
                "message": "Task created successfully",
                "task": task
            });
            HttpResponse::Ok().json(response)
        }
        Err(err) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("{:?}", err)
        })),
    }
}

#[get("/tasks")]
async fn get_all_tasks(opts: Query<FilterOptions>, data: Data<AppState>) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    match sqlx::query_as!(
        TaskModel,
        "SELECT * FROM tasks LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    {
        Ok(tasks) => {
            let response = json!({
                "status": "success",
                "result": tasks.len(),
                "tasks": tasks
            });
            HttpResponse::Ok().json(response)
        }
        Err(err) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("{:?}", err)
        })),
    }
}

#[get("/tasks/{id}")]
async fn get_task_by_id(path: Path<Uuid>, data: Data<AppState>) -> impl Responder {
    let task_id = path.into_inner();

    match sqlx::query_as!(TaskModel, "SELECT * FROM tasks WHERE id = $1", task_id)
        .fetch_one(&data.db)
        .await
    {
        Ok(task) => {
            let response = json!({
                "status": "success",
                "task": task
            });
            HttpResponse::Ok().json(response)
        }
        Err(err) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("{:?}", err)
        })),
    }
}

#[delete("/tasks/{id}")]
async fn delete_task_by_id(path: Path<Uuid>, data: Data<AppState>) -> impl Responder {
    let task_id = path.into_inner();

    match sqlx::query_as!(
        TaskModel,
        "DELETE FROM tasks WHERE id = $1 RETURNING *",
        task_id
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Task deleted successfully"
        })),
        Err(err) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("{:?}", err)
        })),
    }
}

#[patch("/tasks/{id}")]
async fn update_task_by_id(
    path: Path<Uuid>,
    body: Json<UpdateTaskSchema>,
    data: Data<AppState>,
) -> impl Responder {
    let task_id = path.into_inner();

    let existing_task =
        match sqlx::query_as!(TaskModel, "SELECT * FROM tasks WHERE id = $1", task_id)
            .fetch_optional(&data.db)
            .await
        {
            Ok(Some(task)) => task,
            Ok(None) => {
                return HttpResponse::NotFound().json(json!({
                    "status": "error",
                    "message": "Task not found"
                }));
            }
            Err(err) => {
                let error_message = format!("Failed to fetch task: {}", err);
                return HttpResponse::InternalServerError().json(json!({
                    "status": "error",
                    "message": error_message
                }));
            }
        };

    let title = body.title.as_deref().unwrap_or(&existing_task.title);
    let content = body.content.as_deref().unwrap_or(&existing_task.content);
    let status = body.status.as_deref().unwrap_or(&existing_task.status);
    let priority = body.priority.as_deref().unwrap_or(&existing_task.priority);

    let result = sqlx::query_as!(
        TaskModel,
        "UPDATE tasks 
         SET title = $1, content = $2, status = $3, priority = $4, updated_at = NOW() 
         WHERE id = $5 
         RETURNING *",
        title,
        content,
        status,
        priority,
        task_id
    )
    .fetch_one(&data.db)
    .await;

    match result {
        Ok(updated_task) => {
            let response = json!({
                "status": "success",
                "message": "Task updated successfully",
                "task": updated_task
            });
            HttpResponse::Ok().json(response)
        }
        Err(err) => {
            let error_message = format!("Failed to update task: {}", err);
            eprintln!("{}", error_message);
            HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": error_message
            }))
        }
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = scope("/api")
        .service(health)
        .service(create_task)
        .service(get_all_tasks)
        .service(get_task_by_id)
        .service(delete_task_by_id)
        .service(update_task_by_id);

    conf.service(scope);
}
