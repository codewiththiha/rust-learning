use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{Event, KeyCode, poll, read},
    execute,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use rand::RngExt; // In 0.10.0, use RngExt to get random_range
use std::collections::VecDeque;
use std::io::{Write, stdout};
use std::time::{Duration, Instant};

#[derive(Debug, PartialEq, Clone, Copy)]
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut out = stdout();
    execute!(out, Hide, Clear(ClearType::All))?;

    let (cols, rows) = crossterm::terminal::size()?;

    let mut body = VecDeque::from(vec![Point { x: 10, y: 10 }, Point { x: 9, y: 10 }]);
    let mut dir = Direction::Right;
    let mut food = spawn_food(&body, cols, rows);
    let mut score = 0;
    let mut last_tick = Instant::now();

    'game: loop {
        if poll(Duration::from_millis(0))? {
            if let Event::Key(key) = read()? {
                match key.code {
                    KeyCode::Char('w') | KeyCode::Up if dir != Direction::Down => {
                        dir = Direction::Up
                    }
                    KeyCode::Char('s') | KeyCode::Down if dir != Direction::Up => {
                        dir = Direction::Down
                    }
                    KeyCode::Char('a') | KeyCode::Left if dir != Direction::Right => {
                        dir = Direction::Left
                    }
                    KeyCode::Char('d') | KeyCode::Right if dir != Direction::Left => {
                        dir = Direction::Right
                    }
                    KeyCode::Char('q') | KeyCode::Esc => break 'game,
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= Duration::from_millis(100) {
            // to get own value and we intentionally copy the old head's position
            let mut new_head = *body.front().unwrap();

            match dir {
                Direction::Up => new_head.y = new_head.y.saturating_sub(1),
                Direction::Down => new_head.y += 1,
                Direction::Left => new_head.x = new_head.x.saturating_sub(1),
                Direction::Right => new_head.x += 1,
            }

            if new_head.x >= cols || new_head.y >= rows || body.contains(&new_head) {
                break 'game;
            }

            body.push_front(new_head);

            if new_head == food {
                score += 1;
                food = spawn_food(&body, cols, rows);
            } else {
                body.pop_back();
            }
            last_tick = Instant::now();
        }

        execute!(out, Clear(ClearType::All))?;

        // Draw Food
        execute!(out, MoveTo(food.x, food.y))?;
        print!("*");

        // Draw Snake
        for (i, p) in body.iter().enumerate() {
            execute!(out, MoveTo(p.x, p.y))?;
            print!("{}", if i == 0 { "@" } else { "o" });
        }

        execute!(out, MoveTo(0, 0))?;
        print!("Score: {}", score);
        out.flush()?;
    }

    execute!(out, Show, MoveTo(0, rows - 1))?;
    disable_raw_mode()?;
    println!("Game Over! Score: {}", score);
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
