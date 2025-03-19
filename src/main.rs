use chrono::{Datelike, Local};
use std::env;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::process;

fn main() {
    // Fetch Arguments
    let mut output_folder: String = String::new();
    let mut fmt_path: String = String::new();
    let mut ext: String = String::from("md");

    let args: Vec<String> = env::args().collect();

    for arg in args {
        if arg.starts_with("-o=") {
            output_folder = arg.replace("-o=", "")
        }

        if arg.starts_with("-ext=") {
            ext = arg.replace("-ext=", "")
        }

        if arg.starts_with("-fmt=") {
            fmt_path = arg.replace("-fmt=", "")
        }
    }

    if output_folder.len() > 0 && !output_folder.ends_with("/") {
        output_folder.push_str("/");
    }

    let today = Local::now().date_naive();
    let day_number = today.ordinal();
    let date_string = today.format("%d.%m.%y").to_string();

    let file_path = format!("{}#{}-{}.{}", output_folder, day_number, date_string, ext);
    println!("File Name: {}", file_path);

    let mut file: fs::File;

    match fs::OpenOptions::new().append(true).create(true).open(&file_path) {
        Ok(of) => file = of,
        Err(_oe) => {
          println!("Unable to create or find the journal file.");
          println!("Exiting process with code 1.");
          process::exit(1);
        },
    }

    if fmt_path.len() > 0 {
        match fs::File::open(&fmt_path) {
            Ok(mut ffc) => {
                let mut context = String::new();
                ffc.read_to_string(&mut context)
                    .expect("File can't be read.");

                match file.write_all(context.as_bytes()) {
                    Ok(_e) => {
                        println!("[COMPLETED] Wrote format to the journal file");
                    }
                    Err(e) => {
                        println!("Err: Cannot write to the journal file");
                        println!("{}", e);
                        process::exit(0)
                    }
                }
            }
            Err(fre) => {
                println!("Err: Cannot read format from path '{}'", fmt_path);
                println!("{}", fre);
                process::exit(0);
            }
        }
    }
}
