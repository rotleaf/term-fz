use api::{
    requests::api_requests::{details, download, search},
    structs::{DetailResponse, DownloadItems, DownloadResponse, SearchResponse},
};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    style::{self, Color, SetBackgroundColor, SetForegroundColor, Stylize},
    terminal::{self, ClearType},
};
use std::io::{stdout, Error, ErrorKind, Write};

// use colored::*;
pub mod api;
use dotenv::dotenv;

struct TextField {
    content: String,
    cursor_pos: usize,
}

impl TextField {
    fn new() -> Self {
        TextField {
            content: String::new(),
            cursor_pos: 0,
        }
    }

    fn insert_char(&mut self, c: char) {
        self.content.insert(self.cursor_pos, c);
        self.cursor_pos += 1;
    }

    fn delete_char(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.content.remove(self.cursor_pos);
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_pos < self.content.len() {
            self.cursor_pos += 1;
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    terminal::enable_raw_mode().map_err(|e| Error::new(ErrorKind::Other, e))?;
    let mut stdout: std::io::Stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;

    let mut text_field: TextField = TextField::new();
    let field_width: u16 = 50;
    let field_x: u16 = 2;
    let field_y: u16 = 2;

    let mut search_results: Vec<SearchResponse> = Vec::new();
    let mut selected_result: usize = 0;
    let mut show_results: bool = false;
    let mut is_loading: bool = false;
    let mut current_details: Option<DetailResponse> = None;
    let mut is_loading_details: bool = false;
    let mut is_downloading: bool = false;
    // let mut downloading_index: Option<usize> = None;
    let mut selected_download: usize = 0;
    let mut show_downloads: bool = false;
    let mut download_response: Option<DownloadResponse> = None;
    let mut show_file_items: bool = false;
    let mut selected_file_item: usize = 0;

    loop {
        execute!(
            stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        // Draw search label
        execute!(
            stdout,
            cursor::MoveTo(field_x, field_y - 1),
            style::Print("Search:")
        )?;

        // Draw text field box
        execute!(
            stdout,
            cursor::MoveTo(field_x, field_y),
            SetBackgroundColor(Color::White),
            SetForegroundColor(Color::Black),
        )?;

        let display_text: String = if text_field.content.is_empty() {
            " ".repeat(field_width as usize)
        } else {
            format!(
                "{:width$}",
                text_field.content,
                width = field_width as usize
            )
        };

        execute!(stdout, style::Print(&display_text))?;

        execute!(
            stdout,
            SetBackgroundColor(Color::Reset),
            SetForegroundColor(Color::Reset)
        )?;

        // Show loading indicator for search
        if is_loading {
            execute!(
                stdout,
                cursor::MoveTo(field_x, field_y + 2),
                style::Print("Loading results...")
            )?;
        }

        let mut current_y: u16 = field_y + 4;

        // Display results if available and neither downloads nor file items are being shown
        if show_results && !search_results.is_empty() && !show_downloads && !show_file_items {
            execute!(stdout, cursor::MoveTo(field_x, field_y + 2))?;
            execute!(
                stdout,
                style::Print(format!(
                    "Results {} (use arrows to select, Enter to view details):",
                    search_results.len()
                ))
            )?;

            for (i, result) in search_results.iter().enumerate() {
                execute!(stdout, cursor::MoveTo(field_x, current_y + i as u16))?;

                if i == selected_result {
                    execute!(
                        stdout,
                        style::Print(format!(
                            "{} {} ",
                            ">".to_string().bold().green().to_string(),
                            result.title.clone().bold().green()
                        ))
                    )?;
                } else {
                    execute!(stdout, style::Print(format!("  {} ", result.title)))?;
                }
            }
            current_y += search_results.len() as u16 + 1;
        }

        // Show loading indicator for details or display download items
        if is_loading_details {
            execute!(
                stdout,
                cursor::MoveTo(field_x, current_y - 2),
                style::Print("Loading details...")
            )?;
        } else if let Some(details) = &current_details {
            if show_downloads && !show_file_items && !details.download_items.is_empty() {
                execute!(
                    stdout,
                    cursor::MoveTo(field_x, field_y + 2),
                    style::Print(
                        "Download Items (use arrows to select, Enter to download, Esc to go back):"
                    )
                )?;

                for (index, item) in details.download_items.iter().enumerate() {
                    execute!(stdout, cursor::MoveTo(field_x, field_y + 4 + index as u16))?;
                    let this_dl = index == selected_download;

                    let prefix: String = if this_dl {
                        ">".to_string().bold().green().to_string()
                    } else {
                        " ".to_string()
                    };

                    execute!(
                        stdout,
                        style::Print(format!(
                            "{} {} ",
                            prefix,
                            if this_dl {
                                item.file_name.clone().bold().green().to_string()
                            } else {
                                item.file_name.clone()
                            },
                        ))
                    )?;
                }
            }
        }

        if show_file_items {
            if let Some(download_resp) = &download_response {
                execute!(
                    stdout,
                    cursor::MoveTo(field_x, field_y + 2),
                    style::Print("Available Files (use arrows to select, Esc to go back):")
                )?;

                for (index, file) in download_resp.files.iter().enumerate() {
                    execute!(stdout, cursor::MoveTo(field_x, field_y + 4 + index as u16))?;

                    let prefix: String = if index == selected_file_item {
                        ">".to_string().bold().green().to_string()
                    } else {
                        " ".to_string()
                    };

                    let file_name: String = if index == selected_file_item {
                        file.name.clone().bold().green().to_string()
                    } else {
                        file.name.to_string()
                    };

                    execute!(
                        stdout,
                        style::Print(format!(
                            "{} {} (Connections: {})",
                            prefix, file_name, file.connections
                        ))
                    )?;
                }
            }
        }

        if !show_results && !show_downloads && !show_file_items {
            let cursor_x: u16 = field_x + (text_field.cursor_pos as u16);
            execute!(stdout, cursor::MoveTo(cursor_x, field_y), cursor::Show)?;
        }

        stdout.flush()?;

        if let Event::Key(key_event) = event::read().map_err(|e| Error::new(ErrorKind::Other, e))? {
            match key_event.code {
                KeyCode::Char(c) => {
                    if !show_results
                        && !show_downloads
                        && !show_file_items
                        && text_field.content.len() < field_width as usize
                        && !key_event.modifiers.contains(KeyModifiers::CONTROL)
                    {
                        text_field.insert_char(c);
                    }
                }
                KeyCode::Backspace => {
                    if !show_results && !show_downloads && !show_file_items {
                        text_field.delete_char();
                    }
                }
                KeyCode::Left => {
                    if !show_results && !show_downloads && !show_file_items {
                        text_field.move_cursor_left();
                    }
                }
                KeyCode::Right => {
                    if !show_results && !show_downloads && !show_file_items {
                        text_field.move_cursor_right();
                    }
                }
                KeyCode::Enter => {
                    if !show_results && !show_downloads && !show_file_items {
                        // Perform search

                        execute!(
                            stdout,
                            cursor::MoveTo(field_x, field_y + 2),
                            style::Print("Searching...")
                        )?;
                        current_details = None;
                        search_results = search(&text_field.content).await?;
                        selected_result = 0;
                        show_results = true;
                        is_loading = false;
                    } else if show_results
                        && !show_downloads
                        && !show_file_items
                        && !search_results.is_empty()
                    {
                        execute!(
                            stdout,
                            cursor::MoveTo(field_x, current_y),
                            style::Print("Loading details...")
                        )?;

                        let selected: &SearchResponse = &search_results[selected_result];
                        let details: DetailResponse = details(&selected.path).await?;
                        current_details = Some(details);
                        is_loading_details = false;
                        show_downloads = true;
                        show_results = false;
                        selected_download = 0;
                    } else if show_downloads && !show_file_items && !is_downloading {
                        if let Some(details) = &current_details {
                            if selected_download < details.download_items.len() {
                                let download_item: &DownloadItems =
                                    &details.download_items[selected_download];

                                    execute!(
                                        stdout,
                                        cursor::MoveTo(field_x, current_y + 3),
                                        style::Print("Loading files...")
                                    )?;

                                // downloading_index = Some(selected_download);
                                let dl_response: DownloadResponse =
                                    download(&download_item.download_key).await?;
                                download_response = Some(dl_response);
                                is_downloading = false;
                                show_file_items = true;
                                show_downloads = false;
                                selected_file_item = 0;
                                if show_file_items {}
                            }
                        }
                    }
                }
                KeyCode::Up => {
                    if show_results && !show_downloads && !show_file_items && selected_result > 0 {
                        selected_result -= 1;
                    } else if show_downloads && !show_file_items && !is_downloading {
                        if selected_download > 0 {
                            selected_download -= 1;
                        }
                    } else if show_file_items {
                        if selected_file_item > 0 {
                            selected_file_item -= 1;
                        }
                    }
                }
                KeyCode::Down => {
                    if show_results
                        && !show_downloads
                        && !show_file_items
                        && selected_result < search_results.len() - 1
                    {
                        selected_result += 1;
                    } else if show_downloads && !show_file_items && !is_downloading {
                        if let Some(details) = &current_details {
                            if selected_download < details.download_items.len() - 1 {
                                selected_download += 1;
                            }
                        }
                    } else if show_file_items {
                        if let Some(download_resp) = &download_response {
                            if selected_file_item < download_resp.files.len() - 1 {
                                selected_file_item += 1;
                            }
                        }
                    }
                }
                KeyCode::Esc => {
                    if show_file_items {
                        show_file_items = false;
                        show_downloads = true;
                        download_response = None;
                    } else if show_downloads {
                        if !is_downloading {
                            show_downloads = false;
                            show_results = true;
                        }
                    } else if show_results {
                        show_results = false;
                        search_results.clear();
                        current_details = None;
                    } else {
                        execute!(stdout, terminal::LeaveAlternateScreen)?;
                        terminal::disable_raw_mode()
                            .map_err(|e| Error::new(ErrorKind::Other, e))?;
                        println!("Search canceled");
                        break;
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}
