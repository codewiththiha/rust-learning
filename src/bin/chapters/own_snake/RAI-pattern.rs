use crossterm::{
    ExecutableCommand,
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent},
    style::Print,
    terminal::{self, Clear, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::RngExt;
use std::io::Write;
use std::io::stdout;
use std::thread;
use std::time::{Duration, Instant};
use std::{collections::VecDeque, io::Stdout};

pub struct TerminalGuard {
    stdout: Stdout,
}

#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TerminalGuard {
    // unlike standard Result this is specific for io meaning we don't need explict error type as
    // second arguement
    pub fn new() -> std::io::Result<Self> {
        let mut stdout = stdout();
        stdout.execute(EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
        stdout.execute(Hide)?;
        Ok(TerminalGuard { stdout })
    }

    pub fn get_stdout(&mut self) -> &mut std::io::Stdout {
        &mut self.stdout
    }

    pub fn clear(&mut self) -> std::io::Result<()> {
        self.stdout.execute(Clear(terminal::ClearType::All))?;
        Ok(())
    }

    pub fn clear_from_cursor(&mut self) -> std::io::Result<()> {
        self.stdout
            .execute(Clear(terminal::ClearType::FromCursorDown))?;
        Ok(())
    }

    pub fn get_terminal_size(&self) -> std::io::Result<(u16, u16)> {
        terminal::size()
    }

    pub fn print(&mut self, text: &str) -> std::io::Result<()> {
        self.stdout.execute(Print(text))?;
        self.stdout.flush()
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        self.stdout.flush()?;
        Ok(())
    }
    pub fn move_to(&mut self, x: u16, y: u16) -> std::io::Result<()> {
        self.stdout.execute(MoveTo(x, y))?;
        Ok(())
    }

    pub fn print_at(&mut self, x: u16, y: u16, text: &str) -> std::io::Result<()> {
        self.move_to(x, y)?;
        self.print(text)
    }

    pub fn print_at_center(
        &mut self,
        x: u16,
        y: u16,
        cols: u16,
        rows: u16,
        text: &str,
    ) -> std::io::Result<()> {
        let size = self.get_terminal_size()?;
        let center_x = size.0.saturating_sub(cols) / 2;
        let center_y = size.1.saturating_sub(rows) / 2;
        self.move_to(center_x + x, center_y + y)?;
        self.print(text)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        // Best-effort cleanup — ignore errors during drop
        let _do = self.stdout.execute(LeaveAlternateScreen);
        let _do = terminal::disable_raw_mode();
        // give user's cursor back
        let _do = self.stdout.execute(Show);
    }
}

pub fn draw_border_demo() {
    let start = Instant::now();
    let mut terminal1 = TerminalGuard::new();
    let _ = draw_border(terminal1.as_mut().unwrap(), 80, 40);
    // pause for 2 sec to see if it goes into alternatescreen
    thread::sleep(Duration::from_secs(2));
    let size = terminal1.unwrap().get_terminal_size().unwrap();
    let end = start.elapsed();
    println!("{} , {} , took{}seconds", size.0, size.1, end.as_secs_f64());
}

pub fn read_input() -> Result<Option<KeyCode>, Box<dyn std::error::Error>> {
    if let Event::Key(KeyEvent { code, .. }) = event::read()? {
        return Ok(Some(code));
    }
    Ok(None)
}

pub fn input_read_demo() -> Result<(), Box<dyn std::error::Error>> {
    let _t = TerminalGuard::new()?;
    loop {
        if let Some(key) = read_input()? {
            match key {
                KeyCode::Char('q') => break,
                KeyCode::Up => println!("Up!"),
                KeyCode::Down => println!("Down!"),
                KeyCode::Left => println!("Left!"),
                KeyCode::Right => println!("Right!"),
                KeyCode::Char('w') => println!("Up (W)!"),
                KeyCode::Char('s') => println!("Down (S)!"),
                KeyCode::Char('a') => println!("Left (A)!"),
                KeyCode::Char('d') => println!("Right (D)!"),
                _ => {}
            }
        }
    }
    Ok(())
}

pub fn example() -> std::io::Result<()> {
    let mut term = TerminalGuard::new()?;
    let x = 60;
    let y = 30;
    draw_border(&mut term, x, y)?;
    term.print_at_center(1, 27, x, y, "@")?;
    term.print_at_center(1, 28, x, y, "O")?;

    thread::sleep(Duration::from_secs(2));
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut term = TerminalGuard::new()?;
    let cols = 60;
    let rows = 30;
    let mut body = VecDeque::from(vec![Point { x: 30, y: 15 }, Point { x: 30, y: 16 }]);
    let mut dir = Direction::Right;
    let mut food_p = spawn_food(&body, cols, rows);

    'main_loop: loop {
        // initial draw
        draw_border(&mut term, cols, rows)?;

        for (i, p) in body.iter().enumerate() {
            term.print_at_center(p.x, p.y, cols, rows, if i == 0 { "@" } else { "O" })?;
        }
        term.print_at_center(food_p.x, food_p.y, cols, rows, "*")?;

        if let Some(key) = read_input()? {
            match key {
                KeyCode::Char('w') | KeyCode::Up if dir != Direction::Down => dir = Direction::Up,
                KeyCode::Char('s') | KeyCode::Down if dir != Direction::Up => dir = Direction::Down,
                KeyCode::Char('a') | KeyCode::Left if dir != Direction::Right => {
                    dir = Direction::Left
                }
                KeyCode::Char('d') | KeyCode::Right if dir != Direction::Left => {
                    dir = Direction::Right
                }
                KeyCode::Char('q') | KeyCode::Esc => break 'main_loop,
                _ => {}
            }
        };
        let mut new_head = *body.front().unwrap();

        match dir {
            Direction::Up => new_head.y = new_head.y.saturating_sub(1),
            Direction::Down => new_head.y += 1,
            Direction::Left => new_head.x = new_head.x.saturating_sub(1),
            Direction::Right => new_head.x += 1,
        }

        body.push_front(new_head);
        if new_head == food_p {
            food_p = spawn_food(&body, cols, rows);
        } else {
            body.pop_back();
        }
        term.clear()?;
    }

    // initial draw
    // term.print_at_center(30, 15, cols, rows, "@")?;
    // term.print_at_center(30, 16, cols, rows, "O")?;
    Ok(())
}

