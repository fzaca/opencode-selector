use std::io;

use anyhow::{Context, Result};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use opencode_selector::cli::{Cli, Command};
use opencode_selector::config::Config;
use opencode_selector::db::SessionRepository;
use opencode_selector::folders::FolderStore;
use opencode_selector::opencode;
use opencode_selector::tui::theme::Theme;
use opencode_selector::tui::ui::draw;
use opencode_selector::tui::{App, AppEvent, next_event};

fn main() -> Result<()> {
    color_eyre::install().expect("failed to install error handler");
    let cli = Cli::parse_args();
    let mut config = Config::new().context("failed to load configuration")?;

    if let Some(ref db) = cli.db {
        config = config.with_opencode_db_path(db);
    }
    if let Some(ref folders) = cli.folders {
        config = config.with_folders_path(folders);
    }

    config.ensure_dirs()?;

    match cli.command {
        Some(Command::Session { id }) => {
            opencode::launch_session(&id)?;
            return Ok(());
        }
        Some(Command::List { archived: _ }) => {
            list_sessions(&config)?;
            return Ok(());
        }
        None => {}
    }

    run_tui(config)?;
    Ok(())
}

fn list_sessions(config: &Config) -> Result<()> {
    let repo = SessionRepository::open(config.opencode_db_path())
        .context("failed to open opencode database")?;
    let sessions = repo.list_sessions().context("failed to list sessions")?;
    println!("{}", serde_json::to_string_pretty(&sessions)?);
    Ok(())
}

fn run_tui(config: Config) -> Result<()> {
    let repo = SessionRepository::open(config.opencode_db_path())
        .context("failed to open opencode database")?;
    let sessions = repo.list_sessions().context("failed to list sessions")?;

    let mut store =
        FolderStore::open(config.folders_path()).context("failed to open folder store")?;
    let folders = store.folders().to_vec();
    let mappings = store.session_folder_map();

    let cwd = std::env::current_dir().context("failed to get current directory")?;
    let project_filter = repo
        .find_project_for_path(&cwd)
        .context("failed to resolve project for current directory")?;

    let mut app = App::new(sessions, folders, mappings, project_filter);

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let theme = Theme::terminal();
    let result = run_app(&mut terminal, &mut app, &repo, &mut store, theme);

    terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    match result? {
        AppEvent::LaunchSession { id } => opencode::launch_session(&id)?,
        AppEvent::LaunchNew => opencode::launch_new()?,
        AppEvent::Quit | AppEvent::Continue => {}
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    repo: &SessionRepository,
    store: &mut FolderStore,
    theme: Theme,
) -> io::Result<AppEvent> {
    terminal.draw(|f| draw(f, app, theme))?;

    loop {
        let event = next_event(app, repo, store)?;
        if event != AppEvent::Continue {
            return Ok(event);
        }
        terminal.draw(|f| draw(f, app, theme))?;
    }
}
