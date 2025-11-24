// Ratatui-based Select UI with dynamic description

use crate::system::error::{DoumError, Result};
use crate::cli::menu::MenuItem;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use std::io;

/// Enhanced select with ratatui
pub fn ratatui_select(
    title: &str,
    items: &[MenuItem],
    subtitle: Option<&str>,
    current_value: Option<&str>,
) -> Result<Option<MenuItem>> {
    // Setup terminal
    enable_raw_mode().map_err(|e| DoumError::Config(format!("Failed to enable raw mode: {}", e)))?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)
        .map_err(|e| DoumError::Config(format!("Failed to enter alternate screen: {}", e)))?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)
        .map_err(|e| DoumError::Config(format!("Failed to create terminal: {}", e)))?;

    // Run the app
    let result = run_select_app(&mut terminal, title, items, subtitle, current_value);

    // Restore terminal
    disable_raw_mode().map_err(|e| DoumError::Config(format!("Failed to disable raw mode: {}", e)))?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .map_err(|e| DoumError::Config(format!("Failed to leave alternate screen: {}", e)))?;
    terminal.show_cursor()
        .map_err(|e| DoumError::Config(format!("Failed to show cursor: {}", e)))?;

    result
}

/// App state
struct SelectApp<'a> {
    items: &'a [MenuItem],
    state: ListState,
    title: String,
    subtitle: Option<String>,
    current_value: Option<String>,
}

impl<'a> SelectApp<'a> {
    fn new(title: &str, items: &'a [MenuItem], subtitle: Option<&str>, current_value: Option<&str>) -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        
        Self {
            items,
            state,
            title: title.to_string(),
            subtitle: subtitle.map(|s| s.to_string()),
            current_value: current_value.map(|s| s.to_string()),
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn get_selected(&self) -> Option<&MenuItem> {
        self.state.selected().and_then(|i| self.items.get(i))
    }
}

/// Run the select app
fn run_select_app<'a>(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    title: &str,
    items: &'a [MenuItem],
    subtitle: Option<&str>,
    current_value: Option<&str>,
) -> Result<Option<MenuItem>> {
    let mut app = SelectApp::new(title, items, subtitle, current_value);

    loop {
        terminal.draw(|f| ui(f, &mut app))
            .map_err(|e| DoumError::Config(format!("Failed to draw: {}", e)))?;

        if let Event::Key(key) = event::read()
            .map_err(|e| DoumError::Config(format!("Failed to read event: {}", e)))? 
        {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(None),
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous(),
                    KeyCode::Enter => {
                        return Ok(app.get_selected().cloned());
                    }
                    _ => {}
                }
            }
        }
    }
}

