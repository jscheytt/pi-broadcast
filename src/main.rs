// See https://github.com/seanmonstar/warp/blob/master/examples/websockets_chat.rs

use std::{env, fs, sync::Arc};

use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;
use warp::Filter;

mod admin;
mod ws;

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
    // Get port to run on from CLI arguments
    let args: Vec<String> = env::args().collect();
    let port: u16 = match args.get(1) {
        Some(i) => i.parse().expect("Could not convert string to integer"),
        None => 80, // default value
    };

    // Keep track of all connected users, key is usize, value is a websocket sender
    let users = ws::Users::default();
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
            ws.on_upgrade(move |socket| ws::user_connected(socket, users))
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

    let admin = warp::path("admin")
        .map(move || WithTemplate {
            name: "admin.html",
            value: json!({
                "titles": admin::audio_names(),
                "current_dir": admin::current_path(),
            }),
        })
        .map(handlebars);

    let routes = home
        .or(assets)
        .or(media)
        .or(admin)
        .or(websockets)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
