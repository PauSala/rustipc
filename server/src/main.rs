use chrono::Local;
use interprocess::local_socket::GenericFilePath;
use std::{
    fs,
    path::Path,
    process::Command,
    sync::mpsc::{self, Sender},
    thread,
};

use interprocess::local_socket::{prelude::*, ListenerOptions, Stream};
use std::io::{self, prelude::*, BufReader};

static SOCKET_PATH: &str = "rustipc.sock";
static CLIENT_PATH: &str = "../client/target/release/client";

pub fn open_ui() {
    let mut child = Command::new(CLIENT_PATH)
        .spawn()
        .expect("Failed to start the process");
    child.wait().expect("Failed to wait on the child process");
}

pub struct IpcMaster {
    handle_error: fn(io::Result<Stream>) -> Option<Stream>,
    socket_name: String,
    sender: Sender<String>,
}

impl IpcMaster {
    pub fn new(socket_name: String, sender: Sender<String>) -> IpcMaster {
        fn handle_error(conn: io::Result<Stream>) -> Option<Stream> {
            match conn {
                Ok(c) => Some(c),
                Err(e) => {
                    eprintln!("Incoming connection failed: {e}");
                    None
                }
            }
        }
        IpcMaster {
            handle_error,
            socket_name,
            sender,
        }
    }

    pub fn delete_local_socket(&self) {
        let path = Path::new(&self.socket_name);
        if path.exists() {
            fs::remove_file(&self.socket_name).expect("Path should exist");
        }
    }

    pub fn listen(&mut self) -> Result<(), io::Error> {
        let name = self.socket_name.clone().to_fs_name::<GenericFilePath>()?;
        let opts = ListenerOptions::new().name(name);
        self.delete_local_socket();
        let listener = match opts.create_sync() {
            Err(e) if e.kind() == io::ErrorKind::AddrInUse => {
                eprintln!(
                "Error: could not start server because the socket file is occupied. Please check if
				{} is in use by another process and try again.", self.socket_name
            );
                return Err(e);
            }
            x => x?,
        };

        eprintln!("Server running at {}", self.socket_name);
        let mut buffer = String::with_capacity(128);

        for conn in listener.incoming().filter_map(self.handle_error) {
            let mut conn = BufReader::new(conn);
            println!("Incoming connection!");
            conn.read_line(&mut buffer)?;
            conn.get_mut().write_all(b"Hello from server!\n")?;

            if buffer.trim() == "stop" {
                break;
            }
            print!("Client answered: {buffer}");
            self.sender
                .send(buffer.clone())
                .expect("Channel should be available");
            buffer.clear();
        }
        // Remove socket file
        // interprocess should delte it, but on macos is not doing it
        self.delete_local_socket();
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let (tx, rx) = mpsc::channel::<String>();
    let handler = thread::spawn(|| {
        let now = Local::now();
        let formatted_time = now.format("%Y%m%d%H%M%S.sock").to_string();
        let mut master = IpcMaster::new(formatted_time.to_owned(), tx);
        master.listen().unwrap();
    });
    println!("Blocking while command is running");
    open_ui();

    for recieved in rx {
        println!("Recieved: {recieved}")
    }

    let e = handler.join();
    match e {
        Ok(_) => return Ok(()),
        Err(_) => println!("error!"),
    }
    Ok(())
}
