use async_trait::async_trait;
use hyper::{Body, Request, Response, StatusCode};
use smn_web_core::structs::struct_plugin::Plugin;
use std::convert::Infallible;


pub struct PluginUI {
    pub _ui_root: String,
}

impl PluginUI {
    pub fn new(ui_root: String) -> Self {
        Self {
            _ui_root: ui_root,
        }
    }
}

#[async_trait]
impl Plugin for PluginUI {
    async fn plugin_init(&mut self) {
        println!("{} initialized", self.plugin_name());
    }

    fn plugin_name(&self) -> &str {
        "PluginUI"
    }


    fn plugin_can_handle(&self, req: &Request<Body>) -> bool {
        if req.uri().path().starts_with("/ui") {
            return true;
        } else {
            return false;
        }
    }

    async fn plugin_handle(&self, req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let path = req.uri().path();
        let ui_path = path.trim_start_matches("/ui/component");

        // Return the component path for debugging.
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(ui_path.to_string()))
            .unwrap());
    }
}
