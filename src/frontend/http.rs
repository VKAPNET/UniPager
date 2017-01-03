use tiny_http;

pub fn run() {
    let server = tiny_http::Server::http("0.0.0.0:8080").unwrap();
    loop {
        let req = match server.recv() {
            Ok(req) => req,
            Err(_) => break
        };

        let (mime, data) = match req.url() {
            "/main.js" => ("application/javascript", include_str!("main.js")),
            "/style.css" => ("text/css", include_str!("style.css")),
            _ => ("text/html", include_str!("index.html"))
        };

        let res = tiny_http::Response::from_data(data).with_header(
            tiny_http::Header {
                field: "Content-Type".parse().unwrap(),
                value: mime.parse().unwrap()
            }
        );

        req.respond(res).ok();
    }
}
