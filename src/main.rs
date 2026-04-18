use crate::parser::IniFile;
use crate::state::{AppState, FocusArea};
use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::style::{Color, Style};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Row, Table},
    Frame,
};
use std::io::{self, Write};
use std::path::Path;

mod i18n;
mod parser;
mod state;

#[derive(Parser, Debug)]
#[command(name = "ini-editor")]
#[command(about = "Interactive INI/CONF file editor with TUI", long_about = None)]
struct Cli {
    #[arg(help = "Path to the INI/CONF file to edit")]
    file: Option<String>,

    #[arg(
        short = 'l',
        long = "language",
        value_name = "LANG",
        help = "Set the UI language (use -l without value to see available languages)"
    )]
    language: Option<String>,
}

fn get_save_shortcut() -> (&'static str, &'static str) {
    ("s", "s")
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(lang) = cli.language {
        if i18n::is_valid_language(&lang) {
            i18n::set_language(&lang);
        } else {
            eprintln!("Warning: Unknown language '{}'. Using default.", lang);
            eprintln!(
                "Available languages: {}",
                i18n::AVAILABLE_LANGUAGES
                    .iter()
                    .map(|l| l.code)
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }

    let file_path = match cli.file {
        Some(path) => path,
        None => {
            eprintln!("Usage: ini-editor [OPTIONS] <file.ini>");
            eprintln!("  -l, --language LANG  Set UI language");
            std::process::exit(1);
        }
    };

    let path = Path::new(&file_path);

    let ini_file = if path.exists() {
        IniFile::load(path)?
    } else {
        IniFile::new()
    };

    let mut app = AppState::new(ini_file, file_path);

    run_app(&mut app)
}

fn run_app(app: &mut AppState) -> Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let result = run_app_loop(app);

    terminal::disable_raw_mode()?;
    stdout.execute(LeaveAlternateScreen)?;

    print!("\x1B[2J\x1B[?25h");
    io::stdout().flush()?;

    result
}

fn run_app_loop(app: &mut AppState) -> Result<()> {
    let mut terminal =
        ratatui::Terminal::new(ratatui::backend::CrosstermBackend::new(io::stdout()))?;

    loop {
        terminal.draw(|f| ui(f, app))?;

        let event = event::read()?;

        if let Event::Resize(_, _) = event {
            continue;
        }

        if app.show_help {
            if let Event::Key(key) = event {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('?') | KeyCode::Char('q') | KeyCode::Esc => {
                            app.show_help = false;
                        }
                        _ => {}
                    }
                }
            }
            continue;
        }

        if app.show_language_menu {
            if let Event::Key(key) = event {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Down => {
                            app.language_menu_next();
                        }
                        KeyCode::Up => {
                            app.language_menu_prev();
                        }
                        KeyCode::Enter | KeyCode::Char(' ') => {
                            app.language_menu_select();
                        }
                        KeyCode::Esc => {
                            app.show_language_menu = false;
                        }
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        _ => {}
                    }
                }
            }
            continue;
        }

        if let Event::Key(key) = event {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            if key.code == KeyCode::Char('L') {
                app.toggle_language_menu();
                continue;
            }

            match app.focus {
                FocusArea::Sections => {
                    if handle_section_key(app, &key)? {
                        return Ok(());
                    }
                }
                FocusArea::Variables => {
                    if handle_variable_key(app, &key)? {
                        return Ok(());
                    }
                }
                FocusArea::Search => {
                    if handle_search_key(app, &key)? {
                        return Ok(());
                    }
                }
            }
        }
    }
}

