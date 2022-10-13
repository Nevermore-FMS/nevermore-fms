use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;
use std::{path::Path, sync::Arc};

use warp::{Filter, Reply, Rejection};

use crate::{field::Field, plugin::PluginManager};

struct WithTemplate<T: Serialize> {
    name: &'static str,
    value: T,
}

pub async fn handle_index(args: (PluginManager, Arc<Handlebars<'_>>)) -> Result<impl Reply, Rejection> {
    let (plugin_manager, hb) = args;
    Ok(render(
        WithTemplate {
            name: "index.hbl",
            value: json!({
                "plugins": plugin_manager.get_plugins_metadata().await
            }),
        },
        hb,
    ))
}

pub async fn handle_login(args: (PluginManager, Arc<Handlebars<'_>>)) -> Result<impl Reply, Rejection> {
    let (plugin_manager, hb) = args;
    Ok(render(
        WithTemplate {
            name: "login.hbl",
            value: json!({
                "plugins": plugin_manager.get_plugins_metadata().await
            }),
        },
        hb,
    ))
}

fn render<T>(template: WithTemplate<T>, hbs: Arc<Handlebars<'_>>) -> impl warp::Reply
where
    T: Serialize,
{
    let render = hbs
        .render(template.name, &template.value)
        .unwrap_or_else(|err| err.to_string());
    warp::reply::html(render)
}

pub async fn start_web(field: Field, plugin_manager: PluginManager) {
    tokio::spawn(async move {
        let mut reg = Handlebars::new();
        reg.register_template_string("index.hbl", include_str!("./templates/index.hbl"))
            .unwrap();
        reg.register_template_string("login.hbl", include_str!("./templates/login.hbl"))
            .unwrap();

        let static_serve = warp::path("static").and(warp::fs::dir(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("public/static"),
        ));

        let hb = Arc::new(reg);

        //let handlebars = |with_template| async move {render(with_template.await, hb.clone())};

        let index_hb = hb.clone();
        let index_plugin_manager = plugin_manager.clone();
        let index = warp::get()
            .and(warp::path::end())
            .map(move || (index_plugin_manager.clone(), index_hb.clone()))
            .and_then(handle_index);

        let login_hb = hb.clone();
        let login_plugin_manager = plugin_manager.clone();
        let login = warp::get()
            .and(warp::path("login"))
            .map(move || (login_plugin_manager.clone(), login_hb.clone()))
            .and_then(handle_login);

        let route = warp::any().and(index.or(login).or(static_serve));

        warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
    });
}
