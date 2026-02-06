mod rpc_types;
mod jupyter_bridge;

use crate::rpc_types::{Request, Response};
use crate::jupyter_bridge::JupyterBridge;
use std::io::{self, BufRead, Write};

fn main() {
    let bridge = JupyterBridge::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };

        if line.trim().is_empty() {
            continue;
        }

        let req: Result<Request, _> = serde_json::from_str(&line);
        let response = match req {
            Ok(r) => bridge.handle_request(r),
            Err(e) => Response::Error {
                message: format!("invalid_request: {}", e),
            },
        };

        let json = serde_json::to_string(&response).unwrap();
        writeln!(stdout, "{}", json).unwrap();
        stdout.flush().unwrap();
    }
}
