use std::{
    env::{self},
    io,
    path::PathBuf,
    time::Instant,
};

use fp::{
    app::{App, AppResult},
    cli::Cli,
    event::{Event, EventHandler},
    handler::handle_key_events,
    project::Project,
    theme::init_theme,
    tui::Tui,
};
use globwalk::GlobWalkerBuilder;
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> AppResult<()> {
    let cli: Cli = argh::from_env();
    init_theme(cli.theme);

    let searchpath = env::var("HOME")?;

    if !PathBuf::from(&searchpath).is_dir() {
        return Err(format!("'{}' is not a directory", searchpath).into());
    }

    let start = Instant::now();
    // Create an application.
    let mut app = App::new(
        GlobWalkerBuilder::from_patterns(
            searchpath,
            &[
                "{.git,Cargo.toml,package.json,Makefile,go.mod}",
                "!{node_modules,target,build,dist,cmake*,.*}",
            ],
        )
        .build()?
        .filter_map(|d| Some(Project::new(d.ok()?.into_path()))),
    );
    app.start_time = start.elapsed();

    // Initialize the terminal user interface.
    let terminal = Terminal::new(CrosstermBackend::new(io::stderr()))?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.state.is_running() {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            _ => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
