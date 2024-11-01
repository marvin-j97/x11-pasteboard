use clipboard_rs::{
    common::RustImage, Clipboard, ClipboardContext, ClipboardHandler, ClipboardWatcher,
    ClipboardWatcherContext,
};
use std::{thread, time::Duration};

struct Manager {
    ctx: ClipboardContext,
}

impl Manager {
    pub fn new() -> Self {
        let ctx = ClipboardContext::new().unwrap();
        Manager { ctx }
    }
}

impl ClipboardHandler for Manager {
    fn on_clipboard_change(&mut self) {
        if let Ok(img) = self.ctx.get_image() {
            let png_bytes = img.to_png().unwrap();
            let png_bytes = png_bytes.get_bytes();
            let png_bytes = base64::encode(&png_bytes);

            println!(
                "{}",
                serde_json::json!({
                    "image": format!("data:image/png;base64,{png_bytes}"),
                })
            );
        } else {
            let text = self.ctx.get_text().unwrap();

            println!(
                "{}",
                serde_json::json!({
                    "text": text,
                })
            );
        }
    }
}

fn main() {
    let manager = Manager::new();

    let mut watcher = ClipboardWatcherContext::new().unwrap();

    let watcher_shutdown = watcher.add_handler(manager).get_shutdown_channel();

    eprintln!("start watching");
    watcher.start_watch();
}
