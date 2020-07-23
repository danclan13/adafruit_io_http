
use std::error::Error;
use std::time::Duration;
pub mod ada_io_http::AdaClient; 

//use rppal::uart::{Parity, Uart};
fn main() -> Result<(), Box<dyn Error>> {
    AdaClient.set("Danny111","aio_POmF19N8uXsdAjfzuHakIzOdWH4S"); 

    
    loop {
        // Fill the buffer variable with any incoming data.
        thread::sleep(Duration::from_secs(10));
        AdaClient.post("modeon",75);        
    }
}