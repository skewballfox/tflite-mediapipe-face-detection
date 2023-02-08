use clap::Parser;
use image::io::Reader;
use std::env::current_exe;
use std::io;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///the file path to the mediapipe model
    #[arg(value_name = "FILE")]
    model_bin_name: Option<PathBuf>,
    ///the file path to image to be classified
    #[arg(value_name = "FILE")]
    image_name: Option<PathBuf>,
}

fn main() {
    let cli = Args::parse();

    println!("Hello, world!");
}

fn get_model() -> Result<(PathBuf, PathBuf), io::Error> {
    let cli = Args::parse();
    let model_path = if let Some(path) = cli.model_bin_name {
        Some(path)
    } else {
        default_if_cargo_run("face_detection_full_range_sparse.tflite".to_string())
    };
}

fn default_if_cargo_run(file: String) -> Option<PathBuf> {
    match get_project_dir() {
        Some(project_dir) => {
            project_dir.push("data");
            project_dir.push(file);
            Some(project_dir)
        }
        None => None,
    }
}
//not meant for use outside of project, goal is to get project directory if ran with cargo run
fn get_project_dir() -> Option<PathBuf> {
    match current_exe().unwrap().parent() {
        Some(path) if path.ends_with("target") || path.ends_with("release") => {
            if let Ok(res) = String::from_utf8(
                Command::new("cargo")
                    .arg("locate-project")
                    .arg("--message-format")
                    .arg("plain")
                    .output()
                    .expect("failed to execute process")
                    .stdout,
            ) {
                Some(PathBuf::from(res))
            } else {
                None
            }
        }
        _ => None,
    }
}
