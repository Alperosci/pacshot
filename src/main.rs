use std::fs;
#[allow(unused_imports)]
use std::fs::File;
use std::io::{self, Write, *};
use std::path::Path;
use std::process::Command;

fn input(inp: &str) -> String {
    print!("{}", inp);
    io::stdout().flush().unwrap();

    let mut inpbuffer = String::new();
    io::stdin().read_line(&mut inpbuffer).unwrap();

    return inpbuffer.trim().to_string();
}

fn read_lines(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line_res in reader.lines() {
        let line = line_res?;
        lines.push(line);
    }
    Ok(lines)
}

fn handle_ps(packetm: &str) {
    let output = if packetm == "pacman" {
        Command::new("pacman").arg("-Qq").output().unwrap()
    } else if packetm == "apt" {
        Command::new("apt")
            .arg("list")
            .arg("--installed")
            .arg("-qq")
            .arg("|")
            .arg("cut")
            .arg("-d'/'")
            .arg("-f1")
            .output()
            .unwrap()
    } else {
        panic!("Unsupported ...");
    };

    let paketler = String::from_utf8_lossy(&output.stdout);

    let name = input("Enter file name : ");

    std::fs::write(format!("{}.pxs", name), paketler.as_ref()).unwrap();

    println!("Snapshot saved succesfully!");
}

fn handle_us(packetm: &str) -> io::Result<()> {
    let uid = unsafe { libc::getuid() };
    if uid != 0 {
        println!("Root needed ...");
        return Ok(());
    }

    let inppath = input("Enter path : ");
    let path = Path::new(&inppath);

    if !path.exists() {
        println!("Please enter a valid path ...");
        return Ok(());
    } else {
        let installed_output = if packetm == "pacman" {
            Command::new("pacman").arg("-Qq").output().unwrap()
        } else if packetm == "apt" {
            Command::new("apt")
                .arg("list")
                .arg("--installed")
                .output()
                .unwrap()
        } else {
            panic!("Unsupported ...");
        };

        let installed_str = String::from_utf8_lossy(&installed_output.stdout);
        let installed: Vec<String> = installed_str
            .lines()
            .filter(|l| !l.starts_with("Listing"))
            .map(|s| s.split('/').next().unwrap_or("").trim().to_string())
            .collect();

        let lines = read_lines(&inppath)?;

        let mut errors = false;

        for (i, l) in lines.iter().enumerate() {
            if installed.contains(l) {
                print!(
                    "\r{:?} - {:?} found, skipping ...                              ",
                    &i, &l
                );
                io::stdout().flush().unwrap();
                continue;
            }

            let status = if packetm == "pacman" {
                Command::new("pacman")
                    .arg("--noconfirm")
                    .arg("--needed")
                    .arg("-S")
                    .arg(l)
                    .status()?
            } else if packetm == "apt" {
                Command::new("apt")
                    .arg("install")
                    .arg("-y")
                    .arg(l)
                    .status()?
            } else {
                panic!("Unsupported ...");
            };

            if status.success() {
                println!(
                    "{:?} - {:?} installed successfully! {:?} to go ...",
                    &i,
                    &l,
                    (&lines.len() - &i) - 1
                )
            } else {
                eprintln!("An error occurred at '{:?}'...", &l);
                errors = true;
            }
        }
        if errors {
            print!("\rSnapshot used with Errors ...                            \n")
        } else {
            print!("\rSnapshot used successfully ...                           \n")
        }
    }
    Ok(())
}

fn main() {
    let pmanager: &str;
    let osrcontent = fs::read_to_string("/etc/os-release").unwrap();
    if osrcontent.contains("ID=debian") || osrcontent.contains("ID_LIKE=debian") {
        pmanager = "apt";
    } else if osrcontent.contains("ID=arch") || osrcontent.contains("ID_LIKE=arch") {
        pmanager = "pacman";
    } else {
        pmanager = "unknown";
    }

    println!(
        r"
        /$$$$$$$   /$$$$$$   /$$$$$$   /$$$$$$  /$$   /$$  /$$$$$$  /$$$$$$$$
       | $$__  $$ /$$__  $$ /$$__  $$ /$$__  $$| $$  | $$ /$$__  $$|__  $$__/
       | $$  \ $$| $$  \ $$| $$  \__/| $$  \__/| $$  | $$| $$  \ $$   | $$
       | $$$$$$$/| $$$$$$$$| $$      |  $$$$$$ | $$$$$$$$| $$  | $$   | $$
       | $$____/ | $$__  $$| $$       \____  $$| $$__  $$| $$  | $$   | $$
       | $$      | $$  | $$| $$    $$ /$$  \ $$| $$  | $$| $$  | $$   | $$
       | $$      | $$  | $$|  $$$$$$/|  $$$$$$/| $$  | $$|  $$$$$$/   | $$
       |__/      |__/  |__/ \______/  \______/ |__/  |__/ \______/    |__/
       V-0.3.1
"
    );

    let mut mode: String;
    loop {
        mode = input("What mode do you want? (Create[1] - Use[2] - Exit[3])\n > ");
        println!("");

        if mode == "1" {
            handle_ps(&pmanager);
        } else if mode == "2" {
            let _ = handle_us(&pmanager);
        } else if mode == "3" {
            break;
        } else {
            println!("Unknown option ...")
        }
        println!("");
    }
}
