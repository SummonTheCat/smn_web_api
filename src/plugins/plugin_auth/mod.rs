use async_trait::async_trait;
use hyper::{Body, Request, Response};
use rand::{rngs::OsRng, RngCore};
use smn_web_core::structs::struct_plugin::Plugin;
use std::convert::Infallible;

pub mod structs;

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

    fn plugin_can_handle(&self, req: &Request<Body>) -> bool {
        // If starts with /auth, then this plugin can handle it.
        return req.uri().path().starts_with("/auth");
    }

    async fn plugin_handle(&self, req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let stripped_path = req.uri().path().strip_prefix("/auth").unwrap();

        let matched_response = match stripped_path {
            "" => handle_landing(),
            "/tokenget" => handle_get_token(),
            _ => handle_error_404(),
        };

        return matched_response;
    }

}

fn handle_get_token() -> Result<Response<Body>, Infallible> {
    // We are testing some logic here.
    // Generate a random auth token
    let auth_token = generate_token();


    Ok(Response::new(Body::from(auth_token)))
}

fn handle_landing() -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("This is the SmnAuth route, this is an api route and is not meant to be accessed directly.")))
}

fn handle_error_404() -> Result<Response<Body>, Infallible> {
    Ok(Response::builder()
        .status(404)
        .body(Body::from("Not found"))
        .unwrap())
}


// ----- Utility Functions -----

fn generate_token() -> String {
    let mut buffer = vec![0u8; 32];
    OsRng.fill_bytes(&mut buffer);
    // Convert the byte array into a hex string
    buffer.iter().map(|b| format!("{:02x}", b)).collect()
}