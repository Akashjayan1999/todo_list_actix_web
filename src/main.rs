use actix_web::{web::{self, Json}, HttpResponse, Responder};
use sqlx::{self,SqlitePool,prelude::*};
fn main() {
    println!("Hello, world!");
}

async fn db() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite://db.sqlite").await.unwrap();
    pool.execute("CREATE TABLE IF NOT EXISTS todo (id INTEGER PRIMARY KEY AUTOINCREMENT, content TEXT NOT NULL)").await.unwrap();
    pool

}


async fn get_todo_list(pool:web::Data<SqlitePool>) -> impl Responder {
    let todos :Vec<Todo> = sqlx::query_as("SELECT * FROM todos").fetch_all(pool.get_ref()).await.unwrap();
    let todos_json = serde_json::to_string(&todos).unwrap();
    HttpResponse::Ok().body(todos_json)
}

async fn add_todo(todo:Json<TodoRequest>, pool:web::Data<SqlitePool>) -> impl Responder {
    sqlx::query("INSERT INTO todos (content) VALUES (?1)").bind(&todo.content).execute(pool.get_ref()).await.unwrap();
    HttpResponse::Ok().body("Todo added")
}

async fn get_single_todo(id:web::Path<i32>, pool:web::Data<SqlitePool>) -> impl Responder {
    let id = id.into_inner();
    let row :Vec<Todo> = sqlx::query_as("SELECT * FROM todos WHERE id = ?1").bind(&id).fetch_all(pool.get_ref()).await.unwrap();
    if(row.len() == 0){
        let msg = format!("Todo with id {} not found", id);
        return HttpResponse::NotFound().body(msg);
    }else{
        let todo_json = serde_json::to_string(&row[0]).unwrap();
        HttpResponse::Ok().body(todo_json)
    }
    
}


async fn delete_todo(id:web::Path<i32>, pool:web::Data<SqlitePool>) -> impl Responder {
    let id = id.into_inner();
    let row :Vec<Todo> = sqlx::query_as("SELECT * FROM todos WHERE id = ?1").bind(&id).fetch_all(pool.get_ref()).await.unwrap();
    if(row.len() == 0){
        let msg = format!("Todo with id {} not found", id);
        return HttpResponse::NotFound().body(msg);
    }else{
         sqlx::query("DELETE FROM todos WHERE id = ?1").bind(&id).execute(pool.get_ref()).await.unwrap();
         HttpResponse::Ok().body("Todo deleted")
    }
    
}

#[derive(FromRow, serde::Serialize)]
struct Todo {
    id: i64,
    content: String,
}

#[derive(serde::Deserialize)]
struct TodoRequest {
    content: String,
}
