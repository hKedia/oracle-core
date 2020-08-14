use json;
use sincere;
use std::str::from_utf8;

/// Starts the ERG-USD GET API server which can be made publicly available without security risk
pub fn start_get_api(core_port: &str) {
    let mut app = sincere::App::new();

    // Basic welcome endpoint
    app.get("/", move |context| {
        let response_text = format!(
            "This is an Oracle Core Connector. Please use one of the endpoints to interact with it.\n"
        );
        context
            .response
            .header(("Access-Control-Allow-Origin", "*"))
            .from_text(response_text)
            .unwrap();
    });

    // Start the API server with the port designated in the config.
    let port = ((core_port
        .parse::<u16>()
        .expect("Failed to parse oracle core port from config to u16."))
        + 2)
    .to_string();
    let address = "0.0.0.0:".to_string() + &port;
    app.run(&address, 1).ok();
}