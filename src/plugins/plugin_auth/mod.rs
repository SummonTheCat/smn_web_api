use async_trait::async_trait;
use hyper::{Body, Request, Response};
use smn_web_core::structs::struct_plugin::Plugin;
use std::convert::Infallible;

pub struct PluginAuth {
}

impl PluginAuth {
    pub fn new() -> Self {
        Self {
        }
    }
}

#[async_trait]
impl Plugin for PluginAuth {
    async fn plugin_init(&mut self) {
        println!("{} initialized", self.plugin_name());
    }

    fn plugin_name(&self) -> &str {
        "PluginAuth"
    }

    fn plugin_can_handle(&self, _req: &Request<Body>) -> bool {
        true
    }

    async fn plugin_handle(&self, _req: Request<Body>) -> Result<Response<Body>, Infallible> {
        return Ok(Response::new(Body::from("Hello from PluginAuth!")));
    }
}
