use async_std::task;
use rhai::serde::{from_dynamic, to_dynamic};
use rhai::{Dynamic, EvalAltResult, ImmutableString};
use std::collections::HashMap;
use std::str::FromStr;
use surf::http::Method;
use surf::{Request, StatusCode, Url};

#[derive(Debug, Clone)]
pub struct Options {
    url: ImmutableString,
    method: ImmutableString,
    headers: Dynamic,
    body: Dynamic,
}

impl Options {
    // Remember &mut must be used even for getters
    pub fn get_url(&mut self) -> ImmutableString {
        self.url.clone()
    }

    pub fn set_url(&mut self, new_val: ImmutableString) {
        self.url = new_val;
    }

    pub fn get_method(&mut self) -> ImmutableString {
        self.method.clone()
    }

    pub fn set_method(&mut self, new_val: ImmutableString) {
        self.method = new_val;
    }
    pub fn get_headers(&mut self) -> Dynamic {
        self.headers.clone()
    }
    pub fn set_headers(&mut self, new_val: Dynamic) {
        self.headers = new_val;
    }
    pub fn get_body(&mut self) -> Dynamic {
        self.body.clone()
    }
    pub fn set_body(&mut self, new_val: Dynamic) {
        self.body = new_val;
    }

    pub fn new() -> Self {
        Self {
            url: "".into(),
            method: "GET".into(),
            headers: to_dynamic("").unwrap(),
            body: to_dynamic("").unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Response {
    body: Dynamic,
    headers: Dynamic,
}

impl Response {
    pub fn get_body(&mut self) -> Dynamic {
        self.body.clone()
    }
    pub fn set_body(&mut self, new_val: Dynamic) {
        self.body = new_val;
    }
    pub fn get_headers(&mut self) -> Dynamic {
        self.headers.clone()
    }
    pub fn set_headers(&mut self, new_val: Dynamic) {
        self.headers = new_val;
    }
}

pub fn fetch(opts: Options) -> Result<Response, Box<EvalAltResult>> {
    let copts = opts.clone();
    let th_url = copts.url;
    let th_body = copts.body;
    let th_headers = copts.headers;
    let th_method = copts.method;

    match task::block_on(async move {
        let l_url = Url::parse(th_url.as_str())?;
        let l_method = Method::from_str(th_method.as_str())?;

        let mut l_req = Request::new(l_method, l_url);
        if !l_method.is_safe() {
            l_req.set_body(http_types::Body::from_json(&th_body)?);
        }

        let mut l_headers: HashMap<String, String> = HashMap::new();
        if th_headers.type_name() != "string" {
            // the headers have been set to lets try and map them
            l_headers = match from_dynamic(&th_headers) {
                Ok(v) => v,
                Err(e) => {
                    log::warn!("Hashmap Parse Error {:?}", e);
                    return Err(surf::Error::from_str(
                        StatusCode::InternalServerError,
                        "Hashmap error",
                    ));
                }
            };
        }

        if l_method != Method::Trace {
            for (n, v) in l_headers.iter() {
                l_req.set_header(n.as_str(), v.as_str());
            }
        }

        let l_client = surf::client();

        let mut r_resp = l_client.send(l_req).await?;

        let r_body: Dynamic = r_resp.body_json().await?;
        let mut r_hmap = HashMap::new();
        for (n, v) in r_resp.iter() {
            r_hmap.insert(String::from(n.as_str()), String::from(v.as_str()));
        }
        let r_headers = to_dynamic(r_hmap).unwrap();
        Ok::<Response, surf::Error>(Response {
            body: r_body,
            headers: r_headers,
        })
    }) {
        Ok(v) => Ok(v),
        Err(e) => {
            log::error!("Request Error: {}", e);
            Err("Surf Error".into())
        }
    }
}
