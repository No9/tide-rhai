<h1 align="center">tide-rhai</h1>
<div align="center">
  <strong>
    A scripting engine for tide.
  </strong>
</div>

<br />

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/tide-rhai">
    <img src="https://img.shields.io/crates/v/tide-rhai.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/tide-rhai">
    <img src="https://img.shields.io/crates/d/tide-rhai.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/tide-rhai">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

<div align="center">
  <h3>
    <a href="https://docs.rs/tide-rhai">
      API Docs
    </a>
    <span> | </span>
    <a href="https://github.com/no9/tide-rhai/releases">
      Releases
    </a>
    <span> | </span>
    <a href="https://github.com/no9/tide-rhai/blob/master.github/CONTRIBUTING.md">
      Contributing
    </a>
  </h3>
</div>

## Overview

This component provides the ability to run [rhai scripts](https://github.com/rhaiscript/rhai) to process http requests in [tide](https://github.com/http-rs/tide).
Currently it only supprts modifying the messages but additional features such as a http client are being considered.

## Install 
```
$ cargo add tide-rhai
```

## Example

Create a tide server that points to a directory containing rhai scripts.

```rust
use tide_rhai::RhaiDir;
#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();
    let mut app = tide::new();
    app.at("/*")
        .get(RhaiDir::new("/*", "./").unwrap());
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
```
The first parameter for new is the prefix and should be mapped to the `at` parameter. 
The second is the folder with the rhai scripts in 

Creat a rhai script called headers.rhai that selects a header and returns it in a JSON Message.
Note it doesn't have to be called .rhai but [VS Code](https://marketplace.visualstudio.com/items?itemName=Aster.vscode-rhai) has suppport for that file extention.

```rust
let obj = #{};
obj.message = "Is this acceptable?" + ctx.headers["accept"];
obj
```
Here we are using the `headers` property of the context object. If this was a POST then the `ctx` object would also contain a `data` property with the JSON that has been sent to the server.


When you now run to https://localhost:8080/headers.rhai you should see the following:
```json
{"message":"Is this acceptable?text/html,application/xhtml+xml,application/xml;q=0.9,
image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9"}
```
This example can also be ran by cloning this repository and running 

```bash
$ cargo run --example helloworld
```
