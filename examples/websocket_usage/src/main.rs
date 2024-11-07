use rocket::futures::{SinkExt, StreamExt};
use rocket::get;
use rocket::response::content::RawHtml;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, swagger_ui::*};

#[openapi]
#[get("/")]
fn test_websocket() -> RawHtml<&'static str> {
    RawHtml(
        r#"
        <!DOCTYPE html>
        <html>
            <body>
                Echo: <input type="text" id="echo_text" name="echo" size="10" />
                <input type="button" value="Send" onclick="echo_send()" />
                <br/>
                <br/>
                <p id="output"><p>
                <script>
                    // Create WebSocket connection.
                    const hello_socket = new WebSocket("ws://localhost:8000/hello/bob");
                    const echo_socket = new WebSocket("ws://localhost:8000/echo");
                    const output = document.getElementById('output');
                    
                    // Listen for messages
                    hello_socket.addEventListener("message", (event) => {
                        console.log("Hello response: ", event.data);
                        output.innerHTML += "Hello response: " + event.data + "<br/>";
                    });
                    echo_socket.addEventListener("message", (event) => {
                        console.log("Echo response: ", event.data);
                        output.innerHTML += "Echo response: " + event.data + "<br/>";
                    });

                    function echo_send(){
                        echo_socket.send(document.getElementById('echo_text').value);
                    }
                </script>
            </body>
        </html>
        "#,
    )
}

#[openapi]
#[get("/hello/<name>")]
fn hello(ws: rocket_ws::WebSocket, name: &str) -> rocket_ws::Channel<'_> {
    ws.channel(move |mut stream| {
        Box::pin(async move {
            let message = format!("Hello, {}!", name);
            let _ = stream.send(message.into()).await;
            Ok(())
        })
    })
}

#[openapi]
#[get("/echo")]
fn echo(ws: rocket_ws::WebSocket) -> rocket_ws::Channel<'static> {
    ws.channel(move |mut stream| {
        Box::pin(async move {
            while let Some(message) = stream.next().await {
                let _ = stream.send(message?).await;
            }

            Ok(())
        })
    })
}

#[rocket::main]
async fn main() {
    let launch_result = rocket::build()
        .mount("/", openapi_get_routes![test_websocket, hello, echo,])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    allow_spec_file_download: true,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .launch()
        .await;
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}
