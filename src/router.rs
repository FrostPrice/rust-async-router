use anyhow::{anyhow, Result};
use futures::{future::BoxFuture, Future};
use std::{collections::HashMap, hash::Hash};

type HandlerArgs = (i32, i32);
type HandlerResult = Result<String>;

pub struct Handler {
    func: Box<dyn Fn(HandlerArgs) -> BoxFuture<'static, HandlerResult> + Send + Sync + 'static>,
}

impl Handler {
    fn new<P>(raw_func: fn(a: i32, b: i32) -> P) -> Handler
    where
        P: Future<Output = HandlerResult> + Send + 'static,
    {
        Handler {
            func: Box::new(move |(a, b)| Box::pin(raw_func(a, b))),
        }
    }

    pub async fn call(&self, args: HandlerArgs) -> HandlerResult {
        (self.func)(args).await
    }
}

pub struct Router {
    handlers: HashMap<String, Handler>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn add_handler<P>(mut self, name: &str, fun: fn(i32, i32) -> P) -> Self
    where
        P: Future<Output = HandlerResult> + Send + 'static,
    {
        self.handlers.insert(name.to_string(), Handler::new(fun));
        self
    }

    pub fn get(&self, name: &str) -> Result<&Handler> {
        self.handlers
            .get(name)
            .ok_or_else(|| anyhow!("No handler for {name}"))
    }
}