fn handle_section_key(app: &mut AppState, key: &event::KeyEvent) -> Result<bool> {
    // Handle section renaming
    if app.editing_section {
        match key.code {
            KeyCode::Char(c) => {
                if c == 'q' {
                    return Ok(true);
                }
                app.input_buffer.push(c);
                Ok(false)
            }
            KeyCode::Backspace => {
                app.input_buffer.pop();
                Ok(false)
            }
            KeyCode::Enter => {
                apply_section_rename(app);
                Ok(false)
            }
            KeyCode::Esc => {
                app.editing_section = false;
                app.input_buffer.clear();
                Ok(false)
            }
            KeyCode::Tab | KeyCode::Right => {
                apply_section_rename(app);
                app.focus = FocusArea::Variables;
                Ok(false)
            }
            _ => Ok(false),
        }
    } else {
        match key.code {
            KeyCode::Tab | KeyCode::Right => {
                app.focus = FocusArea::Variables;
                Ok(false)
            }
            KeyCode::Char('q') => Ok(true),
            KeyCode::Char('s') => {
                save_and_exit(app);
                Ok(false)
            }
            KeyCode::Char('/') => {
                app.search_mode = true;
                app.search_query.clear();
                app.focus = FocusArea::Search;
                Ok(false)
            }
            KeyCode::Char('?') => {
                app.show_help = true;
                Ok(false)
            }
            KeyCode::Up => {
                app.section_list_state.select_previous();
                app.variable_list_state.select_first();
                Ok(false)
            }
            KeyCode::Down => {
                app.section_list_state.select_next();
                app.variable_list_state.select_first();
                Ok(false)
            }
            KeyCode::Char('a') => {
                start_add_section(app);
                Ok(false)
            }
            KeyCode::Char('d') => {
                if let Some(section) = app.selected_section() {
                    app.confirm_delete_section(section);
                }
                Ok(false)
            }
            KeyCode::Char('y') => {
                if app.confirm_delete.is_some() {
                    app.execute_delete_confirmed();
                }
                Ok(false)
            }
            KeyCode::Char('n') => {
                app.cancel_delete();
                Ok(false)
            }
            KeyCode::Char('e') | KeyCode::Char('E') => {
                start_rename_section(app);
                Ok(false)
            }
            _ => Ok(false),
        }
    }
}

fn handle_variable_key(app: &mut AppState, key: &event::KeyEvent) -> Result<bool> {
    // Handle dropdown menu
    if app.show_dropdown {
        match key.code {
            KeyCode::Down => {
                app.dropdown_select_next();
                Ok(false)
            }
            KeyCode::Up => {
                app.dropdown_select_prev();
                Ok(false)
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                app.dropdown_select();
                Ok(false)
            }
            KeyCode::Esc => {
                app.show_dropdown = false;
                Ok(false)
            }
            _ => Ok(false),
        }
    } else if let Some(ref editing_key) = app.editing_key {
        match key.code {
            KeyCode::Char(c) => {
                if c == 'q' {
                    return Ok(true);
                }
                app.input_buffer.push(c);
                Ok(false)
            }
            KeyCode::Backspace => {
                app.input_buffer.pop();
                Ok(false)
            }
            KeyCode::Enter => {
                if app.editing_key_name {
                    apply_key_rename(app, editing_key.clone());
                } else {
                    apply_value_edit(app, editing_key.clone());
                }
                Ok(false)
            }
            KeyCode::Esc => {
                app.editing_key = None;
                app.editing_key_name = false;
                app.input_buffer.clear();
                Ok(false)
            }
            KeyCode::Tab => {
                if app.editing_key_name {
                    apply_key_rename(app, editing_key.clone());
                } else {
                    apply_value_edit(app, editing_key.clone());
                }
                app.focus = FocusArea::Sections;
                Ok(false)
            }
            _ => Ok(false),
        }
    } else {
        match key.code {
            KeyCode::Char('e') | KeyCode::Char('E') => {
                let current_value = app.selected_value().unwrap_or_default();
                let lower = current_value.to_lowercase();
                if lower == "true"
                    || lower == "false"
                    || lower == "yes"
                    || lower == "no"
                    || lower == "on"
                    || lower == "off"
                {
                    app.toggle_dropdown();
                } else {
                    if let Some(key) = app.selected_key() {
                        app.editing_key = Some(key);
                        app.input_buffer = current_value;
                    }
                }
                Ok(false)
            }
            KeyCode::Tab => {
                app.focus = FocusArea::Sections;
                app.section_list_state.select_first();
                Ok(false)
            }
            KeyCode::BackTab => {
                app.focus = FocusArea::Sections;
                app.section_list_state.select_first();
                Ok(false)
            }
            KeyCode::Left => {
                app.focus = FocusArea::Sections;
                Ok(false)
            }
            KeyCode::Char('q') => Ok(true),
            KeyCode::Char('s') => {
                save_and_exit(app);
                Ok(false)
            }
            KeyCode::Char('/') => {
                app.search_mode = true;
                app.search_query.clear();
                app.focus = FocusArea::Search;
                Ok(false)
            }
            KeyCode::Char('?') => {
                app.show_help = true;
                Ok(false)
            }
            KeyCode::Up => {
                app.variable_list_state.select_previous();
                Ok(false)
            }
            KeyCode::Down => {
                app.variable_list_state.select_next();
                Ok(false)
            }
            KeyCode::Char('a') => {
                if app.selected_section().is_some() {
                    start_add_key(app);
                }
                Ok(false)
            }
            KeyCode::Char('d') => {
                delete_current_variable(app);
                Ok(false)
            }
            KeyCode::Char(' ') => {
                toggle_comment_variable(app);
                Ok(false)
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                start_rename_key(app);
                Ok(false)
            }
            _ => Ok(false),
        }
    }
}

