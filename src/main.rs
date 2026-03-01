mod terminal_emulation;
mod tui_window;
mod keyboard;
mod desktop;
mod shortcut;
mod utils;
mod args;

use std::fs;
use crate::args::Args;
use crate::desktop::MyDesktop;
use crate::shortcut::parse_shortcut_dir;
use appcui::backend::Type;
use appcui::prelude::{App, Theme};
use appcui::system::Themes;
use clap::Parser;
use std::process::exit;
use directories::ProjectDirs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let shortcut_dir = args.shortcut_dir.unwrap_or_else(|| {
        let project_directory = ProjectDirs::from(
            "com",
            "Julien-cpsn",
            "desktop-tui"
        )
            .unwrap();

        let config_directory = project_directory.config_dir().to_path_buf();

        // Create the config dir if it does not exist
        if !config_directory.exists() {
            fs::create_dir_all(&config_directory).expect(&format!("Could not recursively create folder \"{}\"", config_directory.display()));
        }

        config_directory
    });

    let desktop_shortcuts = parse_shortcut_dir(shortcut_dir)?;

    let theme = Theme::new(Themes::Default);

    //theme.menu.text.normal = CharAttribute::new(Color::Blue, Color::Black, CharFlags::None);
    //theme.text.enphasized_2 = CharAttribute::new(Color::Red, Color::Green, CharFlags::None);
    //theme.desktop.character = Character::new(' ', Color::RGB(255, 255, 255), Color::RGB(85, 85, 85), CharFlags::None);

    let app = App::with_backend(Type::CrossTerm)
        .desktop(MyDesktop::new(desktop_shortcuts))
        .app_bar()
        .theme(theme)
        .color_schema(false)
        .build()?;

    app.run();

    exit(0);
}