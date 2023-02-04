use std::net::{IpAddr, Ipv4Addr};
extern crate dotenv;
use dotenv::dotenv;
use std::env;
use std::io::Error;

const PORT: &str = "3000";
const HOST: &str = "127.0.0.1";

#[derive(Debug)]
pub struct Config {
    pub port: u16,
    pub host: IpAddr,
}

pub fn create_config() -> Result<Config, Error> {
    dotenv().ok();
    let mut port: Option<String> = None;
    let mut host: Option<String> = None;
    for (key, value) in env::vars() {
        if key == "PORT" {
            port = Some(value);
        } else if key == "HOST" {
            host = Some(value);
        }
    }
    let port = match port {
        Some(v) => v.parse::<u16>().unwrap(),
        None => PORT.parse::<u16>().unwrap(),
    };
    let host = match host {
        Some(v) => parse_host(v)?,
        None => parse_host(HOST.to_string())?,
    };
    Ok(Config { port, host })
}

pub fn parse_host(host: String) -> Result<IpAddr, Error> {
    let splits = host.split(".");
    let mut a = Vec::<u8>::new();
    for s in splits {
        let num = s.parse::<u8>().unwrap();
        a.push(num);
    }
    let v4 = IpAddr::V4(Ipv4Addr::new(a[0], a[1], a[2], a[3]));
    Ok(v4)
}
