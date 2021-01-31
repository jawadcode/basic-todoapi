use crate::types::*;
use warp::http;

pub async fn add_todo(todo: Todo, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    store.todolist.write().push(todo);

    Ok(warp::reply::with_status(
        format!("Created Todo #{}", store.todolist.read().len() - 1),
        http::StatusCode::CREATED,
    ))
}

pub async fn get_todos(store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&store.todolist.read().clone()))
}

pub async fn get_todo(id: usize, store: Store) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let todos = store.todolist.read().clone();
    let todo = todos.get(id);

    if let None = todo {
        return Ok(Box::new(warp::reply::with_status(
            "Todo not found",
            http::StatusCode::NOT_FOUND,
        )));
    }

    Ok(Box::new(warp::reply::json(&todo.unwrap())))
}

pub async fn update_todo(
    id: usize,
    payload: UpdateTodo,
    store: Store,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let mut todo_write = store.todolist.write();
    let todo = todo_write.get_mut(id);

    if let None = todo {
        return Ok(Box::new(warp::reply::with_status(
            "Todo not found",
            http::StatusCode::NOT_FOUND,
        )));
    }

    let todo = todo.unwrap();

    todo.title = payload.title;
    todo.description = payload.description;

    Ok(Box::new(warp::reply::json(&todo)))
}

pub async fn delete_todo(id: usize, store: Store) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let todoexists = match store.todolist.read().get(id) {
        Some(_) => true,
        None => false,
    };

    if !todoexists {
        return Ok(Box::new(warp::reply::with_status(
            "Todo not found",
            http::StatusCode::NOT_FOUND,
        )));
    }

    store.todolist.write().remove(id);

    Ok(Box::new(warp::reply::with_status(
        format!("Deleted Todo #{}", id),
        http::StatusCode::OK,
    )))
}

pub async fn toggle_complete(
    id: usize,
    store: Store,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let mut todo_write = store.todolist.write();
    let todo = todo_write.get_mut(id);

    if let None = todo {
        return Ok(Box::new(warp::reply::with_status(
            "Todo not found",
            http::StatusCode::NOT_FOUND,
        )));
    }

    let mut todo = todo.unwrap();

    todo.complete = !todo.complete;

    Ok(Box::new(warp::reply::json(&todo)))
}

pub async fn filter_todos(
    filter: String,
    store: Store,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let todos = store.todolist.read().clone();
    let filtered: Vec<Todo>;
    match filter.as_str() {
        "completed" => {
            filtered = todos.into_iter().filter(|t| t.complete).collect();
        }
        "incomplete" => {
            filtered = todos.into_iter().filter(|t| !t.complete).collect();
        }
        _ => {
            return Ok(Box::new(warp::reply::with_status(
                format!(""),
                http::StatusCode::NOT_FOUND,
            )));
        }
    };
    Ok(Box::new(warp::reply::json(&filtered)))
}

pub async fn search_todos(
    search: Search,
    store: Store,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    if let None = search.query {
        return Ok(Box::new(warp::reply::with_status(
            "Missing 'query' query parameter",
            http::StatusCode::BAD_REQUEST,
        )));
    }

    let search_query = search.query.unwrap();
    let search_filter = search.filter.unwrap_or(String::from("")).to_lowercase();

    let todos = store.todolist.read().clone();

    let results: Vec<Todo> = todos
        .into_iter()
        .filter(|t| {
            (t.title.to_lowercase().contains(&search_query) 
            ||  t.description.to_lowercase().contains(&search_query))
            &&  match search_filter.as_str() {
                    "completed" => t.complete,
                    "incomplete" => !t.complete,
                    _ => true,
                }
        })
        .take(search.limit.unwrap_or(std::usize::MAX))
        .collect();

    Ok(Box::new(warp::reply::json(&results)))
}
