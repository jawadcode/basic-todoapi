use pretty_env_logger;
use warp::Filter;

mod handlers;
mod types;

use types::*;

// The usual size limit for a request body is 16kb so I just went with that
const MAX_CONTENT: u64 = 1024 * 16;

fn parse_post() -> impl Filter<Extract = (Todo,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(MAX_CONTENT).and(warp::body::json())
}

fn parse_patch() -> impl Filter<Extract = (UpdateTodo,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(MAX_CONTENT).and(warp::body::json())
}

#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    pretty_env_logger::init();

    // GET "/"
    let help = warp::get()
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::fs::dir("./static"));
    // POST "/api/todos"
    let add_todo = warp::path!("api" / "todos")
        .and(warp::post())
        .and(parse_post())
        .and(store_filter.clone())
        .and_then(handlers::add_todo);
    // GET "/api/todos"
    let get_todos = warp::path!("api" / "todos")
        .and(warp::get())
        .and(store_filter.clone())
        .and_then(handlers::get_todos);
    // GET "/api/todos/:id"
    let get_todo = warp::path!("api" / "todos" / usize)
        .and(store_filter.clone())
        .and(warp::get())
        .and_then(handlers::get_todo);
    // PATCH "/api/todos/:id"
    let patch_todo = warp::path!("api" / "todos" / usize)
        .and(warp::patch())
        .and(parse_patch())
        .and(store_filter.clone())
        .and_then(handlers::update_todo);
    // DELETE "/api/todos/:id"
    let delete_todo = warp::path!("api" / "todos" / usize)
        .and(store_filter.clone())
        .and(warp::delete())
        .and_then(handlers::delete_todo);
    // PATCH "/api/todos/:id/toggle"
    let toggle_todo = warp::path!("api" / "todos" / usize / "toggle")
        .and(store_filter.clone())
        .and(warp::patch())
        .and_then(handlers::toggle_complete);
    // GET "/api/todos/search?query=<query>&limit=<optional limit>&filter=<optional filter>"
    let search_todos = warp::path!("api" / "todos" / "search")
        .and(warp::get())
        .and(warp::query::<Search>())
        .and(store_filter.clone())
        .and_then(handlers::search_todos);
    // GET "/api/todos/:filter"
    let filter_todos = warp::path!("api" / "todos" / String)
        .and(store_filter.clone())
        .and(warp::get())
        .and_then(handlers::filter_todos);

    let routes = help
        .or(add_todo)
        .or(get_todos)
        .or(get_todo)
        .or(patch_todo)
        .or(delete_todo)
        .or(toggle_todo)
        .or(search_todos)
        .or(filter_todos)
        .with(warp::log("todoapi"));

    warp::serve(routes).run(([0, 0, 0, 0], 5000)).await;
}
