mod config;
mod log;

use std::{env, path, process::Command, thread};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config_file = args.get(1);

    let path = match config_file {
        Some(file) => path::Path::new(file),
        None => path::Path::new("./trun.yaml"),
    };

    let config = config::Config::read(path);

    if let Err(ref e) = config {
        match e {
            config::ConfigError::Invalid(e) => {
                log::error(&format!("Your config file is invalid. Error: {e}"))
            }
            config::ConfigError::FilesystemError(e) => {
                log::error(&format!("A filesystem error occured. Error: {e}"))
            }
        }
    }

    let mut handles: Vec<thread::JoinHandle<_>> = Vec::new();
    for task in config.unwrap().tasks {
        let handle = thread::spawn(move || {
            println!("Running target \"{}\"", task.name);

            let out = if cfg!(target_os = "windows") {
                Command::new("cmd").args(["/C", &task.command]).output()
            } else {
                Command::new("sh").args(["-c", &task.command]).output()
            };

            match out {
                Ok(out) => {
                    println!("Task \"{}\" finished successfully", task.name);
                    if task.output.is_some() && task.output.unwrap() {
                        println!(
                            "Output of \"{}\": {}",
                            task.name,
                            String::from_utf8(out.stdout)
                                .unwrap_or_else(|_| { "Output isn't valid UTF-8".to_string() })
                        )
                    }
                }
                Err(why) => println!("Task \"{}\" failed; Error:\n{why}", task.name),
            }
        });
        handles.push(handle);
    }

    for t in handles {
        if let Err(why) = t.join() {
            println!("Failed to join thread: {why:?}");
        };
    }
}
