use std::collections::HashMap;
use std::fs;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    let addr = ([127, 0, 0, 1], 8888).into();

    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, hyper::Error>(service_fn(handle_request)) }
    });

    let server = 
        Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);
    
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
    Ok(())
}

async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let path = _req.uri().path().to_lowercase();

    match path.as_str() {
        "/test" => {
            let query_str = _req.uri().query();
            match query_str {
                Some(str) => {
                    let hash_map = query_str_to_map(str);
                    println!("{:?}", hash_map);
                }
                None => {

                }
            }
            let response = Response::new(
                Body::from(get_file_content("./test.html"))
            );
            Ok(response)
        }
        "/home" => {
            let response = Response::new(
                Body::from(get_file_content("./home.html"))
            );
            Ok(response)
        }
        "/post" => {
            let bytes = hyper::body::to_bytes(_req.into_body()).await?;
            let body = String::from_utf8(bytes.to_vec()).unwrap();
            let body_map: Value = serde_json::from_str(body.as_str()).unwrap();
            println!("{:?}", body_map);
            let response = Response::new(
                Body::from(body)
            );
            Ok(response)
        }
        "/sum" => {
            let query_str = _req.uri().query();
            let mut result: i32 = 0;
            match query_str {
                Some(str) => {
                    let hash_map = query_str_to_map(str);
                    let num_vec: Vec<i32> = hash_map.values()
                    .map(|it| it.parse::<i32>().unwrap())
                    .collect();

                    for num in num_vec {
                        result = result + num;
                    }
                }
                None => {

                }
            }
            let test = get_file_content("./home.html")
            .replace("home", result.to_string().as_str());
            let response = Response::new(
                Body::from(test)
            );
            Ok(response)
        }
        _ => {
            if path.ends_with(".js") {
                let file_path = ".".to_owned() + path.as_str();
                let response = Response::new(
                    Body::from(get_file_content(&file_path))
                );
                return Ok(response);
            }
            let response = Response::new(
                Body::from(get_file_content("./404.html"))
            );
            Ok(response)
        }
    }
}

fn get_file_content(file_name: &str) -> String {
    fs::read_to_string(&file_name).unwrap()
}

fn query_str_to_map(query_str: &str) -> HashMap<&str, &str>  {
    let collect: Vec<&str> = query_str.split("&").collect();
    let mut hash_map = HashMap::new();
    
    for query in collect {
        let key_value: Vec<&str> = query.split("=").collect();
        hash_map.insert(key_value[0], key_value[1]);
    }
    hash_map
}
