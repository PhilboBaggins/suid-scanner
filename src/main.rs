#[macro_use]
extern crate clap;
use clap::{App, Arg};

extern crate exitcode;
use std::process;

extern crate walkdir;
use walkdir::WalkDir;

use std::fs;
use std::os::unix::fs::MetadataExt;

fn scan_path(path: &str) -> std::io::Result<()> {
    for e in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if e.metadata()?.is_file() {
            let meta = fs::metadata(e.path())?;
            let mode = meta.mode();
            match mode & 0o007000 {
                0o007000 => println!("{:o} sug {}", mode, e.path().display()),
                0o006000 => println!("{:o}  ug {}", mode, e.path().display()),
                0o005000 => println!("{:o} su  {}", mode, e.path().display()),
                0o004000 => println!("{:o}  u  {}", mode, e.path().display()),
                0o003000 => println!("{:o}  sg {}", mode, e.path().display()),
                0o002000 => println!("{:o}   g {}", mode, e.path().display()),
                0o001000 => println!("{:o} s   {}", mode, e.path().display()),
                _ => { },
            }
        }
    }

    Ok(())
}

fn main() {
    let matches = App::new("suid-scanner")
        .version(crate_version!())
        .about("Look for files with the SUID bit set")
        .author("Phil B.")
        .arg(Arg::with_name("path")
            .help("Path to scan")
            .takes_value(true)
            .required(true)
            .multiple(true))
        .get_matches();

    for path in matches.values_of("path").unwrap() {
        match scan_path(path) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(exitcode::IOERR);
            }
        }
    }

    process::exit(exitcode::OK);
}
