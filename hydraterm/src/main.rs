
use std::io::{Write, stdout, self, ErrorKind};
use crossterm::event::{Event, KeyModifiers, KeyEvent, KeyCode};
use crossterm::style::Print;
use crossterm::{ cursor, execute, event::read};
use crossterm::terminal::{Clear, ClearType, size, enable_raw_mode, disable_raw_mode};
use hydralang::model::{script, Script};

fn mainloop() -> io::Result<()> {

    let mut stdout = io::stdout();

    let mut input_buffer = String::new();

    let mut script = Script::new(Vec::new(), Vec::new());


    loop {

        let (cols, rows) = size()?;

        execute!(
            stdout,
            Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;


        for (i, line) in script.to_string().split("\n").enumerate() {
            execute!(
                stdout,
                Print(line.to_string()),
                cursor::MoveTo(0, i as u16 + 1)
            )?;
        }


        execute!(
            stdout,
            cursor::MoveTo(0, rows - 3),
            Print("─".repeat(cols as usize)),
            cursor::MoveTo(0, rows - 2),
            Print(input_buffer.clone()),
            cursor::MoveTo(input_buffer.len() as u16, rows - 2)
        )?;


        // This program doesn't really have state updates outside events so we don't need to check for that
        let event = read()?;
        match event {
            Event::Key(k) => {

                if k.modifiers.contains(KeyModifiers::CONTROL) {
                    if let KeyCode::Char('c') = k.code {
                        return Ok(());
                    }
                }

                if let KeyCode::Char(ch) = k.code {
                    input_buffer.push(ch);
                }

                if let KeyCode::Backspace = k.code {
                    input_buffer.pop();
                }

                if let KeyCode::Enter = k.code {
                    if input_buffer.trim() == "exit" {
                        return Ok(());
                    }

                    let new_statement = script::Script::parse(input_buffer.as_str())
                        .map_err(|_| io::Error::new(ErrorKind::InvalidData, "Error Parsing Statement"))?;

                    script.merge(&new_statement);
                    input_buffer.clear();
                    script.run();
                }

            },
            _ => {}

        }

        stdout.flush()?;
    }
}

fn main() -> io::Result<()>{

    
    enable_raw_mode()?;
    mainloop()?;

    execute!(
        io::stdout(),
        Clear(ClearType::All)
    )?;

    disable_raw_mode()?;

    Ok(())
}
