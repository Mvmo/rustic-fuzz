use std::{io::{stdin, stdout}, thread, sync::mpsc::{self, Receiver}, time::Duration};

use ratatui::{Terminal, prelude::CrosstermBackend, widgets::{List, ListItem}};
use crossterm::{execute, terminal::{EnterAlternateScreen, enable_raw_mode, disable_raw_mode, LeaveAlternateScreen}, event::{EnableMouseCapture, DisableMouseCapture, self, KeyEvent, KeyCode}, cursor::{SetCursorShape, CursorShape}};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn spawn_key_listener() -> Result<Receiver<KeyEvent>> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        loop {
            if event::poll(Duration::from_millis(50)).unwrap() {
                if let event::Event::Key(key_event) = event::read().unwrap() {
                    tx.send(key_event).unwrap();
                }
            }
        }
    });

    Ok(rx)
}


fn main() -> Result<()> {
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture, SetCursorShape(CursorShape::Block))?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    enable_raw_mode()?;

    //let items: Vec<String> = stdin().lines()
    //    .filter_map(|line| line.ok())
    //    .collect();
    let items = vec![String::from("hallo"),String::from("kek"),String::from("welt")];

    let distance = levenshtein_distance("hallo wxlt".to_string(), "hallo welt".to_string());
    //println!("{distance}");


    let rx = spawn_key_listener()?;

    loop {
        terminal.draw(|f| {
            let list_items: Vec<ListItem> = items.iter().map(|s| ListItem::new(s.clone())).collect();
            let list: List = List::new(list_items);
            f.render_widget(list, f.size())
        })?;

        if let Ok(key_event) = rx.recv() {
            match key_event.code {
                KeyCode::Esc => break,
                KeyCode::Enter => return Ok(()),
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    terminal.show_cursor()?;

    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

    Ok(())
}

fn levenshtein_distance(u: String, v: String) -> u32 {
    let u_chars: Vec<char> = u.chars().collect();
    let v_chars: Vec<char> = v.chars().collect();

    let m = u.len() + 1;
    let n = v.len() + 1;

    let mut d = vec![vec![0; n]; m];

    for i in 1..m {
        d[i][0] = i;
        for j in 1..n {
            d[0][j] = j;
            let mut replacement_score = d[i - 1][j - 1];
            if u_chars[i - 1] != v_chars[j - 1] {
                replacement_score += 1;
            }

            let insert_score = d[i][j - 1] + 1;
            let delete_score = d[i - 1][j] + 1;

            d[i][j] = *vec![replacement_score, insert_score, delete_score].iter()
                .min()
                .unwrap();
        }
    }

    return d[m - 1][n - 1] as u32;
}
