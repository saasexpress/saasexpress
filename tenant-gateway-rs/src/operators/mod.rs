use axum::{routing::get_service, Router};
use log::{error, info};

use registry::Registry;
use singleton::get_instance;
use singleton_ro::get_instance_ro;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

pub mod enums;
pub mod singleton;
pub mod singleton_ro;
//pub mod fanout2;
// pub mod a;
// pub mod b;
pub mod buffer_to_json;
pub mod json_to_buffer;
pub mod operator;
pub mod registry;
// pub mod reverse_proxy;
// pub mod template;
// pub mod httpin;
// pub mod fanout;
// pub mod actor_fanout;

#[derive(Debug, Clone)]
pub struct Settings<T> {
    pub node: T,
}

pub async fn do_it() -> Result<(), std::io::Error> {
    let app = Router::new().nest_service("/static", get_service(ServeDir::new("static")));

    let addr = SocketAddr::from(([0, 0, 0, 0], 2243));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server running at http://{}", addr);

    match env::current_dir() {
        Ok(path) => println!("Current directory: {}", path.display()),
        Err(e) => eprintln!("Error getting current directory: {}", e),
    }

    let a = axum::serve(listener, app).await;
    return a;
}

pub fn register() -> Registry {
    {
        let mut singleton = get_instance().lock().unwrap();
        singleton.set_value("Updated Singleton!");
    }

    let singleton = get_instance().lock().unwrap();

    info!("Singleton value = {}", singleton.value);

    let singleton_ro = get_instance_ro();

    info!("Singleton RO value = {}", singleton_ro.value);

    let mut registry = Registry::new();

    // registry.register(Box::new(httpin::HTTPIn::new(12)));
    // registry.register(Box::new(reverse_proxy::ReverseProxy { width: 12 }));
    // registry.register(Box::new(template::Template { width: 12 }));
    registry.register(Box::new(buffer_to_json::BufferToJSON { width: 12 }));
    registry.register(Box::new(json_to_buffer::JSONToBuffer { width: 12 }));
    return registry;
}
