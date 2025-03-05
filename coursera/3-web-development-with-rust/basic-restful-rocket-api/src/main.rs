#[macro_use] extern crate rocket;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use std::sync::Mutex;
use rocket::State;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(Clone)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

struct TaskList {
    tasks: Mutex<Vec<Task>>,
}

// post /tasks - Create a new task
#[post("/tasks", format="json", data="<task>")]
fn create_task(task: Json<Task>, state: &State<TaskList>) -> Result<status::Created<Json<Task>>, Status> {
    let mut tasks = state.tasks.lock().unwrap();
    let new_task = task.into_inner();

    // Verifica se já existe uma task com o mesmo ID
    if tasks.iter().any(|t| t.id == new_task.id) {
        return Err(Status::Conflict); // Retorna 409 Conflict se o ID já existir
    }

    let task_id = new_task.id;
    tasks.push(new_task.clone());

    Ok(status::Created::new(format!("/tasks/{}", task_id)) // Retorna a URL da nova tarefa
        .body(Json(new_task)))      // Retorna a própria tarefa criada
}



#[get("/tasks")]
fn get_tasks(state: &State<TaskList>) -> Json<Vec<Task>> {
    let tasks = state.tasks.lock().unwrap();
    Json(tasks.clone())
}


#[get("/tasks/<id>")]
fn get_task(id: usize, state: &State<TaskList>) -> Result<Json<Task>, Status> {
    let tasks = state.tasks.lock().unwrap();
    match tasks.iter().find(|t| t.id == id) {
        Some(task) => Ok(Json(task.clone())), // Retorna 200 OK
        None => Err(Status::NotFound),        // Retorna 404 Not Found
    }
}


#[put("/tasks/<id>", format="json", data="<updated_task>")]
fn update_task(id: usize, updated_task: Json<Task>, state: &State<TaskList>) -> Result<Json<Task>, Status> {
    let mut tasks = state.tasks.lock().unwrap();
    match tasks.iter_mut().find(|t| t.id == id) {
        Some(task) => {
            *task = updated_task.into_inner();
            Ok(Json(task.clone())) // Retorna a tarefa atualizada com 200 OK
        }
        None => Err(Status::NotFound), // Retorna 404 Not Found se o ID não existir
    }
}

#[delete("/tasks/<id>")]
fn delete_task(id: usize, state: &State<TaskList>) -> Result<Status, Status> {
    let mut tasks = state.tasks.lock().unwrap();
    match tasks.iter().position(|t| t.id == id) {
        Some(index) => {
            tasks.remove(index);
            Ok(Status::NoContent) // 204 No Content: sucesso sem resposta no corpo
        }
        None => Err(Status::NotFound), // 404 Not Found se o ID não existir
    }
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(TaskList {tasks: Mutex::new(Vec::new()),})
        .mount("/", routes![create_task, get_tasks, get_task, update_task, delete_task])
}
