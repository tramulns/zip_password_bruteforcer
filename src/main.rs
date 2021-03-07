use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() -> Result<(), std::io::Error> {
    let matches = App::new("Zip Password Brute Forcer")
        .version("0.1.0")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("ZIP File Address"),
        )
        .arg(
            Arg::with_name("word")
                .short("w")
                .long("word")
                .takes_value(true)
                .help("Password List Address"),
        )
        .get_matches();

    let zipfile = matches
        .value_of("file")
        .expect("Please check the ZIP file's path");
    let word_list = matches
        .value_of("word")
        .expect("Please check the Password List file's path");

    let file = File::open(zipfile)?;
    let mut archive = zip::ZipArchive::new(&file)?;

    let mut password = "".to_string();
    let mut attempts = 0;

    let start = Instant::now();
    let mut duration = start.elapsed();
    if let Ok(lines) = read_lines(word_list) {
        'passwords: for line in lines {
            if let Ok(passes) = line {
                attempts += 1;
                let mut count_decrypt_success = 0;
                for i in 0..archive.len() {
                    let file = archive.by_index_decrypt(i, passes.as_bytes());
                    match file {
                        Ok(Ok(_)) => count_decrypt_success += 1,
                        _ => continue 'passwords,
                    };
                }
                if count_decrypt_success == archive.len() {
                    password = passes;
                    duration = start.elapsed();
                    break 'passwords;
                }
            }
        }
    }

    if password.is_empty() {
        println!(" [X] Sorry, Password Not Found :(");
    } else {
        println!(" [*] Password Found :)");
        println!(" [*] Password: {}", password);
        println!(
            " [***] Took {} seconds. That is, {} attempts per second.",
            duration.as_secs(),
            if duration.as_secs() == 0 {
                attempts as f64
            } else {
                attempts as f64 / duration.as_secs() as f64
            }
        )
    }

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
