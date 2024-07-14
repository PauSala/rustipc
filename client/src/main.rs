use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;

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

    // Pick a name.
    let name = "/Users/pausala/dev/rustipc/server/example.sock".to_fs_name::<GenericFilePath>()?;
    let mut buffer = String::with_capacity(128);
    let conn = Stream::connect(name)?;
    let mut conn = BufReader::new(conn);
    conn.get_mut().write_all(command.as_str())?;
    conn.read_line(&mut buffer)?;
    match command {
        Command::Hello => {}
        Command::Stop => {
            let window_handler = dioxus::desktop::window();
            window_handler.close();
        }
    }
    Ok(())
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
