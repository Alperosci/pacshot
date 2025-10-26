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

fn handle_ps() {
    let output = Command::new("pacman").arg("-Qq").output().unwrap();
    let paketler = String::from_utf8_lossy(&output.stdout);
    std::fs::write("pacs.pxs", paketler.as_ref()).unwrap();

    println!("Snapshot saved succesfully!");
}

fn handle_us() -> io::Result<()> {
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
        let installed_output = Command::new("pacman").arg("-Qq").output().unwrap();
        let installed_str = String::from_utf8_lossy(&installed_output.stdout);
        let installed: Vec<String> = installed_str
            .lines()
            .map(|s| s.trim().to_string())
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

            let status = Command::new("pacman")
                .arg("--noconfirm")
                .arg("--needed")
                .arg("-S")
                .arg(l)
                .status()?;

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
       V-0.1
"
    );

    let mut mode: String;
    loop {
        mode = input("What mode do you want? (Create[1] - Use[2] - Exit[3])\n > ");
        println!("");

        if mode == "1" {
            handle_ps();
        } else if mode == "2" {
            let _ = handle_us();
        } else if mode == "3" {
            break;
        } else {
            println!("Unknown option ...")
        }
        println!("");
    }
}