fn handle_search_key(app: &mut AppState, key: &event::KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Char(c) => {
            if c == 'q' {
                return Ok(true);
            }
            if c == 'n' {
                app.next_search_result();
                return Ok(false);
            }
            if c == 'p' {
                app.prev_search_result();
                return Ok(false);
            }
            app.search_query.push(c);
            app.perform_search();
            Ok(false)
        }
        KeyCode::Backspace => {
            app.search_query.pop();
            app.perform_search();
            Ok(false)
        }
        KeyCode::Enter => {
            app.next_search_result();
            Ok(false)
        }
        KeyCode::Esc => {
            app.search_mode = false;
            app.search_query.clear();
            app.search_results.clear();
            app.focus = FocusArea::Sections;
            Ok(false)
        }
        _ => Ok(false),
    }
}

fn start_add_section(app: &mut AppState) {
    app.input_buffer.clear();
    let section_name = format!("new_section_{}", app.ini_file.sections.len() + 1);
    app.ini_file
        .sections
        .entry(section_name.clone())
        .or_default();
    app.ini_file
        .raw_lines
        .push(parser::Line::Section(section_name.clone()));
    app.ini_file.modified = true;

    app.input_buffer = section_name;
    app.editing_section = true;
}

fn start_rename_section(app: &mut AppState) {
    if let Some(section) = app.selected_section() {
        // Don't allow renaming DEFAULT section
        if parser::IniFile::is_default_section(&section) {
            return;
        }
        app.input_buffer = section;
        app.editing_section = true;
    }
}

fn apply_section_rename(app: &mut AppState) {
    let old_section = app.selected_section();
    let new_section = app.input_buffer.trim().to_string();

    if let Some(old) = old_section {
        if !new_section.is_empty() && new_section != old {
            // Move keys to new section
            if let Some(old_keys) = app.ini_file.sections.remove(&old) {
                app.ini_file.sections.insert(new_section.clone(), old_keys);
                app.ini_file.modified = true;

                // Update raw lines
                for line in &mut app.ini_file.raw_lines {
                    if let parser::Line::Section(s) = line {
                        if s == &old {
                            *s = new_section.clone();
                            break;
                        }
                    }
                }
            }
        }
    }

    app.editing_section = false;
    app.input_buffer.clear();

    // Re-select the renamed section
    let sections: Vec<_> = app.ini_file.sections.keys().cloned().collect();
    if let Some(new_idx) = sections.iter().position(|s| *s == new_section) {
        app.section_list_state.select(Some(new_idx));
    }
}

fn start_add_key(app: &mut AppState) {
    if let Some(section) = app.selected_section() {
        let key_name = format!(
            "new_key_{}",
            app.ini_file
                .sections
                .get(&section)
                .map(|s| s.len())
                .unwrap_or(0)
                + 1
        );
        app.ini_file
            .sections
            .entry(section.clone())
            .or_default()
            .insert(key_name.clone(), String::new());
        app.ini_file.raw_lines.push(parser::Line::KeyValue {
            key: key_name.clone(),
            value: String::new(),
            commented: false,
        });
        app.ini_file.modified = true;

        app.input_buffer = key_name;
        app.editing_key = Some(app.input_buffer.clone());
        app.editing_key_name = true;
    }
}

fn apply_value_edit(app: &mut AppState, key: String) {
    if let Some(section) = app.selected_section() {
        if let Some(vars) = app.ini_file.sections.get_mut(&section) {
            vars.insert(key.clone(), app.input_buffer.clone());
            app.ini_file.modified = true;

            for line in &mut app.ini_file.raw_lines {
                if let parser::Line::KeyValue {
                    key: k, value: v, ..
                } = line
                {
                    if k == &key {
                        *v = app.input_buffer.clone();
                        break;
                    }
                }
            }
        }
    }
    app.editing_key = None;
    app.editing_key_name = false;
    app.input_buffer.clear();
}

fn start_rename_key(app: &mut AppState) {
    if let Some(key) = app.selected_key() {
        app.input_buffer = key.clone();
        app.editing_key = Some(key);
        app.editing_key_name = true;
    }
}

fn apply_key_rename(app: &mut AppState, old_key: String) {
    let new_key = app.input_buffer.trim().to_string();
    if let Some(section) = app.selected_section() {
        if let Some(vars) = app.ini_file.sections.get_mut(&section) {
            if let Some(value) = vars.remove(&old_key) {
                if !new_key.is_empty() && new_key != old_key {
                    vars.insert(new_key.clone(), value);
                    app.ini_file.modified = true;

                    for line in &mut app.ini_file.raw_lines {
                        if let parser::Line::KeyValue { key: k, .. } = line {
                            if k == &old_key {
                                *k = new_key.clone();
                                break;
                            }
                        }
                    }
                } else {
                    vars.insert(old_key, value);
                }
            }
        }
    }
    app.editing_key = None;
    app.editing_key_name = false;
    app.input_buffer.clear();
}

