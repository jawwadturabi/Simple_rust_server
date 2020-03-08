use simple_server::{Server, StatusCode};

fn main() {
    let server = Server::new(|req, mut response| {
        let body = String::from_utf8(req.body().to_vec()).unwrap();
        println!("Request received {} {} {:?}", req.method(), req.uri(), body);

        match (req.method().as_str(), req.uri().path()) {
            ("GET", "/") => Ok(response.body(
                String::from("Hello From Rust Server hosted on Clever-Cloud by Jawwad Turabi")
                    .into_bytes(),
            )?),

            ("POST", "/hello") => Ok(response.body(body.into_bytes())?),

            (_, _) => {
                response.status(StatusCode::NOT_FOUND);
                Ok(response.body(String::from("Not Found").into_bytes())?)
            }
        }
    });

    server.listen("0.0.0.0", "8080");
}
