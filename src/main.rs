#![allow(unused)]
use std::{fmt::format, sync::Arc};

// Silence unused warning while exploring (To be removed)
use anyhow::Result;
use router::Router;

mod router;

#[tokio::main]
async fn main() -> Result<()> {
    let mut router = Router::new()
        .add_handler("add", add)
        .add_handler("sub", sub)
        .add_handler("multiply", |a: i32, b: i32| async move {
            Ok(format!("{a} * {b} = {}", a * b))
        });

    let router_arc = Arc::new(router);
    let router = router_arc.clone();
    tokio::task::spawn(async move {
        if let Ok(handler) = router.get("add") {
            println!("->>!!!!!!! {:?}", handler.call((4, 3)).await)
        }
    });

    let router = router_arc.clone();
    println!("->>!!!! {}", router.get("add")?.call((4, 3)).await?);
    println!("->>!!!! {}", router.get("sub")?.call((4, 3)).await?);
    println!("->>!!!! {}", router.get("multiply")?.call((4, 3)).await?);

    Ok(())
}

async fn add(a: i32, b: i32) -> Result<String> {
    Ok(format!("{a} + {b} = {}", a + b))
}

async fn sub(a: i32, b: i32) -> Result<String> {
    Ok(format!("{a} - {b} = {}", a - b))
}
