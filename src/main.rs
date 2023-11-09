#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command as Program;
use std::{env, fs, process};

use iced::widget::{
    button, column, container, horizontal_space, radio, row, text, text_input, Container,
};
use iced::{executor, Application, Command, Element, Length, Settings, Theme};
use once_cell::sync::Lazy;
use serde_json::{json, Value};
use single_instance::SingleInstance;
use sysinfo::{DiskExt, RefreshKind, System, SystemExt};

const UUID: &str = "aethon-f082c8ab-df27-4daf-9d09-48ff15ef0204";

static INSTANCE: Lazy<SingleInstance> =
    Lazy::new(|| SingleInstance::new(UUID).expect("SingleInstance object creation failed."));

#[derive(Debug, Clone)]
enum Manager {
    Startup,
    FolderNotEmptyWarn,
    SingleInstanceWarn,
    Setup {
        launchers: (Option<PathBuf>, Option<PathBuf>),
        selection: Option<Launcher>,
        path: PathBuf,
    },
    Main(PathBuf),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Launcher {
    Store,
    Legacy,
    Custom,
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Manager),
    WarnContinue,
    WarnClose,
    LauncherSelected(Launcher),
    LauncherPathChanged(String),
    Select,
    Selected(Option<PathBuf>),
    Continue,
    Run,
}

