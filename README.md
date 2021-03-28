# klaygroud
A scripting environment for cloudevents.

### run

```bash
# Set TIDE_SECRET to a 32 bit random string
$ export TIDE_SECRET=<32bit>
cargo run
```
Browse to http://127.0,0.1:8080/index.html

### docker 

```bash
docker run -p 8080:8080 -e TIDE_SECRET=<32bit> quay.io/number9/klayground
```
