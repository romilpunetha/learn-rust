use std::collections::HashMap;
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::io::{Read, Write};
#[allow(unused_imports)]
use std::ops::{Add, Sub};
use std::str;
use std::time::{Duration, SystemTime};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Logs from your program will appear here!");

    let mut listener = TcpListener::bind("127.0.0.1:6379").await?;
    loop {
        // The second item contains the IP and port of the new connection.
        let (mut stream, _) = listener.accept().await?;
        process(stream);
    }
}

fn process(mut stream: TcpStream) {
    tokio::spawn(async move {
        let mut cache: HashMap<String, (String, SystemTime)> = HashMap::new();

        let mut buffer = [0; 1024];

        loop {
            stream.read(&mut buffer).await;

            println!("Received a request: {} ", String::from_utf8_lossy(&buffer));

            let str = str::from_utf8(&buffer).unwrap();

            let split: Vec<&str> = str.trim().split("\n").collect();

            if split.len() < 2 {
                continue;
            }

            let command = split.get(split.len() - 2).unwrap();

            println!("Command : {}", command);

            if str.contains("ping") {
                stream.write_all("+PONG\r\n".as_bytes()).await;
            } else if str.contains("set") {
                let key = split.get(4).unwrap().trim();
                let value = split.get(6).unwrap().trim();

                let pair: (String, SystemTime);
                if str.contains("px") {
                    let time = split.get(10).unwrap().trim().parse::<u64>().unwrap();
                    pair = ((*value).parse().unwrap(), SystemTime::now().add(Duration::from_millis(time)))
                } else {
                    pair = ((*value).parse().unwrap(), SystemTime::now().add(Duration::from_millis(1000000000)))
                }

                cache.insert((*key).parse().unwrap(), pair);

                stream.write_all("+OK\r\n".as_bytes()).await;
            } else if str.contains("get") {
                let key = split.get(4).unwrap().trim();


                let value: (String, SystemTime);

                if cache.contains_key(key) {
                    value = cache.get(key).unwrap().to_owned();
                } else {
                    value = ("".to_owned(), SystemTime::now().sub(Duration::from_millis(100000000)));
                }

                let mut response: String;

                if value.1 < SystemTime::now() {
                    cache.remove(key);
                    response = "$-1\r\n".to_owned();
                } else {
                    response = "+".to_owned();
                    response.push_str(&value.0);
                }

                println!("Response : {}", response.trim());

                stream.write_all(response.trim().as_bytes()).await;
            } else {
                let mut response: String = "+".to_owned();
                response.push_str(command);

                println!("Response : {}", response.trim());
                stream.write_all(response.trim().as_bytes()).await;
            }
        }
    });
}