fn delete_current_variable(app: &mut AppState) {
    if let Some(_section) = app.selected_section() {
        if let Some(key) = app.selected_key() {
            if let Some(vars) = app.ini_file.sections.get_mut(&_section) {
                vars.remove(&key);
                app.ini_file.modified = true;

                app.ini_file.raw_lines.retain(|line| match line {
                    parser::Line::KeyValue { key: k, .. } => k != &key,
                    _ => true,
                });
            }
        }
    }
}

fn toggle_comment_variable(app: &mut AppState) {
    if let Some(_section) = app.selected_section() {
        if let Some(key) = app.selected_key() {
            for line in &mut app.ini_file.raw_lines {
                if let parser::Line::KeyValue {
                    key: k,
                    ref mut commented,
                    ..
                } = line
                {
                    if k == &key {
                        *commented = !*commented;
                        app.ini_file.modified = true;
                        break;
                    }
                }
            }
        }
    }
}

fn save_and_exit(app: &mut AppState) {
    match app.ini_file.save(Path::new(&app.file_path)) {
        Ok(_) => {
            app.modified = false;
            app.set_status_success(i18n::t("file_saved").to_string());
        }
        Err(e) => {
            app.set_status_error(format!("{}: {}", i18n::t("file_save_error"), e));
        }
    }
}

fn ui(frame: &mut Frame, app: &mut AppState) {
    let title_style = Style::default().fg(Color::Cyan).bold();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title = if app.search_mode {
        format!(
            " SEARCH: {} ({}/{}) ",
            app.search_query,
            app.search_results.len().max(1),
            app.search_results.len()
        )
    } else {
        format!(" ini-editor - {} ", app.file_path)
    };

    let title_block = Block::default()
        .title(title)
        .title_style(title_style)
        .title_alignment(ratatui::layout::Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(if app.search_mode {
            Color::Yellow
        } else {
            Color::Cyan
        }));
    frame.render_widget(title_block, chunks[0]);

    if app.show_help {
        show_help_popup(frame);
        return;
    }

    if app.show_language_menu {
        show_language_menu_popup(frame, app);
        return;
    }

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(chunks[1]);

    render_sections_panel(frame, app, main_chunks[0]);
    render_variables_panel(frame, app, main_chunks[1]);
    render_help_bar(frame, app, chunks[2]);
}

