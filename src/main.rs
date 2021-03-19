use rhai::serde::{from_dynamic, to_dynamic};
use rhai::{Dynamic, Engine, Map, Scope};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tide::http::mime;
//use http_types::HeaderValue;
use tide::{Body, Response};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Event {
    specversion: String,
    r#type: String,
    source: String,
    subject: String,
    id: String,
    time: String,
    datacontenttype: String,
    data: Map,
}

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

fn buildevent(req: tide::Request<State>, content_type: String, event_data: Map) -> Event {
    let event = Event {
        specversion:  match req.header("Ce-specversion") { 
            Some(h) => String::from(h.as_str()),
            None => String::from("1.0")
        },
        r#type: match req.header("Ce-Type") { 
            Some(h) => String::from(h.as_str()),
            None => String::from("unknown")
        },
        source: match req.header("Ce-Source") { 
            Some(h) => String::from(h.as_str()),
            None => String::from(req.url().clone().into_string())
        },
        subject: match req.header("Ce-Subject") { 
            Some(h) => String::from(h.as_str()),
            None => String::from("unknown")
        },
        id: match req.header("Ce-Id") { 
            Some(h) => String::from(h.as_str()),
            None => String::from("unknown")
        },
        time: match req.header("Ce-Subject") { 
            Some(h) => String::from(h.as_str()),
            None => String::from("unknown")
        },
        datacontenttype: content_type,
        data: event_data,
    };

    return event;
}

#[async_std::main]
async fn main() -> tide::Result<()> {
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

//     -H "Ce-Id: 536808d3-88be-4077-9d7a-a3f162705f79" \
// -H "Ce-specversion: 0.3" \
// -H "Ce-Type: dev.knative.samples.helloworld" \
// -H "Ce-Source: dev.knative.samples/helloworldsource" \
// -H "Content-Type: application/json" \
// -d '{"msg":"Hello World from the curl pod."}'

    // app.with(CorsMiddleware::new()
    //  .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
    //  .allow_origin(Origin::from("localhost:*"))
    //  .allow_credentials(true));

    app.at("/")
        .post(|mut req: tide::Request<State>| async move {

            let content_type = match req.header("Content-Type") { 
                Some(h) => String::from(h.as_str()),
                None => String::from("unknown")
            };

            let location = req.url().clone().into_string();

            let session = req.session_mut();

            let script_content: String = session
                .get("script_content")
                .unwrap_or(String::from(r#"event.data.name = "dynamo"; event"#));

            if content_type != String::from("application/json") {
                let msguuid = Uuid::new_v4();
                let err_event_data = Error {
                    code : 415,
                    description : format!("The content type {} is not supported", content_type)
                };

                let v : Map = serde_json::from_value(serde_json::to_value(err_event_data).unwrap()).unwrap();
                let err_event = buildevent(req, content_type, v);
                let response = Response::builder(415)
                .body(Body::from_json(&err_event)?)
                .header("Ce-Id", msguuid.to_string())
                .header("Ce-specversion", "1.0")
                .header("Ce-Source", location)
                .header("Ce-Type", "roche-error")
                .content_type(mime::JSON)
                .build();
                return Ok(response);
            }
            let event_data: Map = req.body_json().await?;
            let msguuid = Uuid::new_v4();

            // evt
            
            let event = buildevent(req, content_type, event_data);
            
            let engine = Engine::new();
            let dyn_event: Dynamic = to_dynamic(event).unwrap();
            let mut scope = Scope::new();
            scope.push("event", dyn_event);
            let result: Dynamic = match engine.eval_with_scope(&mut scope, script_content.as_str())
            {
                Ok(f) => f,
                Err(error) => panic!("Problem opening the file: {:?}", error),
            };

            let retevt: Event = from_dynamic(&result).unwrap();

            let response = Response::builder(200)
                .body(Body::from_json(&retevt)?)
                .header("Ce-Id", msguuid.to_string())
                .header("Ce-specversion", "0.3")
                .header("Ce-Source", "knative/eventing/samples/hello-world")
                .header("Ce-Type", "dev.knative.samples.hifromknative")
                .content_type(mime::JSON)
                .build();
            Ok(response)
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
    app.at("/").serve_dir("public/")?;

    app.listen("127.0.0.1:8080").await?;
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


