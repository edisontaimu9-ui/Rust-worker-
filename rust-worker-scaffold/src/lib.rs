use serde::Serialize;
use worker::*;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

#[derive(Serialize)]
struct EchoResponse {
    you_sent: String,
}

#[event(fetch)]
async fn fetch(req: Request, env: Env, ctx: Context) -> Result<Response> {
    // Optional: log panics to the Worker console instead of silently failing
    console_error_panic_hook::set_once();

    let router = Router::new();

    router
        .get("/", |_, _| Response::ok("Rust Worker is running."))
        .get_async("/health", |_, _| async move {
            let body = HealthResponse {
                status: "ok",
                service: "rust-worker-scaffold",
            };
            Response::from_json(&body)
        })
        .get_async("/echo/:msg", |_, ctx| async move {
            let msg = ctx.param("msg").unwrap_or(&"".to_string()).to_string();
            let body = EchoResponse { you_sent: msg };
            Response::from_json(&body)
        })
        .post_async("/api/data", |mut req, _ctx| async move {
            // Example: parse incoming JSON
            let payload: serde_json::Value = match req.json().await {
                Ok(v) => v,
                Err(_) => return Response::error("Invalid JSON body", 400),
            };
            Response::from_json(&serde_json::json!({
                "received": payload,
                "note": "replace this handler with real logic"
            }))
        })
        .run(req, env)
        .await
}
