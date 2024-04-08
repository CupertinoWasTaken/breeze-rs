mod status;
mod method;
mod req;
mod res;

use color_eyre::Result;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;

pub use crate::method::Method;
pub use crate::req::{Request, RequestParserError};
pub use crate::res::Response;
use crate::status::StatusCode;

pub const VERSION: &str = "HTTP/1.1";
pub const VERSION_TUPLE: (usize, usize) = (1, 1);

pub struct Server<'a> {
    listener: TcpListener,
    checks: HashMap<PathBuf, Box<dyn FnMut(Request) -> Response + 'a>>,
}

impl<'a> Server<'a> {
    pub fn new(listener: TcpListener) -> Self {
        Self {
            listener,
            checks: HashMap::new(),
        }
    }

    pub fn path<P, F>(&mut self, path: P, predicate: F)
        where
            P: Into<PathBuf>,
            F: FnMut(Request) -> Response + 'a,
    {
        self.checks.insert(path.into(), Box::new(predicate));
    }

    pub fn run(mut self) -> Result<()> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buf = [0; 8192];
                    let read = match stream.read(&mut buf) {
                        Ok(len) => len,
                        Err(_) => continue
                    };

                    let buf = match String::from_utf8(buf[..read].to_vec()) {
                        Ok(str) => str,
                        Err(_) => continue
                    };

                    let req: Request = match buf.try_into() {
                        Ok(req) => req,
                        Err(e) => {
                            eprintln!("{e}");
                            continue
                        }
                    };

                    if req.version == VERSION_TUPLE {
                        let mut res = Response::new();
                        res.status = StatusCode::HTTPVersionNotSupported;

                        stream.write_all(res.to_string().as_ref())?;
                        continue;
                    }

                    match self.checks.get_mut(&req.path) {
                        Some(predicate) => {
                            let predicate = predicate.as_mut();
                            let res = predicate(req).to_string();

                            stream.write_all(res.as_bytes())?
                        }
                        None => {
                            let mut res = Response::new();
                            res.status = StatusCode::NotFound;

                            stream.write_all(res.to_string().as_bytes())?
                        },
                    }
                }
                Err(_) => continue,
            }
        }

        Ok(())
    }
}