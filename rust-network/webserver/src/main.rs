use mio::tcp::{TcpListener, TcpStream};
use mio::{Event, Events, Poll, PollOpt, Ready, Token};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;
use std::{env, process, str};

#[macro_use]
extern crate log;

// リスニングソケットのトークン
const SERVER: Token = Token(0);

// ドキュメントルートのパス
const WEBROOT: &str = "webroot";

struct Connection {
    stream: TcpStream,
    response: Vec<u8>,
}

struct WebServer {
    listening_socket: TcpListener,
    connections: HashMap<usize, Connection>, // 各コネクションごとのストリームとバッファを管理
    next_connection_id: usize,
}

impl WebServer {
    /// サーバの初期化
    fn new(addr: &str) -> Result<Self, failure::Error> {
        let address = addr.parse()?;
        let listening_socket = TcpListener::bind(&address)?;
        Ok(WebServer {
            listening_socket,
            connections: HashMap::new(),
            next_connection_id: 1,
        })
    }

    /// サーバを起動
    fn run(&mut self) -> Result<(), failure::Error> {
        let poll = Poll::new()?;
        poll.register(
            &self.listening_socket,
            SERVER,
            Ready::readable(),
            PollOpt::level(),
        )?;

        let mut events = Events::with_capacity(1024);

        loop {
            match poll.poll(&mut events, None) {
                Ok(_) => {}
                Err(e) => {
                    error!("{}", e);
                    continue;
                }
            }

            for event in &events {
                match event.token() {
                    SERVER => {
                        let (stream, remote) = match self.listening_socket.accept() {
                            Ok(t) => t,
                            Err(e) => {
                                error!("{}", e);
                                continue;
                            }
                        };
                        debug!("Connection from {}", &remote);
                        self.register_connection(&poll, stream)
                            .unwrap_or_else(|e| error!("{}", e));
                    }
                    Token(conn_id) => {
                        self.http_handler(conn_id, event, &poll)
                            .unwrap_or_else(|e| {
                                error!("Error handling connection {}: {}", conn_id, e);
                                self.connections.remove(&conn_id);
                            });
                    }
                }
            }
        }
    }

    fn register_connection(
        &mut self,
        poll: &Poll,
        stream: TcpStream,
    ) -> Result<(), failure::Error> {
        let token = Token(self.next_connection_id);
        poll.register(&stream, token, Ready::readable(), PollOpt::edge())?;

        let conn = Connection {
            stream,
            response: Vec::new(),
        };

        self.connections.insert(self.next_connection_id, conn);
        self.next_connection_id += 1;
        Ok(())
    }

    fn http_handler(
        &mut self,
        conn_id: usize,
        event: Event,
        poll: &Poll,
    ) -> Result<(), failure::Error> {
        let conn = self
            .connections
            .get_mut(&conn_id)
            .ok_or_else(|| failure::err_msg("Failed to get connection."))?;

        if event.readiness().is_readable() {
            let mut buffer = [0u8; 2048];
            let nbytes = conn.stream.read(&mut buffer)?;

            if nbytes != 0 {
                conn.response = make_response(&buffer[..nbytes])?;
                poll.reregister(
                    &conn.stream,
                    Token(conn_id),
                    Ready::writable(),
                    PollOpt::edge(),
                )?;
            } else {
                self.connections.remove(&conn_id);
            }
        } else if event.readiness().is_writable() {
            conn.stream.write_all(&conn.response)?;
            self.connections.remove(&conn_id);
        }
        Ok(())
    }
}

fn make_response(buffer: &[u8]) -> Result<Vec<u8>, failure::Error> {
    let http_pattern = Regex::new(r"(.*) (.*) HTTP/1.([0-1])\r\n.*")?;
    let captures = match http_pattern.captures(str::from_utf8(buffer)?) {
        Some(cap) => cap,
        None => return create_msg_from_code(400, None, "text/plain"),
    };

    let method = &captures[1];
    let uri = &captures[2];

    if method != "GET" {
        return create_msg_from_code(501, None, "text/plain");
    }

    let mut path = env::current_dir()?;
    path.push(WEBROOT);
    // URIの先頭のスラッシュを削除して結合
    path.push(uri.trim_start_matches('/'));

    if path.is_dir() {
        path.push("index.html");
    }

    debug!("Requested path: {:?}", path);

    let content_type = match path.extension().and_then(|s| s.to_str()) {
        Some("html") => "text/html",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        _ => "application/octet-stream",
    };

    match File::open(&path) {
        Ok(file) => {
            debug!("Successfully opened file: {:?}", path);
            let mut reader = BufReader::new(file);
            let mut buf = Vec::new();
            reader.read_to_end(&mut buf)?;
            create_msg_from_code(200, Some(buf), content_type)
        }
        Err(e) => {
            warn!("Failed to open file {:?}: {}", path, e);
            create_msg_from_code(404, None, "text/plain")
        }
    }
}

fn create_msg_from_code(
    status_code: u16,
    msg: Option<Vec<u8>>,
    content_type: &str,
) -> Result<Vec<u8>, failure::Error> {
    match status_code {
        200 => {
            let msg = msg.unwrap_or_default();
            let header = format!(
                "HTTP/1.0 200 OK\r\n\
                Server: mio webserver\r\n\
                Content-Type: {}\r\n\
                Content-Length: {}\r\n\r\n",
                content_type,
                msg.len()
            );
            let mut res = header.into_bytes();
            res.extend(msg);
            Ok(res)
        }
        400 => Ok("HTTP/1.0 400 Bad Request\r\nServer: mio webserver\r\nContent-Type: text/plain\r\n\r\n".into()),
        404 => Ok("HTTP/1.0 404 Not Found\r\nServer: mio webserver\r\nContent-Type: text/plain\r\n\r\n".into()),
        501 => Ok("HTTP/1.0 501 Not Implemented\r\nServer: mio webserver\r\nContent-Type: text/plain\r\n\r\n".into()),
        _ => Err(failure::err_msg("Undefined status code.")),
    }
}

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        error!("Usage: webserver <address:port>");
        process::exit(1);
    }
    let mut server = WebServer::new(&args[1]).unwrap();
    info!("Starting server on {}", args[1]);
    server.run().unwrap();
}
