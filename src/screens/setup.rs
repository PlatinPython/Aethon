use std::ops::DerefMut;
use std::path::{Path, PathBuf};

use crate::{Errors, CONFIG};
use iced::widget::{button, column, horizontal_space, radio, row, text_input};
use iced::{Command, Element, Length};
use sysinfo::{DiskExt, RefreshKind, System, SystemExt};

use crate::screens::main::Main;
use crate::screens::{centering_container, Messages, Screen, Screens};

#[derive(Debug, Clone)]
pub(crate) struct Setup {
    launchers: (Option<PathBuf>, Option<PathBuf>),
    selection: Option<Launcher>,
    path: PathBuf,
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    LauncherSelected(Launcher),
    LauncherPathChanged(String),
    Select,
    Selected(Option<PathBuf>),
    Continue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Launcher {
    Store,
    Legacy,
    Custom,
}

impl Screen for Setup {
    type Message = Message;

    fn update(&mut self, message: Self::Message) -> (Command<Messages>, Option<Screens>) {
        let (store_launcher, legacy_launcher) = &self.launchers;
        match message {
            Message::LauncherSelected(launcher) => {
                self.selection = Some(launcher);
                match launcher {
                    Launcher::Store => self.path = store_launcher.clone().unwrap_or(PathBuf::new()),
                    Launcher::Legacy => {
                        self.path = legacy_launcher.clone().unwrap_or(PathBuf::new())
                    }
                    Launcher::Custom => {}
                }
                (Command::none(), None)
            }
            Message::LauncherPathChanged(path_text) => {
                self.path = PathBuf::from(path_text);
                (Command::none(), None)
            }
            Message::Select => (
                Command::perform(select_launcher(), |option| {
                    Messages::Setup(Message::Selected(option))
                }),
                None,
            ),
            Message::Selected(Some(new_path)) => {
                self.path = new_path;
                (Command::none(), None)
            }
            Message::Selected(None) => (Command::none(), None),
            Message::Continue => {
                let path = self.path.clone();
                (
                    Command::perform(
                        async {
                            CONFIG.lock().await.deref_mut().launcher_path = Some(path);
                            CONFIG.lock().await.save().await
                        },
                        Messages::Save,
                    ),
                    Some(Main::new(self.path.clone()).into()),
                )
            }
        }
    }

    fn view(&self) -> Element<'_, Messages> {
        let path_widget = {
            let mut path_text = text_input(
                "Launcher path",
                self.path
                    .to_str()
                    .map(|path| {
                        if path.is_empty() {
                            if self
                                .selection
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
            if self
                .selection
                .is_some_and(|launcher| launcher == Launcher::Custom)
            {
                path_text =
                    path_text.on_input(|path| Messages::Setup(Message::LauncherPathChanged(path)));
            }

            let path_button = button("Select").on_press_maybe(
                if self
                    .selection
                    .is_some_and(|launcher| launcher == Launcher::Custom)
                {
                    Some(Messages::Setup(Message::Select))
                } else {
                    None
                },
            );

            row![path_text, path_button].spacing(10)
        };

        let options = column![
            radio("Store", Launcher::Store, self.selection, |launcher| {
                Messages::Setup(Message::LauncherSelected(launcher))
            }),
            radio("Legacy", Launcher::Legacy, self.selection, |launcher| {
                Messages::Setup(Message::LauncherSelected(launcher))
            }),
            radio("Custom", Launcher::Custom, self.selection, |launcher| {
                Messages::Setup(Message::LauncherSelected(launcher))
            }),
            path_widget,
            row![
                horizontal_space(Length::Fill),
                button("Continue").on_press_maybe(
                    if self.path.exists()
                        && self.path.is_file()
                        && self
                            .path
                            .extension()
                            .is_some_and(|extension| extension == "exe")
                    {
                        Some(Messages::Setup(Message::Continue))
                    } else {
                        None
                    }
                )
            ]
        ];
        centering_container(options.spacing(10)).into()
    }
}

impl From<Setup> for Screens {
    fn from(value: Setup) -> Self {
        Screens::Setup(value)
    }
}

pub(crate) async fn load_setup() -> Result<Setup, Errors> {
    CONFIG.lock().await.save().await?;

    let launchers = get_potential_locations();
    let (selection, path) = match &launchers {
        (None, None) => (None, PathBuf::new()),
        (Some(launcher), None) => (Some(Launcher::Store), launcher.clone()),
        (None, Some(launcher)) => (Some(Launcher::Legacy), launcher.clone()),
        (Some(launcher), Some(_)) => (Some(Launcher::Store), launcher.clone()),
    };
    Ok(Setup {
        launchers,
        selection,
        path,
    })
}

async fn select_launcher() -> Option<PathBuf> {
    rfd::AsyncFileDialog::new()
        .set_title("Select Minecraft Launcher")
        .add_filter("Application", &["exe"])
        .pick_file()
        .await
        .map(|file| file.path().to_path_buf())
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
