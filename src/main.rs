use std::net::TcpStream;
use std::str::from_utf8;
use std::io::Write;
use std::io::Read;
use std::io::{Error, ErrorKind};
use regex::Regex;

const MSG_SIZE: usize = 32;

pub struct Memcached {
    pub host: String,
    pub port: u32,
}

pub struct MemcachedConnection {
    pub socket: TcpStream,
    pub expire: u32,
}

#[allow(dead_code)]
fn main() {

}

impl Memcached {

    pub fn connect(&mut self) -> Result<MemcachedConnection, Error> {
        let connection_string = format!("{}:{}", &self.host, &self.port);
        let tcp_connection = TcpStream::connect(connection_string);

        match tcp_connection {
            Ok(stream) => Ok(MemcachedConnection { socket: stream, expire: 300 }),
            Err(err) => {
                Err(Error::new(ErrorKind::Other, err))
            }
        }
    }

}

impl MemcachedConnection {
    fn send_to_server(&mut self, data: String) -> Result<String, Error> {
        let _ = self.socket.write(data.as_bytes());
        let mut socket_result: Vec<String> = Vec::new();
        loop {
            let mut buffer = [0u8; MSG_SIZE];

            let total_read_from_socket = self.socket.read(&mut buffer);
            let socket_data = from_utf8(&buffer);

            match socket_data {
                Ok(s_data) => socket_result.push(s_data.to_string()),
                Err(err) => println!("Convert error: {}", err),
            }

            match total_read_from_socket {
                Ok(ok) => {
                    if ok < MSG_SIZE {
                        break;
                    }
                }
                Err(err) => {
                    return Err(Error::new(ErrorKind::Other, err));
                }
            }


        }

        Ok(socket_result.join(""))
    }

    pub fn write(&mut self, key: String, value: String, expire: Option<u32>) -> Result<(), Error> {
        
        let write_data = format!("set {} 0 {} {}\r\n{}\r\n", key, expire.unwrap_or(self.expire), &value.len(), value);
        let server_output_res = self.send_to_server(write_data);
        match server_output_res {
            Ok(server_output) => {
                let split = server_output.split("\r\n").next();
                if split == Some("STORED") {
                    Ok(())
                } else {
                    Err(Error::new(ErrorKind::Other, "Could not write"))
                }
            } Err(err) => {
                Err(Error::new(ErrorKind::Other, err))
            }
        }
    }
    
    pub fn read(&mut self, key: String) -> Result<String, Error> {
        let read_data = format!("get {}\r\n", key);
        let server_output_res = self.send_to_server(read_data);
        let is_exists_re = Regex::new(r"^VALUE").unwrap();

        match server_output_res {
            Ok(server_output) => {
                let check = is_exists_re.is_match(&server_output);
                if ! check {
                    return Err(Error::new(ErrorKind::NotFound, "no such key"));
                }
                let split: Vec<&str> = server_output.split("\r\n").collect();
                Ok(split[1].to_string())
            } Err (err) => {
                Err(Error::new(ErrorKind::Other, err))
            }
        }
    }

    pub fn delete(&mut self, key: String) -> Result<(), Error>{
        let delete_data = format!("delete {}\r\n", key);
        let server_output_res = self.send_to_server(delete_data);
        let deleted_re = Regex::new(r"^DELETED").unwrap();
        let not_found_re = Regex::new(r"^NOT_FOUND").unwrap();

        match server_output_res {
            Ok(server_output) => {
                let check = deleted_re.is_match(&server_output);
                if check {
                    return Ok(());
                }

                if not_found_re.is_match(&server_output) {
                    return Err(Error::new(ErrorKind::NotFound, "no such key"));
                }
                    Err(Error::new(ErrorKind::Other, "Unknown error"))
                

            } Err (err) => {
                Err(Error::new(ErrorKind::Other, err))
            }
        }
    }
}