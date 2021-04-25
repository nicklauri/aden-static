#![allow(unused_variables, unused_import_braces, unused_imports, dead_code)]
#![deny(unused_must_use)]

mod cache;
mod config;
mod request;
mod server;
mod threadpool;
mod utils;

use std::{io, mem, net::SocketAddr, rc::*, sync::*, thread, time::*};

use tokio::{
    io::{AsyncBufRead, AsyncRead},
    net::{TcpListener, TcpStream},
};

use cache::*;
use config::*;
use server::start_server;
use utils::*;

fn main() {
    let result = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(num_cpus::get())
        .build()
        .unwrap()
        .block_on(start_server())
        .unwrap();
}
