extern crate ansi_term;
extern crate getopts;
extern crate regex;
extern crate walkdir;

use ansi_term::Colour::Blue;
use getopts::Options;
use regex::Regex;
use std::env;
use std::path::PathBuf;
use walkdir::{WalkDir, WalkDirIterator};

fn print_usage(opts: Options) {
    let brief = format!("Usage: fnd [options] [SEARCHTEXT] [DIR]");
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args : Vec<String> = env::args().collect();
    let mut opts = Options::new();
    let mut re : Option<Regex> = None;
    let mut ci = false;

    opts.optflag("h", "help", "print this help menu");
    opts.optflag("r", "", "treat search text as regex");
    opts.optflag("i", "", "make search case insensitive");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(_) => {
            print_usage(opts);
            return;
        }
    };

    if matches.opt_present("h") {
        print_usage(opts);
        return;
    }

    let mut search = match matches.free.get(0) {
        Some(x) => x.clone(),
        _ => String::from("")
    };

    if matches.opt_present("i") {
        search = search.to_lowercase();
        ci = true;
    }

    if matches.opt_present("r") {
        re = Some(Regex::new(&search).unwrap());
    }

    let dir = match matches.free.get(1) {
        Some(d) => PathBuf::from(d),
        _ => env::current_dir().unwrap()
    };

    let mut walker = WalkDir::new(&dir).into_iter();

    loop {
        let entry = match walker.next() {
            None => break,
            Some(Err(_)) => { walker.skip_current_dir(); continue; },
            Some(Ok(entry)) => entry,
        };
        if !entry.path().is_dir() {
            let name = match entry.path().file_name() {
                Some(nm) => nm.to_str().unwrap(),
                _ => ""
            };
            let mut namecheck = name.to_string();
            if ci {
                namecheck = name.to_lowercase();
            };

            if let Some(ref reg) = re {
                if reg.is_match(&namecheck) {
                    println!("{} - {}", Blue.paint(format!("{:30.30}", name)), entry.path().display());
                }
            } else {
                if (&namecheck).contains(&search) {
                    println!("{} - {}", Blue.paint(format!("{:30.30}", name)), entry.path().display());
                }
            }
        }
    }
}