/// Draw the UI
fn ui(f: &mut Frame, app: &mut SelectApp) {
    // Create layout with more breathing room
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(2),  // Title (no border)
            Constraint::Min(3),     // List
            Constraint::Length(7),  // Description (increased for better visibility)
            Constraint::Length(1),  // Help
        ])
        .split(f.area());

    // Title - clean, no border
    let title_text = vec![
        Line::from(Span::styled(
            &app.title, 
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD)
        )),
    ];
    let title_widget = Paragraph::new(title_text);
    f.render_widget(title_widget, chunks[0]);

    // List items with minimal styling
    let items: Vec<ListItem> = app.items.iter().enumerate().map(|(idx, item)| {
        let is_selected = Some(idx) == app.state.selected();
        let is_current = app.current_value.as_ref().map_or(false, |cv| &item.id == cv);
        
        let mut label = String::new();
        let mut style = Style::default();
        
        // Selection indicator
        if is_selected {
            label.push_str("  › ");
            style = style.fg(Color::Cyan).add_modifier(Modifier::BOLD);
        } else {
            label.push_str("    ");
            style = style.fg(Color::Gray);
        }
        
        label.push_str(&item.label);
        
        // Current value indicator - more prominent
        if is_current {
            label.push_str(" ✓");
            if !is_selected {
                style = style.fg(Color::Green);
            }
        }
        
        // Special colors for back/exit - more visible
        if item.id == "back" {
            style = if is_selected {
                style.fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                style.fg(Color::Yellow)
            };
        } else if item.id == "exit" {
            style = if is_selected {
                style.fg(Color::Red).add_modifier(Modifier::BOLD)
            } else {
                style.fg(Color::Red)
            };
        }
        
        ListItem::new(Line::from(Span::styled(label, style)))
    }).collect();

    let list = List::new(items)
        .block(Block::default()
            .borders(Borders::NONE));

    f.render_stateful_widget(list, chunks[1], &mut app.state);

    // Description - more prominent with better spacing
    let description = if let Some(selected) = app.get_selected() {
        selected.description.clone()
    } else {
        String::from("")
    };

    let desc_lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Description", 
            Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM)
        )),
        Line::from(""),
        Line::from(Span::styled(
            description, 
            Style::default().fg(Color::White)
        )),
    ];

    let desc_widget = Paragraph::new(desc_lines)
        .block(Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(Color::DarkGray)));
    
    f.render_widget(desc_widget, chunks[2]);

    // Help text - minimal
    let help_text = if let Some(ref subtitle) = app.subtitle {
        subtitle.clone()
    } else {
        String::from("↑↓ j/k  •  ↵ select  •  esc cancel")
    };
    
    let help = Paragraph::new(Line::from(Span::styled(
        help_text,
        Style::default().fg(Color::DarkGray)
    )));
    f.render_widget(help, chunks[3]);
}

/// Text input with ratatui
pub fn ratatui_input(
    prompt: &str,
    default: Option<&str>,
    help: Option<&str>,
) -> Result<String> {
    // Setup terminal
    enable_raw_mode().map_err(|e| DoumError::Config(format!("Failed to enable raw mode: {}", e)))?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)
        .map_err(|e| DoumError::Config(format!("Failed to enter alternate screen: {}", e)))?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)
        .map_err(|e| DoumError::Config(format!("Failed to create terminal: {}", e)))?;

    // Run the input app
    let result = run_input_app(&mut terminal, prompt, default, help);

    // Restore terminal
    disable_raw_mode().map_err(|e| DoumError::Config(format!("Failed to disable raw mode: {}", e)))?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .map_err(|e| DoumError::Config(format!("Failed to leave alternate screen: {}", e)))?;
    terminal.show_cursor()
        .map_err(|e| DoumError::Config(format!("Failed to show cursor: {}", e)))?;

    result
}

/// Input app state
struct InputApp {
    input: String,
    prompt: String,
    help: Option<String>,
    cursor_position: usize,
}

impl InputApp {
    fn new(prompt: &str, default: Option<&str>, help: Option<&str>) -> Self {
        let input = default.unwrap_or("").to_string();
        let cursor_position = input.len();
        
        Self {
            input,
            prompt: prompt.to_string(),
            help: help.map(|s| s.to_string()),
            cursor_position,
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, c: char) {
        self.input.insert(self.cursor_position, c);
        self.move_cursor_right();
    }

    fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            self.input.remove(self.cursor_position - 1);
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.min(self.input.len())
    }
}

/// Run input app
fn run_input_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    prompt: &str,
    default: Option<&str>,
    help: Option<&str>,
) -> Result<String> {
    let mut app = InputApp::new(prompt, default, help);

    loop {
        terminal.draw(|f| ui_input(f, &app))
            .map_err(|e| DoumError::Config(format!("Failed to draw: {}", e)))?;

        if let Event::Key(key) = event::read()
            .map_err(|e| DoumError::Config(format!("Failed to read event: {}", e)))? 
        {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Enter => return Ok(app.input.clone()),
                    KeyCode::Char(c) => app.enter_char(c),
                    KeyCode::Backspace => app.delete_char(),
                    KeyCode::Left => app.move_cursor_left(),
                    KeyCode::Right => app.move_cursor_right(),
                    KeyCode::Esc => return Err(DoumError::Config("Input cancelled".to_string())),
                    _ => {}
                }
            }
        }
    }
}