// note this saturating_sub method prevent the unsigned integers to go crazy when we do 0 - 1
// since unsigned integers have no minus values , so think of that method as safe extract
fn draw_border(term: &mut TerminalGuard, cols: u16, rows: u16) -> std::io::Result<()> {
    let size = term.get_terminal_size().unwrap();
    term.clear().unwrap();

    // values needed to center the border
    let start_x = (size.0.saturating_sub(cols)) / 2;
    let start_y = (size.1.saturating_sub(rows)) / 2; // Fixed: was using size.0 for Y

    // Draw top and bottom horizontal lines
    for x in 0..cols {
        term.print_at(start_x + x, start_y, "─")?;
        term.print_at(start_x + x, start_y + rows - 1, "─")?;
    }

    // Draw left and right vertical lines
    for y in 0..rows {
        term.print_at(start_x, start_y + y, "│")?;
        term.print_at(start_x + cols - 1, start_y + y, "│")?;
    }

    // Draw corners
    term.print_at(start_x, start_y, "┌")?;
    term.print_at(start_x + cols - 1, start_y, "┐")?;
    term.print_at(start_x, start_y + rows - 1, "└")?;
    term.print_at(start_x + cols - 1, start_y + rows - 1, "┘")?;

    Ok(())
}

fn spawn_food(body: &VecDeque<Point>, cols: u16, rows: u16) -> Point {
    let mut rng = rand::rng(); // New 0.10.0 function name
    loop {
        let p = Point {
            // New 0.10.0 method name: random_range
            x: rng.random_range(1..cols - 1),
            y: rng.random_range(1..rows - 1),
        };
        if !body.contains(&p) {
            return p;
        }
    }
}

// fn draw_border(term: &mut TerminalGuard, cols: u16, rows: u16) -> std::io::Result<()> {
//     let size = term.get_terminal_size().unwrap();
//     let start_x = size.0.saturating_sub(cols) / 2;
//     let start_y = size.1.saturating_sub(rows) / 2;
//
//     for x in 0..cols {
//         term.print_at(x, 0, "─")?;
//         term.print_at(x, rows.saturating_sub(1), "─")?;
//     }
//     for y in 0..rows {
//         term.print_at(0, y, "│")?;
//         term.print_at(cols.saturating_sub(1), y, "│")?;
//     }
//     term.print_at(0, 0, "┌")?;
//     term.print_at(cols.saturating_sub(1), 0, "┐")?;
//     term.print_at(0, rows.saturating_sub(1), "└")?;
//     term.print_at(cols.saturating_sub(1), rows.saturating_sub(1), "┘")?;
//     Ok(())
// }
