use async_std::path::PathBuf as AsyncPathBuf;
use rhai::serde::{from_dynamic, to_dynamic};
use rhai::{Dynamic, Engine, Scope};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tide::log;
use tide::{Endpoint, Request, Response, Result, StatusCode};

use std::path::{Path, PathBuf};
use std::{ffi::OsStr, io};

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Context {
    data: Value,
    headers: HashMap<String, String>,
}

/// Struct that implements an [`Endpoint`] to and matches requests to rhai files.
pub struct RhaiDir {
    prefix: String,
    dir: PathBuf,
}

impl RhaiDir {
    ///```
    /// use tide_rhai::RhaiDir;
    /// let mut app = tide::new();
    /// app.at("/*")
    /// .get(RhaiDir::new("/*", "./examples/app/").unwrap());
    ///```
    #[allow(dead_code)]
    pub fn new(prefix: &str, dir: impl AsRef<Path>) -> io::Result<Self> {
        let dir = dir.as_ref().to_owned().canonicalize()?;
        let prefix = String::from(prefix);
        Ok(Self { prefix, dir })
    }
}

#[async_trait::async_trait]
impl<State> Endpoint<State> for RhaiDir
where
    State: Clone + Send + Sync + 'static,
{
    async fn call(&self, mut req: Request<State>) -> Result {
        let path = req.url().path();
        let path = path
            .strip_prefix(&self.prefix.trim_end_matches('*'))
            .unwrap();

        let path = path.trim_start_matches('/');
        let mut file_path = self.dir.clone();
        for p in Path::new(path) {
            if p == OsStr::new(".") {
                continue;
            } else if p == OsStr::new("..") {
                file_path.pop();
            } else {
                file_path.push(&p);
            }
        }

        log::info!("Requested file: {:?}", file_path);
        let file_path = AsyncPathBuf::from(file_path);
        if !file_path.starts_with(&self.dir) {
            log::warn!("Unauthorized attempt to read: {:?}", file_path);
            Ok(Response::new(StatusCode::Forbidden))
        } else {
            let res = match std::fs::read_to_string(&file_path) {
                Ok(s) => {
                    let mut m = HashMap::new();
                    for (n, v) in req.iter() {
                        m.insert(String::from(n.as_str()), String::from(v.as_str()));
                    }
                    let data: Value;

                    match req.method() {
                        http_types::Method::Put
                        | http_types::Method::Post
                        | http_types::Method::Patch => {
                            data = match req.body_json().await {
                                Ok(v) => v,
                                Err(e) => {
                                    log::warn!("error parsing value {:?}", e);
                                    let j = r#"{}"#;
                                    let retval: Value = serde_json::from_str(j).unwrap();
                                    retval
                                }
                            }
                        }
                        _ => {
                            let j = r#"{}"#;
                            data = serde_json::from_str(j).unwrap();
                        }
                    }

                    let ctx = Context {
                        headers: m,
                        data: data,
                    };

                    let dyn_ctx: Dynamic = to_dynamic(ctx).unwrap();
                    let mut scope = Scope::new();
                    scope.push("ctx", dyn_ctx);
                    let engine = Engine::new_raw();
                    let result = match engine.eval_with_scope(&mut scope, s.as_str()) {
                        Ok::<Dynamic, _>(o) => {
                            let evt: Value = match from_dynamic(&o) {
                                Ok(v) => v,
                                Err(e) => {
                                    log::warn!("Error parsing return value from script {:?}", e);
                                    let j = r#"{"Error" : "Script return value error"}"#;
                                    let retval: Value = serde_json::from_str(j).unwrap();
                                    retval
                                }
                            };
                            Ok(Response::builder(StatusCode::Ok).body(evt).build())
                        }
                        Err(e) => {
                            log::warn!("Script execution error: {:?}", e);
                            Ok(Response::new(StatusCode::InternalServerError))
                        }
                    };
                    result
                }
                Err(e) if e.kind() == io::ErrorKind::NotFound => {
                    log::warn!("File not found: {:?}", &file_path);
                    Ok(Response::new(StatusCode::NotFound))
                }
                Err(e) => return Err(e.into()),
            };
            res
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[async_std::test]
    async fn get() {
        tide::log::start();
        let mut app = tide::new();

        app.at("/*").all(RhaiDir::new("/*", "./test").unwrap());

        use tide_testing::TideTestingExt;

        assert_eq!(
            app.get("/hello").recv_string().await.unwrap(),
            r#"{"hello":"world"}"#
        );
        assert_eq!(
            app.post("/missing").await.unwrap().status(),
            tide::http::StatusCode::NotFound
        );
    }

    #[async_std::test]
    async fn put() {
        let mut app = tide::new();
        app.at("/*").all(RhaiDir::new("/*", "./test").unwrap());

        use tide_testing::TideTestingExt;
        let put_body = r#"{"hello":"world!"}"#;
        let response_body: serde_json::value::Value = app
            .put("/put")
            .body(tide::Body::from_string(put_body.into()))
            .content_type("application/custom")
            .header("custom", "header-value")
            .recv_json()
            .await
            .unwrap();

        assert_eq!(response_body, json!({"hello":"rhai"}));
    }

    #[async_std::test]
    async fn parse_error() {
        let mut app = tide::new();
        app.at("/*").all(RhaiDir::new("/*", "./test").unwrap());

        use tide_testing::TideTestingExt;

        assert_eq!(
            app.post("/parse_error").await.unwrap().status(),
            tide::http::StatusCode::InternalServerError
        );
    }
}
