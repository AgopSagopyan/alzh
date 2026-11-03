use crossterm::event::{self,Event,KeyCode};
use ratatui::Frame;
use ratatui::widgets::{Block,List,ListState};

pub struct App {
    projects: Vec<String>,
    state: ListState,
    should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            projects: vec!["alzh".to_string(), "SeatVision".to_string()],
            state: ListState::default().with_selected(Some(0)),
            should_quit: false,
        }
    }

    // tust gelince durumu degistirir, cizim yapmaz
    
    fn handle_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('j') | KeyCode::Down => self.state.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.state.select_previous(),
            _ => {}
        }
    }

    // durumu ekrana cizer
    // fn draw(&mut self, frame: &mut Frame)
    fn draw(&mut self, frame: &mut Frame) {
        let list = List::new(self.projects.clone()).block(Block::bordered().title("alzh board")).highlight_symbol("> ");
        frame.render_stateful_widget(list, frame.area(), &mut self.state);
    }
}

// ana dongu
pub fn run() -> std::io::Result<()> {
    let mut app = App::new();

    ratatui::run(|mut terminal| {
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
