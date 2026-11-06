use crossterm::event::{self, Event, KeyCode};
use ratatui::Frame;
use ratatui::widgets::{Block, List, ListState};

use crate::manifest::Manifest;
use std::io;
use std::path::{Path, PathBuf};

pub struct App {
    manifest: Manifest,
    path: PathBuf,
    state: ListState,
    //ilk d basildi mi, ikinci d bekleniyor mu?
    pending_d: bool,
    should_quit: bool,
}

impl App {
    pub fn new(manifest: Manifest, path: PathBuf) -> Self {
        Self {
            manifest,
            path,
            state: ListState::default().with_selected(Some(0)),
            pending_d: false,
            should_quit: false,
        }
    }

    // tust gelince durumu degistirir, cizim yapmaz

    fn handle_key(&mut self, code: KeyCode) -> io::Result<()> {
        // d disinda bir tus gelirse bekleyen d iptal olur
        let was_pending = self.pending_d;
        self.pending_d = false;

        match code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('j') | KeyCode::Down => self.state.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.state.select_previous(),
            KeyCode::Char('d') if was_pending => self.delete_selected()?,
            KeyCode::Char('d') => self.pending_d = true,
            _ => {}
        }
        Ok(())
    }

    // secili todoyu siler ve dosyaya kaydeder
    fn delete_selected(&mut self) -> io::Result<()> {
        let items = &mut self.manifest.todo.items;

        let Some(index) = self.state.selected() else {
            return Ok(());
        };
        if index >= items.len() {
            return Ok(());
        }

        items.remove(index);

        // son satir silindiyse secimi bir yukari al
        if index >= items.len() {
            self.state.select(items.len().checked_sub(1));
        }

        self.manifest.save(&self.path)
    }

    // durumu ekrana cizer
    // fn draw(&mut self, frame: &mut Frame)
    fn draw(&mut self, frame: &mut Frame) {
        let list = List::new(self.manifest.todo.items.clone())
            .block(Block::bordered().title("alzh board"))
            .highlight_symbol("> ");
        frame.render_stateful_widget(list, frame.area(), &mut self.state);
    }
}

// ana dongu
pub fn run(path: &Path) -> std::io::Result<()> {
    let manifest = Manifest::load(path)?;
    let mut app = App::new(manifest, path.to_path_buf());

    ratatui::run(|terminal| {
        while !app.should_quit {
            terminal.draw(|frame| app.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                if key.is_press() {
                    app.handle_key(key.code);
                }
            }
        }
        Ok(())
    })
}
