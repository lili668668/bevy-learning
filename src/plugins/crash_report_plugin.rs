use std::io::Write;
use std::panic::PanicHookInfo;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use bevy::prelude::*;

pub struct CrashReportPlugin;

impl Plugin for CrashReportPlugin {
    fn build(&self, _app: &mut App) {
        let previous_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            write_crash_report(info);
            previous_hook(info);
        }));
    }
}

fn write_crash_report(info: &PanicHookInfo) {
    let Ok(home) = std::env::var("HOME") else { return; };
    let dir = PathBuf::from(home).join("Library/Logs/poker");
    if std::fs::create_dir_all(&dir).is_err() { return; }

    let seconds = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let path = dir.join(format!("crash-{seconds}.txt"));
    let Ok(mut file) = std::fs::File::create(&path) else { return; };

    let message = info.payload_as_str().unwrap_or("(not a string)");
    let location = info.location().map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column())).unwrap_or_default();
    let thread = std::thread::current();

    let _ = writeln!(file, "version: {}", env!("CARGO_PKG_VERSION"));
    let _ = writeln!(file, "os: {} {}", std::env::consts::OS, std::env::consts::ARCH);
    let _ = writeln!(file, "time: {seconds}");
    let _ = writeln!(file, "thread: {}", thread.name().unwrap_or("(unnamed)"));
    let _ = writeln!(file, "location: {location}");
    let _ = writeln!(file, "message: {message}");
}
