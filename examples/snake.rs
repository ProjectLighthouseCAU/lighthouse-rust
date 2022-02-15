use async_std::{task, sync::Mutex};
use lighthouse_client::{Connection, Authentication, LighthouseResult, LIGHTHOUSE_COLS, LIGHTHOUSE_ROWS, Display, BLACK, LIGHTHOUSE_SIZE, GREEN};
use log::{info, Level, debug};
use rand::prelude::*;
use std::{env, collections::VecDeque, sync::Arc, time::Duration};

const UPDATE_INTERVAL: Duration = Duration::from_millis(300);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn random_pos() -> Self {
        let mut rng = thread_rng();
        Vec2::new(rng.gen_range(0..(LIGHTHOUSE_COLS as i32)), rng.gen_range(0..(LIGHTHOUSE_ROWS as i32)))
    }

    fn random_dir() -> Self {
        let random_offset = || { if thread_rng().gen() { 1 } else { -1 } };
        if thread_rng().gen() {
            Vec2::new(0, random_offset())
        } else {
            Vec2::new(random_offset(), 0)
        }
    }

    fn pixel_index(self) -> usize {
        self.y as usize * LIGHTHOUSE_COLS + self.x as usize
    }

    fn add_wrapping(self, rhs: Self) -> Self {
        Self::new(
            (self.x + rhs.x).rem_euclid(LIGHTHOUSE_COLS as i32),
            (self.y + rhs.y).rem_euclid(LIGHTHOUSE_ROWS as i32),
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Snake {
    fields: VecDeque<Vec2>,
    dir: Vec2,
}

impl Snake {
    fn from_initial_length(length: usize) -> Self {
        let mut pos = Vec2::random_pos();
        let dir = Vec2::random_dir();

        let mut fields = VecDeque::new();
        for _ in 0..length {
            fields.push_back(pos);
            pos = pos.add_wrapping(dir);
        }

        Self { fields, dir }
    }

    fn step(&mut self) {
        let head = *self.fields.front().unwrap();
        self.fields.pop_back();
        self.fields.push_front(head.add_wrapping(self.dir));
    }

    fn render(&self) -> Display {
        let mut pixels = [BLACK; LIGHTHOUSE_SIZE];
        debug!("Fields: {:?}", &self.fields);

        for field in &self.fields {
            pixels[field.pixel_index()] = GREEN;
        }

        Display::new(pixels)
    }
}

async fn run_updater(auth: Authentication, shared_snake: Arc<Mutex<Snake>>) -> LighthouseResult<()> {
    let mut conn = Connection::new(auth).await?;
    info!("Connected to the Lighthouse server");

    loop {
        // Update the snake and render it
        let display = {
            let mut snake = shared_snake.lock().await;
            snake.step();
            snake.render()
        };

        // Send the rendered snake to the lighthouse
        conn.send_display(display).await?;
        debug!("Sent display");

        // Wait for a short period of time
        task::sleep(UPDATE_INTERVAL).await;
    }
}

async fn run_controller(auth: Authentication, shared_snake: Arc<Mutex<Snake>>) -> LighthouseResult<()> {
    let mut conn = Connection::new(auth).await?;

    // Request input events
    conn.request_stream().await?;

    loop {
        // Receive a user input event from the web interface
        let event = conn.receive_input_event().await?;

        if event.is_down {
            // Map the key code to a direction vector
            let opt_dir = match event.key {
                Some(37) => Some(Vec2::new(-1,  0)), // Left
                Some(38) => Some(Vec2::new( 0, -1)), // Up
                Some(39) => Some(Vec2::new( 1,  0)), // Right
                Some(40) => Some(Vec2::new( 0,  1)), // Down
                _ => None,
            };

            // Update the snake's direction
            if let Some(dir) = opt_dir {
                info!("Rotating snake to point to {:?}", dir);
                let mut snake = shared_snake.lock().await;
                snake.dir = dir;
            }
        }
    }
}

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let username = env::var("LIGHTHOUSE_USERNAME").unwrap();
    let token = env::var("LIGHTHOUSE_TOKEN").unwrap();
    let auth = Authentication::new(username.as_str(), token.as_str());
    let snake = Arc::new(Mutex::new(Snake::from_initial_length(3)));

    task::spawn(run_updater(auth.clone(), snake.clone()));
    task::block_on(run_controller(auth, snake)).unwrap();
}
