use serde_json::Value;
use tide::Response;

async fn validate_json(mut req: tide::Request<()>) -> Result<Response, tide::Error> {
    let request_string = req.body_string().await?;
    let parse_result: Result<Value, serde_json::Error> = serde_json::from_str(request_string.as_str());
    if let Ok(result) = parse_result {
        Ok(
            Response::builder(200)
                .content_type("application/json")
                .body(result.to_string())
                .build()
        )
    } else {
        Ok(
            Response::builder(400)
                .build()
        )
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    let mut v2_route = app.at("/api/v2");

    let mut validate_route = v2_route.at("/validate");

    validate_route.post(validate_json);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