/// Draw input UI
fn ui_input(f: &mut Frame, app: &InputApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Prompt
            Constraint::Length(3),  // Input
            Constraint::Length(2),  // Help
        ])
        .split(f.area());

    // Prompt
    let prompt_text = vec![
        Line::from(Span::styled(&app.prompt, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
    ];
    let prompt_widget = Paragraph::new(prompt_text)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(prompt_widget, chunks[0]);

    // Input
    let input_widget = Paragraph::new(app.input.as_str())
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input_widget, chunks[1]);

    // Set cursor position
    f.set_cursor_position((chunks[1].x + app.cursor_position as u16 + 1, chunks[1].y + 1));

    // Help
    let help_text = if let Some(ref help) = app.help {
        help.clone()
    } else {
        String::from("Enter: Submit | Esc: Cancel")
    };
    
    let help = Paragraph::new(Line::from(Span::styled(
        help_text,
        Style::default().fg(Color::DarkGray)
    )));
    f.render_widget(help, chunks[2]);
}

/// Password input with ratatui
pub fn ratatui_password(
    prompt: &str,
    help: Option<&str>,
) -> Result<String> {
    // Setup terminal
    enable_raw_mode().map_err(|e| DoumError::Config(format!("Failed to enable raw mode: {}", e)))?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)
        .map_err(|e| DoumError::Config(format!("Failed to enter alternate screen: {}", e)))?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)
        .map_err(|e| DoumError::Config(format!("Failed to create terminal: {}", e)))?;

    // Run the password app
    let result = run_password_app(&mut terminal, prompt, help);

    // Restore terminal
    disable_raw_mode().map_err(|e| DoumError::Config(format!("Failed to disable raw mode: {}", e)))?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .map_err(|e| DoumError::Config(format!("Failed to leave alternate screen: {}", e)))?;
    terminal.show_cursor()
        .map_err(|e| DoumError::Config(format!("Failed to show cursor: {}", e)))?;

    result
}

/// Run password app
fn run_password_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    prompt: &str,
    help: Option<&str>,
) -> Result<String> {
    let mut app = InputApp::new(prompt, None, help);

    loop {
        terminal.draw(|f| ui_password(f, &app))
            .map_err(|e| DoumError::Config(format!("Failed to draw: {}", e)))?;

        if let Event::Key(key) = event::read()
            .map_err(|e| DoumError::Config(format!("Failed to read event: {}", e)))? 
        {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Enter => return Ok(app.input.clone()),
                    KeyCode::Char(c) => app.enter_char(c),
                    KeyCode::Backspace => app.delete_char(),
                    KeyCode::Esc => return Err(DoumError::Config("Input cancelled".to_string())),
                    _ => {}
                }
            }
        }
    }
}

/// Draw password UI
fn ui_password(f: &mut Frame, app: &InputApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Prompt
            Constraint::Length(3),  // Input (masked)
            Constraint::Length(2),  // Help
        ])
        .split(f.area());

    // Prompt
    let prompt_text = vec![
        Line::from(Span::styled(&app.prompt, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
    ];
    let prompt_widget = Paragraph::new(prompt_text)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(prompt_widget, chunks[0]);

    // Input (masked)
    let masked = "*".repeat(app.input.len());
    let input_widget = Paragraph::new(masked.as_str())
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Password"));
    f.render_widget(input_widget, chunks[1]);

    // Help
    let help_text = if let Some(ref help) = app.help {
        help.clone()
    } else {
        String::from("Enter: Submit | Esc: Cancel")
    };
    
    let help = Paragraph::new(Line::from(Span::styled(
        help_text,
        Style::default().fg(Color::DarkGray)
    )));
    f.render_widget(help, chunks[2]);
}
