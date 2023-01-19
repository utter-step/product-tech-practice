use std::{
    error::Error,
    io::prelude::*,
    fs,
    net,
};

const CUR_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn not_found() -> Result<Vec<u8>, Box<dyn Error>> {
    let data = fs::read_to_string(format!("{CUR_DIR}/content/not-found.html"))?;

    return Ok(format!(r#"HTTP/1.1 404 Not Found
Server: vlads-laptop
Content-Type: text/html

{data}"#).into_bytes());
}

fn handle_request(request: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let (request_head, _rest) = request.split_once("\r\n").unwrap();
    println!("now handling: {request_head}");

    let instruction = if request_head == "GET / HTTP/1.1" {
        "Ань, привет, <a href='/hi-anya'>заходи</a>!"
    } else if request_head == "GET /hi-anya HTTP/1.1" {
        "Привет, Аня, ты в тайной комнате :)"
    } else if request_head.starts_with("GET /images/") {
        let image_path = request_head
            .strip_prefix("GET /images/")
            .map(|s| s.strip_suffix(" HTTP/1.1"))
            .flatten()
            .unwrap();

        let image_data = fs::read(
            format!("{CUR_DIR}/content/images/{image_path}")
        );

        match image_data {
            Ok(mut data) => {
                let mut response = r#"HTTP/1.1 200 OK
Server: vlads-laptop
Content-Type: image/jpg

"#.to_owned().into_bytes();

                response.append(&mut data);

                return Ok(response)
            },
            Err(e) => {
                dbg!(e);

                return not_found();
            },
        }
    } else {
        return not_found();
    };

    println!(
        "Got data:\n{request}",
    );

    Ok(format!(r#"HTTP/1.1 200 OK
Server: vlads-laptop
Content-Type: text/html

<!DOCTYPE html>
<html>
<head>
<title>Playground For Vlad's lecture</title>
<meta charset="utf-8">
</head>
<body>
<div>{instruction}</div>
<img src="https://utterstep-public.fra1.cdn.digitaloceanspaces.com/poll-26-12.jpg" width="300px" />
<pre>
{request}
</pre>
</body>
</html>"#).into_bytes())
}

fn main() -> Result<(), Box<dyn Error>> {
    let listener = net::TcpListener::bind("127.0.0.1:12345")?;

    for stream in listener.incoming() {
        let mut stream = stream?;

        println!("got connection from {}", stream.peer_addr()?);

        let mut data = [0; 2048];
        stream.read(&mut data)?;

        let request_data = String::from_utf8_lossy(&data);

        let response = handle_request(&request_data)?;

        stream.write_all(&response)?;
    }

    Ok(())
}
