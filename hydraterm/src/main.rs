
use std::error::Error;
use std::fmt::Display;
use std::io::{Write, self, Stdout};
use crossterm::event::{Event, KeyModifiers, KeyCode, KeyEventKind};
use crossterm::style::Print;
use crossterm::{ cursor, execute, event::read};
use crossterm::terminal::{Clear, ClearType, size, enable_raw_mode, disable_raw_mode};
use hydralang::builtin::base::base_config;
use hydralang::model::{script, Script};

#[derive(Debug)]
enum WindowReturn {
    Exit,
    Error(Box<dyn Error>)
}

impl Display for WindowReturn {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for WindowReturn {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            WindowReturn::Exit => None,
            WindowReturn::Error(e) => Some(e.as_ref()),
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

fn handle_event(app: &mut Application, event: Event) -> Result<(), WindowReturn> {
    match event {
        Event::Key(k) => {

            if !(k.kind == KeyEventKind::Press) {
                return Ok(())
            }

            if k.modifiers.contains(KeyModifiers::CONTROL) {
                if let KeyCode::Char('c') = k.code {
                    return Err(WindowReturn::Exit);
                }
            }

            if let KeyCode::Char(ch) = k.code {
                app.input_buffer.push(ch);
            }

            if let KeyCode::Backspace = k.code {
                app.input_buffer.pop();
            }

            if let KeyCode::Enter = k.code {
                if app.input_buffer.trim() == "exit" {
                    return Err(WindowReturn::Exit);
                }

                if !(app.input_buffer.trim() == "") {

                    let new_statement = script::Script::parse(app.input_buffer.as_str())
                        .map_err(|e| WindowReturn::Error(Box::new(e) as Box<dyn Error>))?;

                    app.script.merge(&new_statement);
                    app.input_buffer.clear();
                    app.script.run();
                
                }
            }

        },
        _ => {}

    }

    return Ok(());
}

struct Application{
    pub stdout: Stdout,
    pub input_buffer: String,
    pub script: Script
}

impl Application {

    fn new() -> Self {
        Self {
            stdout: io::stdout(),
            input_buffer: String::new(),
            script: base_config()
        }
    }

}

fn mainloop() -> Result<(), Box<dyn Error>> {

    let mut app = Application::new();


    loop {

        let (cols, rows) = size()?;

        execute!(
            app.stdout,
            Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;


        for (i, line) in app.script.to_string().split("\n").enumerate() {
            execute!(
                app.stdout,
                Print(line.to_string()),
                cursor::MoveTo(0, i as u16 + 1)
            )?;
        }


        execute!(
            app.stdout,
            cursor::MoveTo(0, rows - 3),
            Print("─".repeat(cols as usize)),
            cursor::MoveTo(0, rows - 2),
            Print(app.input_buffer.clone()),
            cursor::MoveTo(app.input_buffer.len() as u16, rows - 2)
        )?;


        // This program doesn't really have state updates outside events so we don't need to check for that
        let event = read()?;
        let event_handle_result = handle_event(&mut app, event);
        if let Err(e) = event_handle_result {
            match e {
                WindowReturn::Exit => return Result::Ok(()),
                WindowReturn::Error(e) => return Result::Err(e),
            }
        }

        app.stdout.flush()?;
    }
}

fn main() -> Result<(), Box<dyn Error>>{

    
    enable_raw_mode()?;
    mainloop()?;

    execute!(
        io::stdout(),
        Clear(ClearType::All)
    )?;

    disable_raw_mode()?;

    Ok(())
}
