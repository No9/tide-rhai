use cloudevents_sdk_tide::*;
//use cloudevents_sdk_tide::Event;
use rhai::serde::{from_dynamic, to_dynamic};
use rhai::{Dynamic, Engine, Scope};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tide::{Body, Response, StatusCode};
use tide::utils::After;

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
                .unwrap_or(String::from(r#"event"#));

            let engine = Engine::new();
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


