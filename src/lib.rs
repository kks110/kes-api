mod models;
mod requests;

extern crate console_error_panic_hook;
use std::panic;
use worker::*;
use crate::requests::{authorised, handle_get, handle_post};

#[event(fetch, respond_with_errors)]
pub async fn main(request: Request, env: Env, _ctx: Context) -> Result<Response> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    Router::new()
        .get_async("/", |req, ctx| async move {
            if authorised(&req, &ctx) {
                handle_get(ctx).await
            } else {
                Response::error("Unauthorized", 401)
            }
        })
        .post_async("/", |req, ctx| async move {
            if authorised(&req, &ctx) {
                handle_post(req, ctx).await
            } else {
                Response::error("Unauthorized", 401)
            }
        })
        .run(request, env)
        .await
}
