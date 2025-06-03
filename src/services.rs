use crate::model::TaskModel;
use crate::{AppState, schema::CreateTaskSchema};
use actix_web::{
    HttpResponse, Responder, get, post, web,
    web::{Data, Json, scope},
};
use serde_json::json;

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

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = scope("/api").service(health).service(create_task);

    conf.service(scope);
}
