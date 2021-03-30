//extern crate dotenv;
//extern crate s3;

use cloudevents_sdk_tide::*;
//use cloudevents_sdk_tide::Event;
use rhai::serde::{from_dynamic, to_dynamic};
use serde_json::{Value};
use rhai::{Dynamic, Engine, Scope, ImmutableString};
use rhai::RegisterFn;                       // use 'RegisterFn' trait for 'register_fn'
// use rhai::RegisterResultFn; 
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tide::{Body, Response, StatusCode};
use tide::utils::After;
use dotenv::dotenv;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use std::env;
use std::str;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Script {
    content: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Error {
    description: String,
    code: u32
}

#[derive(Clone)]
struct State {
    config: Arc<String>,
}

impl State {
    fn new() -> Self {
        Self {
            config: Arc::new(String::from("")),
        }
    }
}

#[allow(dead_code)]
struct Storage {
    name: String,
    region: Region,
    credentials: Credentials,
    bucket: String,
    location_supported: bool,
}

fn log(s: ImmutableString) {
    println!("{:?}", s)
}

fn log_dynamic(s: Dynamic) {
    println!("{:?}", s)
}

fn get_object(s:ImmutableString) -> ImmutableString {
    let s3_access_key = env::var("S3_ACCESS_KEY").unwrap_or_default();
    let s3_secret = env::var("S3_SECRET").unwrap_or_default();
    let s3_bucket_name = env::var("S3_BUCKET_NAME").unwrap_or_default();
    let s3_region = env::var("S3_REGION").unwrap_or_default();
    let s3 = Storage {
        name: "aws".into(),
        region: s3_region.to_string().parse().unwrap(),
        credentials: Credentials::new(Some(s3_access_key.as_str()), Some(s3_secret.as_str()), None, None, None)
            .unwrap(),
        bucket: s3_bucket_name,
        location_supported: false,
    };

    let bucket = match Bucket::new(&s3.bucket, s3.region, s3.credentials) {
        Ok(v) => v,
        Err(e) => panic!("Bucket Creation Failed: {}", e),
    };

    let (data, _code) = bucket.get_object_blocking(s.as_str()).unwrap();

    //let (d, c) = bucket.get_object("/test.file").unwrap();
    
    let r : ImmutableString = str::from_utf8(&data).unwrap().into();
    r
}

fn get_object_as_map(s:ImmutableString) -> Dynamic {
    let s3_access_key = env::var("S3_ACCESS_KEY").unwrap_or_default();
    let s3_secret = env::var("S3_SECRET").unwrap_or_default();
    let s3_bucket_name = env::var("S3_BUCKET_NAME").unwrap_or_default();
    let s3_region = env::var("S3_REGION").unwrap_or_default();
    let s3 = Storage {
        name: "aws".into(),
        region: s3_region.to_string().parse().unwrap(),
        credentials: Credentials::new(Some(s3_access_key.as_str()), Some(s3_secret.as_str()), None, None, None)
            .unwrap(),
        bucket: s3_bucket_name,
        location_supported: false,
    };

    let bucket = match Bucket::new(&s3.bucket, s3.region, s3.credentials) {
        Ok(v) => v,
        Err(e) => panic!("Bucket Creation Failed: {}", e),
    };

    let (data, _code) = bucket.get_object_blocking(s.as_str()).unwrap();

    //let (d, c) = bucket.get_object("/test.file").unwrap();
    let v: Value = serde_json::from_str(str::from_utf8(&data).unwrap()).unwrap();
    let dyn_event: Dynamic = to_dynamic(v).unwrap();
    dyn_event
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();
    tide::log::start();
    let mut app = tide::with_state(State::new());
    // curl -d "{\"name\": \"chashu\"}" http://127.0.0.1:8080/
    // let script = r#"event.data.name = "dynamo"; event"#;
    app.with(tide::sessions::SessionMiddleware::new(
        tide::sessions::MemoryStore::new(),
        std::env::var("TIDE_SECRET")
            .expect(
                "Please provide a TIDE_SECRET value of at \
                      least 32 bytes in order to run this example",
            )
            .as_bytes(),
    ));

    app.with(After(|mut res: Response| async {
        if res.status() == StatusCode::NotFound {
            let msg = format!("Error: {:?}", "File not found :( <br/> Try <a href=/public/index.html>/public/index.html</a>.");
            res.set_content_type("text/html");
            res.set_body(msg);
        }
        Ok(res)
    }));
    // app.with(CorsMiddleware::new()
    //  .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
    //  .allow_origin(Origin::from("localhost:*"))
    //  .allow_credentials(true));
    app.at("/").get(tide::Redirect::new("/public/index.html"));
    app.at("/")
        .post(|mut req: tide::Request<State>| async move {
            
            let body = req.body_bytes().await?;
            let evtreq = req.to_event(body.to_vec())?;
            
            let session = req.session_mut();

            let script_content: String = session
                .get("script_content")
                .unwrap_or(String::from(r#"log(event);event"#));

            // let engine = Engine::new();
            let mut engine = Engine::new_raw();
            engine.register_fn("log", log);
            engine.register_fn("log", log_dynamic);
            engine.register_fn("get_object", get_object);
            engine.register_fn("get_object_map", get_object_as_map);
            
            
            let dyn_event: Dynamic = to_dynamic(evtreq).unwrap();
            let mut scope = Scope::new();
            scope.push("event", dyn_event);
            let result: Dynamic = match engine.eval_with_scope(&mut scope, script_content.as_str())
            {
                Ok(f) => f,
                Err(error) => panic!("Problem opening the file: {:?}", error),
            };

            let retevt: Event = from_dynamic(&result).unwrap();
            let resp = Response::new(200).event(retevt).unwrap();
            Ok(resp)
        });

    app.at("/save")
        .post(|mut req: tide::Request<State>| async move {
            let new_script: Script = req.body_json().await?;
            let session = req.session_mut();
            let retval = new_script.clone();
            session
                .insert("script_content", new_script.content)
                .unwrap();

            Ok(Body::from_json(&retval)?)
        });
    app.at("/public").serve_dir("public/")?;

    app.listen("0.0.0.0:8080").await?;
    Ok(())
}



#[cfg(test)]
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use surf;
    #[async_std::test]
    async fn test_post() {
        let str = surf::post("http://localhost:8080").recv_string().await.unwrap();   
        println!("{}", str);
    }


