// See https://github.com/seanmonstar/warp/blob/master/examples/websockets_chat.rs

use std::{env, fs, path::Path, sync::Arc};

use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;
use warp::Filter;

pub mod lib;

// See https://github.com/seanmonstar/warp/blob/master/examples/handlebars_template.rs

struct WithTemplate<T: Serialize> {
    name: &'static str,
    value: T,
}

fn render<T>(template: WithTemplate<T>, hbs: Arc<Handlebars>) -> impl warp::Reply
where
    T: Serialize,
{
    let render = hbs
        .render(template.name, &template.value)
        .unwrap_or_else(|err| err.to_string());
    warp::reply::html(render)
}

#[tokio::main]
async fn main() {
    // Keep track of all connected users, key is usize, value is a websocket sender
    let users = lib::Users::default();
    // Turn our "state" into a new Filter
    let users = warp::any().map(move || users.clone());

    let home = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("assets/index.html"));

    let assets = warp::path("assets").and(warp::fs::dir("assets"));

    let media = warp::path("media").and(warp::fs::dir("media"));

    let websockets = warp::path("ws")
        .and(warp::ws())
        .and(users)
        .map(|ws: warp::ws::Ws, users| {
            ws.on_upgrade(move |socket| lib::user_connected(socket, users))
        });

    // -- Admin route
    // Read template from file
    let template =
        fs::read_to_string("assets/admin.html").expect("Something went wrong reading the file");

    // Register the template
    let mut hb = Handlebars::new();
    hb.register_template_string("admin.html", template.clone())
        .unwrap();

    // Turn Handlebars instance into a Filter so we can combine it easily with others
    let hb = Arc::new(hb);
    // Create a reusable closure to render template
    let handlebars = move |with_template| render(with_template, hb.clone());

    let current_path = env::current_dir().expect("Could not find directory");
    let current_path = format!("{}/media", current_path.display());

    let admin = warp::path("admin")
        .map(move || WithTemplate {
            name: "admin.html",
            value: json!({
                "titles": audio_names(),
                "current_dir": current_path,
            }),
        })
        .map(handlebars);

    let routes = home
        .or(assets)
        .or(media)
        .or(admin)
        .or(websockets)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn audio_names() -> Vec<String> {
    let paths = fs::read_dir(&Path::new("media")).unwrap();
    let audio_extension = ".mp3";
    paths
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(|s| String::from(s)))
            })
        })
        .filter(|name| name.ends_with(audio_extension))
        .map(|name| name.replace(audio_extension, ""))
        .collect::<Vec<String>>()
}
