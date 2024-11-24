use anyhow::Result;
use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use lib_wabbaauto::{test_downloader, AppState, DownloadManager, ModlistStatus};
use log::LevelFilter;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Row, Table, Widget},
    DefaultTerminal, Frame,
};
use simplelog::{CombinedLogger, Config, WriteLogger};
use std::{borrow::Borrow, fs};
use std::{fs::File, io, time::Duration};
use tokio::sync::watch;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    status_json: String,

    #[arg(short, long)]
    download_dir: String,
}

fn read_status_json(status_path: &str) -> Result<ModlistStatus> {
    assert!(
        fs::exists(status_path)?,
        "Provided status path does not exist! Make sure you have provided the right path"
    );
    let status = fs::read_to_string(status_path)?;
    Ok(serde_json::from_str(status.as_str())?)
}

fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello World!");
    frame.render_widget(text, frame.area());
}

#[derive(Debug)]
pub struct App {
    exit: bool,
    download_dir: String,
    modlist_name: String,
    modlist_version: String,
    state_receiver: watch::Receiver<AppState>,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let state = self.state_receiver.borrow();

        let n_modlists = (state.total).to_string();
        let downloads = state.successes.to_string();
        let failed = state.failed.to_string();
        let status_rows = vec![
            Row::new(vec!["Modlist", self.modlist_name.as_str()]),
            Row::new(vec!["Version", self.modlist_version.as_str()]),
            Row::new(vec!["Modlists to download", n_modlists.as_str()]),
            Row::new(vec!["Downloaded successfully", downloads.as_str()]),
            Row::new(vec!["Failed", failed.as_str()]),
        ];

        let failed_rows = state
            .recent_fails
            .iter()
            .map(|x| Row::new(vec![x.to_owned()]))
            .collect::<Vec<_>>();

        let download_rows = state
            .getting_archives
            .iter()
            .map(|(name, status)| Row::new(vec![name.to_owned(), format!("{}", status).to_owned()]))
            .collect::<Vec<_>>();

        let widths = [Constraint::Percentage(50), Constraint::Percentage(50)];

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let upper_top = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[0]);

        Table::new(status_rows, widths)
            .column_spacing(1)
            .style(Style::new().white())
            .block(Block::bordered().border_set(border::PLAIN).title("Main"))
            .render(upper_top[0], buf);

        Table::new(download_rows, widths)
            .column_spacing(1)
            .style(Style::new().green())
            .block(
                Block::bordered()
                    .border_set(border::PLAIN)
                    .title("Downloading")
                    .title_bottom("Press q to quit"),
            )
            .render(layout[1], buf);

        Table::new(failed_rows, [Constraint::Percentage(100)])
            .column_spacing(1)
            .style(Style::new().red())
            .block(
                Block::bordered()
                    .border_set(border::PLAIN)
                    .title("Failed mods (recent)"),
            )
            .render(upper_top[1], buf);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let status_path = args.status_json;
    let download_dir = args.download_dir;
    let status = read_status_json(status_path.as_str())?;
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        File::create("log.log").unwrap(),
    )])
    .unwrap();

    let mut terminal = ratatui::init();

    let (tx, rx) = watch::channel(AppState::default());

    let status_copy = status.clone();
    let download_dir_copy = download_dir.clone();

    let handle = tokio::spawn(async move {
        let mut downloader = DownloadManager::new(status_copy, download_dir_copy);
        downloader.start(tx).await;
    });

    if !fs::exists(download_dir.clone())? {
        fs::create_dir_all(download_dir.clone())?;
    }

    let _ = App {
        modlist_name: status.name.clone(),
        modlist_version: status.version.clone(),
        download_dir: download_dir.clone(),
        state_receiver: rx,
        exit: false,
    }
    .run(&mut terminal);

    handle.abort();
    ratatui::restore();

    Ok(())
}
