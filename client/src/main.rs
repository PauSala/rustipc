use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;
static SOCKET_PATH: &str = "../server/rustipc.sock";

fn main() {
    LaunchBuilder::desktop()
        .with_cfg(Config::new().with_window(WindowBuilder::new().with_resizable(true)))
        .launch(app)
}

enum Command {
    Hello,
    Stop,
}

impl Command {
    fn as_str(&self) -> &[u8] {
        match self {
            Command::Hello => b"Hello\n",
            Command::Stop => b"stop\n",
        }
    }
}

fn send_to_server(command: Command) -> std::io::Result<()> {
    use interprocess::local_socket::{prelude::*, GenericFilePath, Stream};
    use std::io::{prelude::*, BufReader};
    let window_handler = dioxus::desktop::window();
    // Pick a name.
    let name = SOCKET_PATH.to_fs_name::<GenericFilePath>()?;
    let mut buffer = String::with_capacity(128);
    let conn = Stream::connect(name);
    match conn {
        Err(_) => {
            window_handler.close();
            Ok(())
        }
        Ok(conn) => {
            let mut conn = BufReader::new(conn);
            let e = conn.get_mut().write_all(command.as_str());
            if e.is_err() {
                window_handler.close();
                return Ok(());
            }
            let e = conn.read_line(&mut buffer);
            if e.is_err() {
                window_handler.close();
                return Ok(());
            }
            match command {
                Command::Hello => {
                    return Ok(());
                }
                Command::Stop => {
                    window_handler.close();
                    return Ok(());
                }
            }
        }
    }
}

#[cfg(not(feature = "collect-assets"))]
const _STYLE: &str = include_str!("assets/fileexplorer.css");

fn app() -> Element {
    rsx! {
        div {
            link { href:"https://fonts.googleapis.com/icon?family=Material+Icons", rel:"stylesheet" }
            header {
                i { class: "material-icons icon-menu", "menu" }
            }
            style { "{_STYLE}" }
            main {
                div{
                    button { onclick: move |_| send_to_server(Command::Hello).unwrap(), "Say Hello!" },
                    button { onclick: move |_| send_to_server(Command::Stop).unwrap(), "Close" },
                },
            }
        }
    }
}
