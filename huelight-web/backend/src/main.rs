use axum::{
    routing::get,
    Router,
    Json,
};
use serde::Serialize;
use tower_http::cors::{CorsLayer, Any};
use huelight_core::models::light::{LightResponse, Light, LightState};

#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

async fn hello() -> Json<HelloResponse> {
    println!("Hello endpoint called");
    Json(HelloResponse {
        message: "Hello from Rust!".into(),
    })
}

#[derive(Serialize)]
struct LightDto {
    id: u32,
    name: String,
    state: LightState,
    _type: String,
}

async fn set_light_state(light_state: LightState) -> Json<String> {
    // Placeholder for setting light state
    println!("Set light state endpoint called with: {:?}", light_state);
    Json("Light state updated".into())
}

async fn get_lights() -> Json<Vec<LightDto>> {

    println!("Get lights endpoint called");

    // Placeholder implementation
    let mut lights = std::collections::HashMap::new();
    lights.insert(1, Light {
        state: LightState::default()
            .with_on(true)
            .with_brightness(200)
            .with_hue(30000)
            .with_saturation(150),
        name: "Living Room Light".into(),
        _type: "Extended color light".into(),
    });
    lights.insert(2, Light {
        state: LightState::default()
            .with_on(false)
            .with_brightness(100)
            .with_hue(20000)
            .with_saturation(50),
        name: "Office Light".into(),
        _type: "Extended color light".into(),
    });


    // Simulate the response.
    let lights = LightResponse(lights);

    // Convert to List of LightDto.
    let mut light_dtos = Vec::new();
    for (id, light) in &lights.0 {
        println!("Light ID: {}, Name: {}, State: {:?}, Type: {}", id, light.name, light.state, light._type);
        light_dtos.push(LightDto {
            id: *id,
            name: light.name.clone(),
            state: light.state.clone(),
            _type: light._type.clone(),
        });
    }



    Json(light_dtos)
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)   // tighten this in production
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/hello", get(hello))
        .route("/api/lights", get(get_lights))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
