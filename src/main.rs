use std::{path::PathBuf, sync::Arc};

use axum::{extract::Path, routing::get};
use rspc::{Config, Router};

fn mount() -> Arc<Router<(), ()>> {
    <rspc::Router>::new()
        .config(
            Config::new().export_ts_bindings(
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("./bindings.ts"),
            ),
        )
        // Basic query
        .query("version", |t| {
            t(|_, _: ()| async move { env!("CARGO_PKG_VERSION") })
        })
        .build()
        .arced() // This function is a shortcut to wrap the router in an `Arc`.
}

#[tokio::main]
async fn main() {
    let router = mount();

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello world!!!" }))
        .route(
            "/rspc/:id",
            router
                .endpoint(|path: Path<String>| {
                    println!("Client requested operation '{}'", *path);
                    ()
                })
                .axum(),
        );

    let addr = "[::]:8080".parse::<std::net::SocketAddr>().unwrap();
    println!("listening on http://{}/", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
