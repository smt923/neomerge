extern crate regex;

use regex::Regex;

use std::fs::{copy, File};
use std::process::exit;
use std::fs::OpenOptions;
use std::path::Path;
use std::io::prelude::*;
use std::io;

fn main() {
    println!("Checking for existing mods.cfg...");

    let config = Path::new("mods.cfg");

    if !config.exists() {
        create_config(config);
    }

    let mut modsfile = OpenOptions::new()
        .read(true)
        .open("mods.cfg")
        .expect("Could not open file");
    let mut contents = String::new();
    modsfile
        .read_to_string(&mut contents)
        .expect("Could not read mods.cfg");

    let mods = read_mods(contents);
    backup_getmods();

    let mut getmods = OpenOptions::new()
        .read(true)
        .write(true)
        .open("getmods.php")
        .expect("Could not open file");

    getmods.set_len(0).expect("Could not clear getmods.php");
    let mut contents = String::new();

    let mut i = 0;
    for m in mods {
        let folder = m.0;
        let name = m.1;

        contents.push_str(&format!(
            "&strModName{2}={1}&strModURL{2}={0}\r\n",
            folder,
            name,
            i
        ));
        i += 1;
    }

    let mut contents_rows = String::from(format!("nRows={}\r\n", i));
    contents_rows.push_str(&contents);

    getmods
        .write(contents_rows.as_bytes())
        .expect("Could not save new getmods.php");

    println!("Finished!");
    pause();
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we
    // print without a newline and flush manually.
    write!(stdout, "Press enter to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn read_mods(file: String) -> Vec<(String, String)> {
    // (folder, mod name)
    let mut mods: Vec<(String, String)> = vec![];

    for l in file.lines() {
        if l.starts_with("#") {
            continue;
        }
        let pair: Vec<&str> = l.split("=").collect();
        if pair.len() > 1 {
            mods.push((pair[0].trim().to_string(), pair[1].trim().to_string()))
        }
    }
    mods
}

fn backup_getmods() {
    println!(
        "Backing up 'getmods.php' - to restore rename 'BACKUP_getmods.php' back to 'getmods.php'!\n
NOTE: This backup is overwritten every time you run this program, so if your changes didn't work,
make sure you make your own copy of your backup so you can resetore to something that worked!\n"
    );
    if let Err(e) = copy("getmods.php", "BACKUP_getmods.php") {
        eprintln!("Could not backup getmods.php - exiting\n{}", e);
        exit(1)
    };
}

fn create_config(path: &std::path::Path) {
    File::create(&path).expect("Could not create mods.cfg");

    let mut getmods = File::open("getmods.php").expect("Could not open file");
    let mut mods = OpenOptions::new()
        .read(true)
        .write(true)
        .open("mods.cfg")
        .expect("Could not open file");

    let re = Regex::new(
        r#"=(?P<name>[A-Za-z0-9()\\/-]*)&strModURL[0-9]=(?P<folder>[A-Za-z0-9()\\/-]*)"#,
    ).unwrap();
    let mut getmods_contents = String::new();
    getmods
        .read_to_string(&mut getmods_contents)
        .expect("Could not read getmods.php");

    let mut modsvec = vec![];

    for line in getmods_contents.lines() {
        println!("{}", line);
        let caps = re.captures(line);
        if let Some(caps) = caps {
            let name = caps.get(1).map_or("", |m| m.as_str());
            let folder = caps.get(2).map_or("", |m| m.as_str());

            modsvec.push((folder, name));
        }
    }
    let mut mods_toml = String::new();

    for pair in modsvec {
        mods_toml.push_str(&format!("{} = {}\n", pair.0, pair.1))
    }

    let mut help = String::from(
        "# FORMAT: \"folder\" = \"modname\"
# Quotes on the folder name are optional, but use them if it has special characters or symbols\n\n",
    );

    if cfg!(windows) {
        mods_toml = mods_toml.replace("\n", "\r\n");
        help = help.replace("\n", "\r\n");
    }

    mods.write(help.as_bytes())
        .expect("Could not save mods.cfg");
    mods.write(mods_toml.as_bytes())
        .expect("Could not save mods.cfg");

    println!(
        "Config has been created! If you add new mods to this config, run this program again
and it will read the config and update the getmods.php file"
    );
    pause();
    exit(1)
}
