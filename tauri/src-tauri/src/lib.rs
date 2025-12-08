// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::{
    fs, io,
    io::Write,
    sync::{Arc, Mutex},
};
use tauri::{Manager, State};
use tauri_plugin_shell::{process::CommandChild, ShellExt};

struct NextProgress(Arc<Mutex<Option<CommandChild>>>);

pub fn append_text_to_file(file_path: &str, content: &str) -> io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .append(true) // ⭐️ 开启追加模式 ⭐️
        .create(true) // 如果文件不存在则创建
        .open(file_path)?; // ? 操作符处理错误并返回

    file.write(content.as_bytes())?;

    println!("✅ Content successfully appended to: {}", file_path);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let next_progress = NextProgress(Arc::new(Mutex::new(None)));
    let next_progress_clone = next_progress.0.clone();

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .manage(next_progress)
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { .. } => {
                let state: State<NextProgress> = window.state();
                let mut guard = state.0.lock().unwrap();
                if let Some(child) = guard.take() {
                    let _ = child.kill();
                }
            }
            _ => {}
        })
        .setup(move |app| {
            let resource_path = app
                .path()
                .resolve("standalone/server.js", tauri::path::BaseDirectory::Resource)?;
            println!("Resolved resource path: {:?}", resource_path);
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let current_dir = resource_path
                    .parent()
                    .expect("Failed to get parent directory of resource path")
                    .parent()
                    .expect("Failed to get grandparent directory of resource path")
                    .join("_up_")
                    .to_path_buf();
                println!("Current directory for sidecar: {:?}", current_dir);
                append_text_to_file(
                    "D:\\Users\\Administrator\\AppData\\Local\\chat-with-geogebra\\log.txt",
                    &format!("Current directory: {:?}\n", current_dir)
                ).expect("Failed to append to log file");
                let (mut rx, child) = app_handle
                    .shell()
                    .sidecar("nodejs")
                    .unwrap()
                    .args(["standalone/server.js"])
                    .current_dir(current_dir)
                    .spawn()
                    .expect("Failed to spawn sidecar");

                println!("Node.js sidecar started with PID: {}", child.pid());
                append_text_to_file(
                    "D:\\Users\\Administrator\\AppData\\Local\\chat-with-geogebra\\log.txt",
                    &format!("Node.js sidecar started with PID: {}\n", child.pid())
                ).expect("Failed to append to log file");

                {
                    let mut guard = next_progress_clone.lock().unwrap();
                    *guard = Some(child);
                }

                while let Some(event) = rx.recv().await {
                    match event {
                        tauri_plugin_shell::process::CommandEvent::Stdout(line) => {
                            println!("Node.js stdout: {}", String::from_utf8_lossy(&line));
                append_text_to_file(
                    "D:\\Users\\Administrator\\AppData\\Local\\chat-with-geogebra\\log.txt",
                    &format!("Node.js stdout: {}\n", String::from_utf8_lossy(&line))
                ).expect("Failed to append to log file");
                        }
                        tauri_plugin_shell::process::CommandEvent::Stderr(line) => {
                            eprintln!("Node.js stderr: {}", String::from_utf8_lossy(&line));
                append_text_to_file(
                    "D:\\Users\\Administrator\\AppData\\Local\\chat-with-geogebra\\log.txt",
                    &format!("Node.js stderr: {}\n", String::from_utf8_lossy(&line))
                ).expect("Failed to append to log file");
                        }
                        tauri_plugin_shell::process::CommandEvent::Terminated(code) => {
                            println!("Node.js sidecar exited with code: {:?}", code);
                        }
                        tauri_plugin_shell::process::CommandEvent::Error(_) => todo!(),
                        _ => todo!(),
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