fn show_help_popup(frame: &mut Frame) {
    let area = frame.area();
    let popup_block = Block::default()
        .title(format!(" {} ", i18n::t("help_title")))
        .title_style(Style::default().fg(Color::Yellow).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::LightYellow))
        .style(Style::default().bg(Color::DarkGray).fg(Color::White));

    let inner_rect = popup_block.inner(area);
    frame.render_widget(popup_block, area);

    let (save_shortcut_mac, save_label) = get_save_shortcut();

    let navigation = i18n::t("navigation");
    let section_next = i18n::t("section_next");
    let section_prev = i18n::t("section_prev");
    let move_list = i18n::t("move_list");
    let search = i18n::t("search");
    let activate_search = i18n::t("activate_search");
    let next_prev_result = i18n::t("next_prev_result");
    let go_result = i18n::t("go_result");
    let quit_search = i18n::t("quit_search");
    let inline_edition = i18n::t("inline_edition");
    let edit_value = i18n::t("edit_value");
    let rename_key = i18n::t("rename_key");
    let cancel_edition = i18n::t("cancel_edition");
    let actions = i18n::t("actions");
    let add_section_var = i18n::t("add_section_var");
    let delete = i18n::t("delete");
    let comment_uncomment = i18n::t("comment_uncomment");
    let commands = i18n::t("commands");
    let quit_no_save = i18n::t("quit_no_save");
    let save_quit = i18n::t("save_quit");
    let help_quit = i18n::t("help_quit");

    let help_lines = vec![
        vec![Span::raw(format!("{}:", navigation))],
        vec![Span::raw("")],
        vec![
            Span::raw("  "),
            Span::styled("Tab", Style::default().fg(Color::Cyan)),
            Span::raw(" → "),
            Span::styled("→", Style::default().fg(Color::Cyan)),
            Span::raw(format!(" - {}", section_next)),
        ],
        vec![
            Span::raw("  "),
            Span::styled("←", Style::default().fg(Color::Cyan)),
            Span::raw(" ← "),
            Span::raw(format!(" - {}", section_prev)),
        ],
        vec![
            Span::raw("  "),
            Span::styled("↑↓", Style::default().fg(Color::Cyan)),
            Span::raw(format!(" - {}", move_list)),
        ],
        vec![Span::raw("")],
        vec![Span::raw(format!("{}:", search))],
        vec![
            Span::raw("  "),
            Span::styled("/", Style::default().fg(Color::Yellow)),
            Span::raw(format!(" - {}", activate_search)),
        ],
        vec![
            Span::raw("  "),
            Span::styled("n/p", Style::default().fg(Color::Yellow)),
            Span::raw(format!(" - {}", next_prev_result)),
        ],
        vec![
            Span::raw("  "),
            Span::styled("Enter", Style::default().fg(Color::Blue)),
            Span::raw(format!(" - {}", go_result)),
        ],
        vec![
            Span::raw("  "),
            Span::styled("Esc", Style::default().fg(Color::Blue)),
            Span::raw(format!(" - {}", quit_search)),
        ],
        vec![Span::raw("")],
        vec![Span::raw(format!("{}:", inline_edition))],
        vec![
            Span::raw("  "),
            Span::styled("e", Style::default().fg(Color::Yellow)),
            Span::raw(format!(" - {}", edit_value)),
        ],
        vec![
            Span::raw("  "),
            Span::styled("r", Style::default().fg(Color::Blue)),
            Span::raw(format!(" - {}", rename_key)),
        ],
        vec![
            Span::raw("  "),
            Span::styled("Esc", Style::default().fg(Color::Blue)),
            Span::raw(format!(" - {}", cancel_edition)),
        ],
        vec![Span::raw("")],
        vec![Span::raw(format!("{}:", actions))],
        vec![
            Span::raw("  "),
            Span::styled("a", Style::default().fg(Color::Green)),
            Span::raw(format!(" - {}", add_section_var)),
        ],
        vec![
            Span::raw("  "),
            Span::styled("d", Style::default().fg(Color::Red)),
            Span::raw(format!(" - {}", delete)),
        ],
        vec![
            Span::raw("  "),
            Span::styled("Space", Style::default().fg(Color::Magenta)),
            Span::raw(format!(" - {}", comment_uncomment)),
        ],
        vec![Span::raw("")],
        vec![Span::raw(format!("{}:", commands))],
        vec![
            Span::raw("  "),
            Span::styled("q", Style::default().fg(Color::Red).bold()),
            Span::raw(format!(" - {}", quit_no_save)),
        ],
        vec![
            Span::raw("  "),
            Span::styled(save_shortcut_mac, Style::default().fg(Color::Green)),
            Span::raw(format!(" - {}", save_quit)),
        ],
        vec![
            Span::raw("  "),
            Span::styled("?", Style::default().fg(Color::Yellow)),
            Span::raw(format!(" - {}", help_quit)),
        ],
    ];

    let lines: Vec<Line> = help_lines.into_iter().map(Line::from).collect();

    let paragraph = Paragraph::new(lines)
        .style(Style::default().fg(Color::White))
        .block(Block::default());
    frame.render_widget(paragraph, inner_rect);
}

fn show_language_menu_popup(frame: &mut Frame, app: &mut AppState) {
    use crate::i18n::AVAILABLE_LANGUAGES;

    let area = frame.area();
    let popup_height = (AVAILABLE_LANGUAGES.len() as u16) + 4;
    let popup_width = 40u16;
    let popup_x = (area.width - popup_width) / 2;
    let popup_y = (area.height - popup_height) / 2;
    let popup_area = Rect::new(popup_x, popup_y, popup_width, popup_height);

    let popup_block = Block::default()
        .title(format!(" {} ", i18n::t("select_language")))
        .title_style(Style::default().fg(Color::Green).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green))
        .style(Style::default().bg(Color::Black).fg(Color::White));

    frame.render_widget(popup_block, popup_area);

    let inner_area = Rect::new(popup_x + 1, popup_y + 1, popup_width - 2, popup_height - 2);

    let items: Vec<ListItem> = AVAILABLE_LANGUAGES
        .iter()
        .enumerate()
        .map(|(idx, lang)| {
            let is_selected = idx == app.language_menu_index;
            let is_current = lang.code == i18n::get_current_language();
            let marker = if is_current { " (*)" } else { "    " };
            let display = format!("{}{}", lang.native_name, marker);

            if is_selected {
                ListItem::new(format!("▶ {}", display))
                    .style(Style::default().fg(Color::Black).bg(Color::Green).bold())
            } else {
                ListItem::new(format!("  {}", display)).style(Style::default().fg(Color::White))
            }
        })
        .collect();

    let list = List::new(items)
        .block(Block::default())
        .style(Style::default().bg(Color::Black));

    frame.render_widget(list, inner_area);
}

