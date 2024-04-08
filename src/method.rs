use std::str::FromStr;
use color_eyre::eyre::eyre;
use color_eyre::Report;

#[derive(Debug)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

impl Default for Method {
    fn default() -> Self {
        Self::Get
    }
}

impl FromStr for Method {
    type Err = Report;

    fn from_str(s: &str) -> color_eyre::Result<Self, Self::Err> {
        use crate::method::Method::*;

        match s.to_uppercase().as_str() {
            "GET" => Ok(Get),
            "HEAD" => Ok(Head),
            "POST" => Ok(Post),
            "PUT" => Ok(Put),
            "DELETE" => Ok(Delete),
            "CONNECT" => Ok(Connect),
            "OPTIONS" => Ok(Options),
            "TRACE" => Ok(Trace),
            "PATCH" => Ok(Patch),
            _ => Err(eyre!("Input ({s}) doesn't match any known HTTP Method.")),
        }
    }
}