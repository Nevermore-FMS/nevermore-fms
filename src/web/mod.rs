use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{web::{self}, App, HttpServer, cookie::Key};
use handlebars::Handlebars;
use jfs::Store;
use std::{sync::Arc, path::Path, collections::HashMap};

use crate::{field::Field, plugin::PluginManager};

use self::users::{Users, User};

pub mod users;
pub mod routes;

pub async fn start_web(field: Field, plugin_manager: PluginManager) -> anyhow::Result<()> {
    let mut reg = Handlebars::new();
    reg.register_template_string("index.hbl", include_str!("./templates/index.hbl"))?;
    reg.register_template_string("login.hbl", include_str!("./templates/login.hbl"))?;
    reg.register_template_string("plugin_manager.hbl", include_str!("./templates/plugin_manager.hbl"))?;

    let hb = Arc::new(reg);


    let store = Store::new("data").unwrap();

    let mut users: HashMap<String, User> = HashMap::new();

    users.insert("test".to_string(), User::new("Test".to_string(), "test".to_string(), "test".to_string()));

    let users_obj = Users{
        users
    };

    store.save_with_id(&users_obj, "users").unwrap();

    tokio::spawn(HttpServer::new(move || {
            
        App::new()
        .app_data(web::Data::new(field.clone()))
        .app_data(web::Data::new(plugin_manager.clone()))
        .app_data(web::Data::new(hb.clone()))
        .app_data(web::Data::new(store.clone()))
        .wrap(SessionMiddleware::new(CookieSessionStore::default(), Key::generate()))
        .service(routes::index)
        .service(routes::login_ui)
        .service(routes::login)
        .service(routes::plugin_manager_ui)
        .service(actix_files::Files::new("/static", Path::new(env!("CARGO_MANIFEST_DIR")).join("public/static")))
    })
    .bind(("127.0.0.1", 8080))?
    .run());
    Ok(())
}