fn render_sections_panel(frame: &mut Frame, app: &mut AppState, area: Rect) {
    let focus_section_style = Style::default().fg(Color::Green).bold().underlined();
    let normal_section_style = Style::default().fg(Color::Green);

    let sections: Vec<ListItem> = app
        .ini_file
        .sections
        .keys()
        .enumerate()
        .map(|(idx, s)| {
            let is_selected = app.section_list_state.selected() == Some(idx);
            let is_editing = is_selected && app.editing_section;

            if is_editing || is_selected {
                let prefix = if is_editing { "▶ [Editing] " } else { "▶ " };
                let name = if is_editing { &app.input_buffer } else { s };
                ListItem::new(format!("{}{}", prefix, name)).style(focus_section_style)
            } else {
                ListItem::new(format!("  {}", s)).style(normal_section_style)
            }
        })
        .collect();

    let list = List::new(sections).block(
        Block::default()
            .title(format!(" {} ", i18n::t("sections")))
            .title_style(Style::default().fg(Color::Green).bold())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green)),
    );

    frame.render_stateful_widget(list, area, &mut app.section_list_state);
}

fn render_variables_panel(frame: &mut Frame, app: &mut AppState, area: Rect) {
    let focus_key_fg = Style::default().fg(Color::Black).bold();
    let focus_key_bg = Color::Cyan;

    let editing_fg = Style::default().fg(Color::White).bold();
    let editing_bg = Color::Magenta;

    let commented_fg = Style::default().fg(Color::DarkGray);

    let normal_key = Style::default().fg(Color::Cyan);
    let normal_value = Style::default().fg(Color::Gray);

    let vars = app.current_variables();
    let variables: Vec<Row> = vars
        .iter()
        .enumerate()
        .map(|(idx, (k, v, commented))| {
            let is_selected = app.variable_list_state.selected() == Some(idx);
            let is_editing = is_selected && app.editing_key.as_ref() == Some(k);

            if is_editing {
                let input_text = if app.input_buffer.is_empty() {
                    "█".to_string()
                } else {
                    format!("{}█", app.input_buffer)
                };
                if app.editing_key_name {
                    Row::new(vec![
                        Span::styled(format!("▶ {}", input_text), editing_fg).bg(editing_bg),
                        Span::styled("= ", Style::default().fg(Color::DarkGray).bg(editing_bg)),
                        Span::styled(v, Style::default().fg(Color::Gray).bg(editing_bg)),
                    ])
                } else {
                    Row::new(vec![
                        Span::styled(format!("▶ {}", k), editing_fg).bg(editing_bg),
                        Span::styled("= ", Style::default().fg(Color::DarkGray).bg(editing_bg)),
                        Span::styled(input_text, editing_fg).bg(editing_bg),
                    ])
                }
            } else if is_selected {
                Row::new(vec![
                    Span::styled(format!("▶ {}", k), focus_key_fg).bg(focus_key_bg),
                    Span::styled("= ", Style::default().fg(Color::DarkGray).bg(focus_key_bg)),
                    Span::styled(
                        v.clone(),
                        Style::default().fg(Color::Yellow).bg(focus_key_bg),
                    ),
                ])
            } else if *commented {
                Row::new(vec![Span::styled(format!("# {} = {}", k, v), commented_fg)])
            } else {
                Row::new(vec![
                    Span::styled(format!("  {}", k), normal_key),
                    Span::styled("= ", Style::default().fg(Color::DarkGray)),
                    Span::styled(v.clone(), normal_value),
                ])
            }
        })
        .collect();

    let widths = [
        Constraint::Percentage(30),
        Constraint::Percentage(5),
        Constraint::Percentage(65),
    ];

    let table = Table::new(variables, widths)
        .block(
            Block::default()
                .title(i18n::t("variables"))
                .title_style(Style::default().fg(Color::Yellow).bold())
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .row_highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black).bold());

    frame.render_stateful_widget(table, area, &mut app.variable_list_state);

    if app.show_dropdown && !app.dropdown_options.is_empty() {
        let dropdown_list_height = app.dropdown_options.len() as u16;

        if let Some(selected_idx) = app.variable_list_state.selected() {
            let row_height = 1u16;
            let row_y = area.y + 1 + selected_idx as u16;
            let value_x = area.x + (area.width as u32 * 35 / 100) as u16;
            let value_width = (area.width as u32 * 65 / 100) as u16;
            let clear_value_rect = Rect::new(value_x, row_y, value_width, row_height);
            frame.render_widget(
                Paragraph::new("").style(Style::default().bg(Color::Cyan)),
                clear_value_rect,
            );
        }

        let dropdown_items: Vec<ListItem> = app
            .dropdown_options
            .iter()
            .enumerate()
            .map(|(idx, opt)| {
                if idx == app.dropdown_index {
                    ListItem::new(format!("▶ {}", opt))
                        .style(Style::default().fg(Color::Black).bg(Color::Cyan).bold())
                } else {
                    ListItem::new(format!("  {}", opt)).style(Style::default().fg(Color::White))
                }
            })
            .collect();

        let dropdown_list = List::new(dropdown_items).block(
            Block::default()
                .title(format!(" {} ", i18n::t("choose")))
                .title_style(Style::default().fg(Color::Cyan).bold())
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .style(Style::default().bg(Color::Black)),
        );

        let dropdown_width = 20u16;
        let dropdown_area = Rect::new(
            area.x + 36,
            area.y + 1,
            dropdown_width,
            dropdown_list_height + 2,
        );

        let clear_rect = Rect::new(
            area.x + 36,
            area.y + 1,
            area.width - 36,
            dropdown_list_height + 2,
        );
        frame.render_widget(
            Paragraph::new("").style(Style::default().bg(Color::Black)),
            clear_rect,
        );
        frame.render_widget(dropdown_list, dropdown_area);
    }
}

