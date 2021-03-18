use rhai::serde::{from_dynamic, to_dynamic};
use rhai::{Dynamic, Engine, Map, Scope};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tide::Body;
#[derive(Deserialize, Serialize, Debug, Clone)]
struct Event {
    specversion: String,
    r#type: String,
    source: String,
    subject: String,
    id: String,
    time: String,
    datacontenttype: String,
    pub data: Map,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Script {
    content: String,
}
#[derive(Clone)]
struct State {
    config: Arc<String>,
}

impl State {
    fn new() -> Self {
        Self {
            config : Arc::new(String::from("")),
            }
        }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let mut app = tide::with_state(State::new());
    // curl -d "{\"name\": \"chashu\"}" http://127.0.0.1:8080/execute
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

    // app.with(tide::utils::Before(|mut request: tide::Request<State>| async move {
    //     let session = request.session_mut();
    //     let script_content: usize = session.get("script_content").unwrap_or_default();
    //     session.insert("visits", visits + 1).unwrap();
    //     request
    // }));

    // app.with(CorsMiddleware::new()
    //  .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
    //  .allow_origin(Origin::from("localhost:*"))
    //  .allow_credentials(true));
 
    app.at("/execute")
        .post(|mut req: tide::Request<State>| async move {
            let event_data: Map = req.body_json().await?;
           
            let event = Event {
                specversion: String::from("1.0"),
                r#type: String::from("cloudevent"),
                source: String::from("/path"),
                subject: String::from("/path"),
                id: String::from("/path"),
                time: String::from("/path"),
                datacontenttype: String::from("application/json"),
                data: event_data,
            };
            let session = req.session_mut();
            // let script_content = String::from(r#"event.data.name = "dynamo"; event"#);
            
            let script_content : String = session.get("script_content").unwrap();

            let engine = Engine::new();
            let dyn_event: Dynamic = to_dynamic(event).unwrap();
            let mut scope = Scope::new();
            scope.push("event", dyn_event);
            let result: Dynamic = match engine.eval_with_scope(&mut scope, script_content.as_str())
            {
                Ok(f) => f,
                Err(error) => panic!("Problem opening the file: {:?}", error),
            };

            let retval: Map = from_dynamic(&result).unwrap();
            println!("{:?}", retval);
            Ok(Body::from_json(&retval)?)
        });

    app.at("/save").post(|mut req: tide::Request<State>| async move {

        let new_script: Script = req.body_json().await?;
        let session = req.session_mut();
        let retval = new_script.clone();
        session.insert("script_content", new_script.content).unwrap();
        
        Ok(Body::from_json(&retval)?)
    });
    app.at("/").serve_dir("public/")?;

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
