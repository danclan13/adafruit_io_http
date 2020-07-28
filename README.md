# Adafruit_io_http

**Adafruit_io_http is a client library in rust for communication with Adafruit iot server.**

---

Add this to your cargo.toml file:

```toml
[dependencies]
adafruit_io_http = "0.1.1"
```

Example of transmitting and reading data from an iot cloud using the library: 

```rust
use std::time::Duration;
use std::thread;
extern crate adafruit_io_http;

fn main() {    
  let username = "YOUR_USERNAME";    
  let aiokey = "YOUR_AIO_KEY";    
  let mut ada = adafruit_io_http::ada_io_http::AdaClient::set(username.to_string(), aiokey.to_string());
  let data = 13;

   loop {
        let feedkey = "YOUR_FEED";
        ada.post(feedkey.to_string(), data.to_string());
        thread::sleep(Duration::from_secs(5));
        let data_new = ada.get(feedkey.to_string());
        println!("{:}", data_new);
        thread::sleep(Duration::from_secs(5));
    }
}
```
