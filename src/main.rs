use std::error::Error;
use std::fs;

use serde_json::{json, Value};

fn main() -> Result<(), Box<dyn Error>> {
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
    Ok(())
}
