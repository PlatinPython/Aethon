use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;
use std::process::{exit, Command};
use std::{fs, str};

use serde_json::{json, Value};

fn main() -> Result<(), Box<dyn Error>> {
    let launcher_path = get_install_location();
    println!("Selected path: {:?}", launcher_path);

    let path = dirs::config_dir()
        .map(|path| path.join(".minecraft/launcher_profiles.json"))
        .expect("Path should exist.");
    let mut value = serde_json::from_str::<Value>(&fs::read_to_string(&path)?)?;
    if !value["profiles"].is_object() {
        value["profiles"] = json!({});
    }
    let profiles = &mut value["profiles"];
    profiles["aethon"] = json!({
        "name": "Aethon",
        "type": "custom",
        "icon": "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACAAQMAAAD58POIAAAABlBMVEUAAAD4APit1uGJAAAAI0lEQVRIx2P4DwUMMDAqMCowKjAqQKTAaDCMCowKjAqQKQAABpD8LlM5SL4AAAAASUVORK5CYII",
        "lastVersionId": "latest-release"
    });
    fs::write(&path, serde_json::to_string(&value)?)?;

    Command::new(launcher_path).spawn()?;
    Ok(())
}

fn get_install_location() -> PathBuf {
    match get_potential_locations() {
        None => {
            print!("Found no install location. Please install the vanilla launcher. Would you like to provide a different path to check? [Y/n] ");
            stdout().flush().unwrap();
            let mut answer = String::new();
            stdin().read_line(&mut answer).unwrap();
            if !answer.trim().eq_ignore_ascii_case("y") {
                println!("Abort.");
                exit(1);
            }
            loop {
                print!("Please provide the path: ");
                stdout().flush().unwrap();
                let mut path = String::new();
                stdin().read_line(&mut path).unwrap();
                let path = PathBuf::from(path.trim());
                if path.exists()
                    && path
                        .extension()
                        .map_or(false, |extension| extension.eq("exe"))
                {
                    return path;
                } else {
                    println!("Invalid path.");
                }
            }
        }
        Some(paths) => {
            println!("Found paths:");
            for (i, path) in paths.iter().enumerate() {
                println!("{}. {:?}", i + 1, path);
            }
            loop {
                print!("Please select one (1-{}) ", paths.len());
                stdout().flush().unwrap();
                let mut answer = String::new();
                stdin().read_line(&mut answer).unwrap();
                if let Ok(selection) = answer.trim().parse::<usize>() {
                    if selection - 1 < paths.len() {
                        return paths[selection - 1].clone();
                    } else {
                        println!("Number out of bounds.");
                    }
                } else {
                    println!("Failed to parse number.");
                }
            }
        }
    }
}

fn get_potential_locations() -> Option<Vec<PathBuf>> {
    let mut paths = vec![];
    if let Ok(drives) = get_drives() {
        for drive in drives {
            let launcher = PathBuf::from(format!(
                "{}/XboxGames/Minecraft Launcher/Content/Minecraft.exe",
                drive
            ));
            if launcher.exists() {
                paths.push(launcher);
            }
            let launcher = PathBuf::from(format!(
                "{}/Program Files (x86)/Minecraft Launcher/MinecraftLauncher.exe",
                drive
            ));
            if launcher.exists() {
                paths.push(launcher);
            }
        }
    } else {
        return None;
    }
    if paths.is_empty() {
        None
    } else {
        Some(paths)
    }
}

fn get_drives() -> Result<Vec<String>, Box<dyn Error>> {
    let output = Command::new("cmd")
        .args(["/C", "wmic logicaldisk get deviceid"])
        .output()?;

    let output = str::from_utf8(&output.stdout)?.trim();

    Ok(output
        .lines()
        .skip(1)
        .map(str::trim)
        .map(String::from)
        .collect())
}
