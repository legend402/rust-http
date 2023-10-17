use std::collections::HashMap;
use std::fs::{self};
use std::path::PathBuf;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use reqwest::Client;
use serde_json::Value;
use urlencoding::decode;
use encoding_rs::GBK;


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
                None => {}
            }
            let response = get_response_file(get_file_content("./test.html"));
            Ok(response)
        }
        "/home" => {
            let response = get_response_file(get_file_content("./home.html"));
            Ok(response)
        }
        "/post" => {
            let bytes = hyper::body::to_bytes(_req.into_body()).await?;
            let body = String::from_utf8(bytes.to_vec()).unwrap();
            let body_map: Value = serde_json::from_str(body.as_str()).unwrap();
            println!("{:?}", body_map);
            let response = get_response_content(body);
            Ok(response)
        }
        "/sum" => {
            let query_str = _req.uri().query();
            let mut result: i32 = 0;
            match query_str {
                Some(str) => {
                    let hash_map: HashMap<&str, &str> = query_str_to_map(str);
                    let num_vec: Vec<i32> = hash_map.values()
                    .map(|it| it.parse::<i32>().unwrap())
                    .collect();

                    result = reduce(num_vec.clone().into_iter(), |cur, item| cur + item, 0);
                }
                None => {

                }
            }
            let test = get_file_content("./home.html")
            .replace("home", result.to_string().as_str());
            let response = get_response_file(test);
            Ok(response)
        }
        "/getjuejindata" => {
            let client = Client::new();
            let url = "https://api.juejin.cn/recommend_api/v1/article/recommend_cate_feed";
            let value = serde_json::json!({
                "cate_id": "6809637767543259144",
                "cursor": "0",
                "id_type": 2,
                "limit": 20,
                "sort_type": 300,
            }).to_string();
            
            let response = client.post(url)
                .header("Content-Type", "application/json; charset=utf-8")
                .body(value)
                .send()
                .await.expect("请求失败");
            let str_content = response.text().await.unwrap();
            let response: Response<Body> = get_response_content(str_content);
            Ok(response)
        }
        "/filelist" => {
            let query_str = _req.uri().query();
            let mut dir_path = String::new();
            match query_str {
                Some(str) => {
                    let hash_map: HashMap<&str, &str> = query_str_to_map(str);
                    dir_path = hash_map.get("path").unwrap().to_string();
                    // 转译一些特殊字符
                    dir_path = decode(&dir_path).unwrap().to_string();
                }
                None => {}
            }

            if PathBuf::from(&dir_path).is_dir() {
                let dirs = fs::read_dir(&dir_path);
                match dirs {
                    Ok(dir) => {
                        let entries: Vec<String> = dir.map(|it| it.unwrap().path().to_string_lossy().to_string()).collect();
    
                        let vec_str = serde_json::json!(entries).to_string();
            
                        let response: Response<Body> = get_response_content(vec_str);
                        Ok(response)
                    },
                    Err(err) => {
                        let value = serde_json::json!({
                            "message": err.to_string(),
                        }).to_string();
                        let response = get_response_content(value);
                        Ok(response)
                    }
                }
            } else {
                let file = fs::read(&dir_path);
                match file {
                    Ok(content) => {
                        let (decoded, _, _) = GBK.decode(&content);
                        let content_str = decoded.to_string();
                        let value = serde_json::json!({
                            "message": content_str,
                        }).to_string();
                        let response: Response<Body> = get_response_content(value);
                        Ok(response)
                    },
                    Err(err) => {
                        let value = serde_json::json!({
                            "message": err.to_string(),
                        }).to_string();
                        let response = get_response_content(value);
                        Ok(response) 
                    }
                }
            }
        }
        _ => {
            if path.ends_with(".js") || path.ends_with(".html") {
                let file_path = String::from(".") + &String::from(path);
                let response = get_response_file(get_file_content(&file_path));
                return Ok(response);
            }
            let response = get_response_file(get_file_content("./404.html"));
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

fn reduce<F, T, D>(iter: impl Iterator<Item = T>, accumulator: F, default_value: D) -> D
where
   F: Fn(D, T) -> D,
{
   let mut accumulator_value: D = default_value;
   for item in iter {
       accumulator_value = accumulator(accumulator_value, item);
   }
   accumulator_value
}

fn get_response_content<T>(content: T) -> Response<Body>
where 
    T: ToString, hyper::Body: From<T>
{
    Response::builder()
        .header("Content-Type", "application/json; charset=utf-8")
        .body(Body::from(content))
        .expect("Fail to create response")
}

fn get_response_file<T>(content: T) -> Response<Body>
where 
    T: ToString, hyper::Body: From<T>
{
    Response::new(Body::from(content))
}