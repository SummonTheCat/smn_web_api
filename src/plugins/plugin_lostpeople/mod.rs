use std::convert::Infallible;

use async_trait::async_trait;
use hyper::{Body, Request, Response};
use smn_web_core::structs::struct_plugin::Plugin;

pub struct PluginLostPeople {
}

impl PluginLostPeople {
    pub fn new() -> Self {
        Self {
        }
    }
}

#[async_trait]
impl Plugin for PluginLostPeople {
    async fn plugin_init(&mut self) {
        println!("{} initialized", self.plugin_name());
    }

    fn plugin_name(&self) -> &str {
        "PluginLostPeople"
    }

    fn plugin_can_handle(&self, req: &Request<Body>) -> bool {
        // If starts with /lostpeople, then this plugin can handle it.
        return req.uri().path().starts_with("/lostpeople");
    }

    async fn plugin_handle(&self, req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let stripped_path = req.uri().path().strip_prefix("/lostpeople").unwrap();

        let matched_response = match stripped_path {
            "/status" => handle_status(),
            _ => handle_error_404(),
        };

        return matched_response;
    }

}

fn handle_status() -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Lost People Server Running...")))
}

fn handle_error_404() -> Result<Response<Body>, Infallible> {
    Ok(Response::builder()
        .status(404)
        .body(Body::from("Not found"))
        .unwrap())
}