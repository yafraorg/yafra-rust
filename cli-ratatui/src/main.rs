use clap::{CommandFactory, Parser, Subcommand};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use mysql::prelude::Queryable;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::{
    env,
    fs,
    io,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "yafra-rust-cli")]
#[command(about = "A CLI tool example with TUI", long_about = None)]
#[command(version = VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List the contents of the current directory
    List,
    /// Show version information
    Version,
}

struct App {
    directory_items: Vec<String>,
    current_dir: String,
}

impl App {
    fn new() -> io::Result<App> {
        let current_dir = env::current_dir()?;
        let current_dir_str = current_dir.to_string_lossy().to_string();
        
        let mut items = Vec::new();
        let entries = fs::read_dir(&current_dir)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            
            if path.is_dir() {
                items.push(format!("📁 {}/", name));
            } else {
                items.push(format!("📄 {}", name));
            }
        }
        
        items.sort();
        
        Ok(App {
            directory_items: items,
            current_dir: current_dir_str,
        })
    }
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::List) => {
            run_tui_list()?;
        }
        Some(Commands::Version) => {
            println!("directory-lister version {}", VERSION);
        }
        None => {
            // Show help if no command is provided
            let mut cmd = Cli::command();
            cmd.print_help().unwrap();
        }
    }

    // Database connection setup
    let database_url = format!(
        "mysql://{}:{}@localhost:3306/{}",
        env::var("DB_USER").unwrap_or_else(|_| "root".to_string()),
        env::var("DB_PASSWORD").unwrap_or_else(|_| "root1234".to_string()),
        env::var("DB_NAME").unwrap_or_else(|_| "business".to_string())
    );
    println!("Database connection will be established to: {}", database_url);
    // Database query execution
    match mysql::Pool::new(database_url) {
        Ok(pool) => {
            match pool.get_conn() {
                Ok(mut conn) => {
                    let query = "SELECT * FROM transactions";
                    match conn.query_map(query, |row: mysql::Row| {
                        // Convert row to a string representation
                        format!("{:?}", row)
                    }) {
                        Ok(results) => {
                            println!("Orders table contents:");
                            for (i, row) in results.iter().enumerate() {
                                println!("Row {}: {}", i + 1, row);
                            }
                        }
                        Err(e) => println!("Error executing query: {}", e),
                    }
                }
                Err(e) => println!("Error getting connection: {}", e),
            }
        }
        Err(e) => println!("Error creating connection pool: {}", e),
    }
    
    Ok(())
}

fn run_tui_list() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    // Create app
    let app = App::new()?;
    let res = run_app(&mut terminal, app);
    
    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    
    if let Err(err) = res {
        println!("{:?}", err);
    }
    
    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;
        
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                _ => {}
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());
    
    // Title
    let title = Paragraph::new(vec![Line::from(vec![
        Span::styled(
            "Directory Lister",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
    ])])
    .block(Block::default().borders(Borders::ALL).title("Title"));
    f.render_widget(title, chunks[0]);
    
    // Directory listing
    let items: Vec<ListItem> = app
        .directory_items
        .iter()
        .map(|item| {
            ListItem::new(Line::from(Span::raw(item.as_str())))
        })
        .collect();
    
    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Contents of: {}", app.current_dir)),
        )
        .style(Style::default().fg(Color::White));
    f.render_widget(list, chunks[1]);
    
    // Help
    let help = Paragraph::new("Press 'q' or 'Esc' to quit")
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .style(Style::default().fg(Color::Gray));
    f.render_widget(help, chunks[2]);
}