fn render_help_bar(frame: &mut Frame, app: &mut AppState, area: Rect) {
    let (save_shortcut, _) = get_save_shortcut();

    let is_editing_key = app.editing_key.is_some();
    let is_editing_section = app.editing_section;

    let confirm_delete_section = i18n::t("confirm_delete_section");
    let yes = i18n::t("yes");
    let no = i18n::t("no");
    let navigate = i18n::t("navigate");
    let select = i18n::t("select");
    let validate = i18n::t("validate");
    let close = i18n::t("close");
    let new_name = i18n::t("new_name");
    let cancel = i18n::t("cancel");
    let search_active = i18n::t("search_active");
    let next_result = i18n::t("next_result");
    let prev_result = i18n::t("prev_result");
    let quit_search = i18n::t("quit_search");
    let to_variables = i18n::t("to_variables");
    let quit_no_save = i18n::t("quit_no_save");
    let add = i18n::t("add");
    let rename = i18n::t("rename");
    let delete = i18n::t("delete");
    let search_label = i18n::t("search_label");
    let language = i18n::t("language");
    let help_quit = i18n::t("help_quit");
    let save_quit = i18n::t("save_quit");
    let new_value = i18n::t("new_value");
    let to_sections = i18n::t("to_sections");
    let edit = i18n::t("edit");
    let comment_uncomment = i18n::t("comment_uncomment");

    let text = if let Some((ref msg, success)) = app.status_message {
        let color = if success { Color::Green } else { Color::Red };
        Line::from(vec![Span::styled(
            msg.clone(),
            Style::default().fg(color).bold(),
        )])
    } else if let Some(item) = &app.confirm_delete {
        if item.starts_with("section:") {
            let section = item.trim_start_matches("section:");
            Line::from(vec![
                Span::styled(
                    format!("{}: ", confirm_delete_section),
                    Style::default().fg(Color::Red).bold(),
                ),
                Span::styled(section, Style::default().fg(Color::Yellow).bold()),
                Span::raw(" | "),
                Span::styled("y", Style::default().fg(Color::Green)),
                Span::raw(format!(" {} | ", yes)),
                Span::styled("n", Style::default().fg(Color::Red)),
                Span::raw(format!(" {}", no)),
            ])
        } else {
            Line::from(vec![])
        }
    } else if app.show_dropdown {
        Line::from(vec![
            Span::styled("↑↓", Style::default().fg(Color::Cyan)),
            Span::raw(format!(" {} | ", navigate)),
            Span::styled("Space", Style::default().fg(Color::Green)),
            Span::raw(format!(" {} | ", select)),
            Span::styled("Enter", Style::default().fg(Color::Green)),
            Span::raw(format!(" {} | ", validate)),
            Span::styled("Esc", Style::default().fg(Color::Red)),
            Span::raw(format!(" {}", close)),
        ])
    } else if is_editing_section {
        Line::from(vec![
            Span::styled(
                format!("{}: ", new_name),
                Style::default().fg(Color::Yellow),
            ),
            Span::styled(&app.input_buffer, Style::default().fg(Color::White).bold()),
            Span::raw(" | "),
            Span::styled("Enter", Style::default().fg(Color::Green)),
            Span::raw(format!(" {} | ", validate)),
            Span::styled("Esc", Style::default().fg(Color::Red)),
            Span::raw(format!(" {}", cancel)),
        ])
    } else if app.search_mode {
        Line::from(vec![
            Span::styled(
                format!("{}: ", search_active),
                Style::default().fg(Color::Yellow),
            ),
            Span::styled(&app.search_query, Style::default().fg(Color::White).bold()),
            Span::raw(" | "),
            Span::styled("Enter", Style::default().fg(Color::Green)),
            Span::raw(format!("/n {} | ", next_result)),
            Span::styled("p", Style::default().fg(Color::Cyan)),
            Span::raw(format!(" {} | ", prev_result)),
            Span::styled("Esc", Style::default().fg(Color::Red)),
            Span::raw(format!(" {}", quit_search)),
        ])
    } else if app.focus == FocusArea::Sections {
        Line::from(vec![
            Span::styled("Tab", Style::default().fg(Color::Cyan)),
            Span::styled(" →", Style::default().fg(Color::Cyan)),
            Span::raw(format!(" {} | ", to_variables)),
            Span::styled("q", Style::default().fg(Color::Red).bold()),
            Span::raw(format!(" {} | ", quit_no_save)),
            Span::styled("a", Style::default().fg(Color::Green)),
            Span::raw(format!(" {} | ", add)),
            Span::styled("e", Style::default().fg(Color::Yellow)),
            Span::raw(format!(" {} | ", rename)),
            Span::styled("d", Style::default().fg(Color::Red)),
            Span::raw(format!(" {} | ", delete)),
            Span::styled("/", Style::default().fg(Color::Yellow)),
            Span::raw(format!(" {} | ", search_label)),
            Span::styled("L", Style::default().fg(Color::Blue)),
            Span::raw(format!(" {} | ", language)),
            Span::styled("?", Style::default().fg(Color::Magenta)),
            Span::raw(format!(" {} | ", help_quit)),
            Span::styled(save_shortcut, Style::default().fg(Color::Green)),
            Span::raw(format!(" {}", save_quit)),
        ])
    } else if is_editing_key {
        let help_text = if app.editing_key_name {
            vec![
                Span::styled(
                    format!("{}: ", new_name),
                    Style::default().fg(Color::Yellow),
                ),
                Span::styled(&app.input_buffer, Style::default().fg(Color::White).bold()),
                Span::raw(" | "),
                Span::styled("Enter", Style::default().fg(Color::Green)),
                Span::raw(format!(" {} | ", rename)),
                Span::styled("Esc", Style::default().fg(Color::Yellow)),
                Span::raw(format!(" {}", cancel)),
            ]
        } else {
            vec![
                Span::styled(
                    format!("{}: ", new_value),
                    Style::default().fg(Color::Yellow),
                ),
                Span::styled(&app.input_buffer, Style::default().fg(Color::White).bold()),
                Span::raw(" | "),
                Span::styled("Enter", Style::default().fg(Color::Green)),
                Span::raw(format!(" {} | ", validate)),
                Span::styled("Esc", Style::default().fg(Color::Yellow)),
                Span::raw(format!(" {}", cancel)),
            ]
        };
        Line::from(help_text)
    } else {
        Line::from(vec![
            Span::styled("Tab", Style::default().fg(Color::Cyan)),
            Span::styled(" →", Style::default().fg(Color::Cyan)),
            Span::raw(format!(" {} | ", to_sections)),
            Span::styled("a", Style::default().fg(Color::Green)),
            Span::raw(format!(" {} | ", add)),
            Span::styled("e", Style::default().fg(Color::Yellow)),
            Span::raw(format!(" {} | ", edit)),
            Span::styled("d", Style::default().fg(Color::Red)),
            Span::raw(format!(" {} | ", delete)),
            Span::styled("Space", Style::default().fg(Color::Magenta)),
            Span::raw(format!(" {} | ", comment_uncomment)),
            Span::styled("/", Style::default().fg(Color::Yellow)),
            Span::raw(format!(" {} | ", search_label)),
            Span::styled("L", Style::default().fg(Color::Blue)),
            Span::raw(format!(" {} | ", language)),
            Span::styled("?", Style::default().fg(Color::Magenta)),
            Span::raw(format!(" {}", help_quit)),
        ])
    };

    let (text, _text_style) = if let Some((ref msg, success)) = app.status_message {
        let style = if success {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Red)
        };
        (
            Line::from(vec![Span::styled(msg.clone(), style)]),
            Some(style),
        )
    } else {
        (text, None)
    };

    let bar = Paragraph::new(text)
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );
    frame.render_widget(bar, area);
}