impl Application for Manager {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (Manager::Startup, Command::perform(load(), Message::Loaded))
    }

    fn title(&self) -> String {
        String::from("Aethon")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match self {
            Manager::Startup => {
                if let Message::Loaded(manager) = message {
                    *self = manager;
                }
                Command::none()
            }
            Manager::FolderNotEmptyWarn => match message {
                Message::WarnContinue => Command::perform(load_setup(), Message::Loaded),
                Message::WarnClose => process::exit(0),
                Message::Loaded(manager) => {
                    *self = manager;
                    Command::none()
                }
                _ => Command::none(),
            },
            Manager::SingleInstanceWarn => {
                if let Message::WarnClose = message {
                    process::exit(0);
                }
                Command::none()
            }
            Manager::Setup {
                launchers: (store_launcher, legacy_launcher),
                selection,
                path,
            } => match message {
                Message::LauncherSelected(launcher) => {
                    *selection = Some(launcher);
                    match launcher {
                        Launcher::Store => *path = store_launcher.clone().unwrap_or(PathBuf::new()),
                        Launcher::Legacy => {
                            *path = legacy_launcher.clone().unwrap_or(PathBuf::new())
                        }
                        _ => {}
                    }
                    Command::none()
                }
                Message::LauncherPathChanged(path_text) => {
                    *path = PathBuf::from(path_text);
                    Command::none()
                }
                Message::Select => Command::perform(select_launcher(), Message::Selected),
                Message::Selected(Some(new_path)) => {
                    *path = new_path;
                    Command::none()
                }
                Message::Continue => {
                    *self = Manager::Main(path.clone());
                    Command::none()
                }
                _ => Command::none(),
            },
            Manager::Main(path) => {
                if let Message::Run = message {
                    run(path).unwrap()
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        match self {
            Manager::Startup => centering_container(text("Loading...")).into(),
            Manager::FolderNotEmptyWarn => {
                centering_container({
                    column![
                        container(text("Current folder is not empty, please move the executable to an empty folder (Recommended) or continue anyway (Not recommended).")).width(Length::Fill).center_x(),
                        row![
                            horizontal_space(Length::FillPortion(2)),
                            button("Continue").width(Length::Fill).on_press(Message::WarnContinue),
                            horizontal_space(Length::Fill),
                            button("Close").width(Length::Fill).on_press(Message::WarnClose),
                            horizontal_space(Length::FillPortion(2)),
                        ].spacing(10)
                    ].spacing(10)
                }).into()
            }
            Manager::SingleInstanceWarn => {
                centering_container({
                    column![
                    container(text("Another instance is already running.")).width(Length::Fill).center_x(),
                    row![
                        horizontal_space(Length::FillPortion(2)),
                        button("Ok").width(Length::Fill).on_press(Message::WarnClose),
                        horizontal_space(Length::FillPortion(2)),
                    ].spacing(10)
                ].spacing(10)
                }).into()
            }
            Manager::Setup {
                selection, path, ..
            } => {
                let path_widget = {
                    let mut path_text = text_input(
                        "Launcher path",
                        path.to_str()
                            .map(|path| {
                                if path.is_empty() {
                                    if selection
                                        .is_some_and(|launcher| launcher == Launcher::Custom)
                                    {
                                        ""
                                    } else {
                                        "Not found"
                                    }
                                } else {
                                    path
                                }
                            })
                            .unwrap_or(""),
                    );
                    if selection.is_some_and(|launcher| launcher == Launcher::Custom) {
                        path_text = path_text.on_input(Message::LauncherPathChanged);
                    }

                    let path_button = button("Select").on_press_maybe(
                        if selection.is_some_and(|launcher| launcher == Launcher::Custom) {
                            Some(Message::Select)
                        } else {
                            None
                        },
                    );

                    row![path_text, path_button].spacing(10)
                };

                let options = column![
                    radio(
                        "Store",
                        Launcher::Store,
                        *selection,
                        Message::LauncherSelected
                    ),
                    radio(
                        "Legacy",
                        Launcher::Legacy,
                        *selection,
                        Message::LauncherSelected
                    ),
                    radio(
                        "Custom",
                        Launcher::Custom,
                        *selection,
                        Message::LauncherSelected
                    ),
                    path_widget,
                    row![
                        horizontal_space(Length::Fill),
                        button("Continue").on_press_maybe(
                            if path.exists()
                                && path.is_file()
                                && path.extension().is_some_and(|extension| extension == "exe")
                            {
                                Some(Message::Continue)
                            } else {
                                None
                            }
                        )
                    ]
                ];
                centering_container(options.spacing(10)).into()
            }
            Manager::Main(_) => centering_container(button("Run").on_press(Message::Run)).into(),
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn main() -> Result<(), iced::Error> {
    Manager::run(Settings::default())
}

fn run(launcher_path: &Path) -> Result<(), Box<dyn Error>> {
    println!("Selected path: {:?}", launcher_path);

    add_profile()?;

    Program::new(launcher_path).spawn()?;
    Ok(())
}

fn add_profile() -> Result<(), Box<dyn Error>> {
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

async fn select_launcher() -> Option<PathBuf> {
    rfd::AsyncFileDialog::new()
        .set_title("Select Minecraft Launcher")
        .add_filter("Application", &["exe"])
        .pick_file()
        .await
        .map(|m| m.path().to_path_buf())
}

async fn load() -> Manager {
    if !INSTANCE.is_single() {
        return Manager::SingleInstanceWarn;
    }
    if env::current_exe()
        .ok()
        .as_deref()
        .and_then(Path::parent)
        .map(Path::read_dir)
        .and_then(Result::ok)
        .map(|dir| dir.filter_map(Result::ok))
        .map(|dir| dir.count() > 1)
        .unwrap_or(true)
    {
        return Manager::FolderNotEmptyWarn;
    }
    load_setup().await
}

async fn load_setup() -> Manager {
    let launchers = get_potential_locations();
    let (selection, path) = match &launchers {
        (None, None) => (None, PathBuf::new()),
        (Some(launcher), None) => (Some(Launcher::Store), launcher.clone()),
        (None, Some(launcher)) => (Some(Launcher::Legacy), launcher.clone()),
        (Some(launcher), Some(_)) => (Some(Launcher::Store), launcher.clone()),
    };
    Manager::Setup {
        launchers,
        selection,
        path,
    }
}

fn get_potential_locations() -> (Option<PathBuf>, Option<PathBuf>) {
    let mut store_launcher = None;
    let mut legacy_launcher = None;

    for drive in get_drives() {
        if store_launcher.is_none() {
            let launcher = drive.join("XboxGames/Minecraft Launcher/Content/Minecraft.exe");
            if launcher.exists() {
                store_launcher = Some(launcher);
            }
        }
        if legacy_launcher.is_none() {
            let launcher =
                drive.join("Program Files (x86)/Minecraft Launcher/MinecraftLauncher.exe");
            if launcher.exists() {
                legacy_launcher = Some(launcher);
            }
        }
    }

    (store_launcher, legacy_launcher)
}

fn get_drives() -> Vec<PathBuf> {
    let sys = System::new_with_specifics(RefreshKind::new().with_disks_list());
    sys.disks()
        .iter()
        .map(DiskExt::mount_point)
        .map(Path::to_path_buf)
        .collect()
}

fn centering_container<'a, Message>(
    content: impl Into<Element<'a, Message>>,
) -> Container<'a, Message> {
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .padding(10)
}
