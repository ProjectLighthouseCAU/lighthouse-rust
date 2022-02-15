use async_std::{task, sync::Mutex};
use lighthouse_client::{Connection, Authentication, LighthouseResult, Display, LIGHTHOUSE_SIZE, GREEN, Color, RED, Pos, Delta};
use log::{info, Level, debug};
use std::{env, collections::{VecDeque, HashSet}, sync::Arc, time::Duration};

const UPDATE_INTERVAL: Duration = Duration::from_millis(200);
const FRUIT_COLOR: Color = RED;
const SNAKE_COLOR: Color = GREEN;
const SNAKE_INITIAL_LENGTH: usize = 3;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Snake {
    fields: VecDeque<Pos>,
    dir: Delta,
}

impl Snake {
    fn from_initial_length(length: usize) -> Self {
        let mut pos = rand::random();
        let dir = Delta::random_direction();

        let mut fields = VecDeque::new();
        for _ in 0..length {
            fields.push_back(pos);
            pos -= dir;
        }

        Self { fields, dir }
    }

    fn head(&self) -> Pos { *self.fields.front().unwrap() }

    fn back(&self) -> Pos { *self.fields.back().unwrap() }

    fn grow(&mut self) {
        self.fields.push_back(self.back() - self.dir);
    }

    fn step(&mut self) {
        let head = self.head();
        self.fields.pop_back();
        self.fields.push_front(head + self.dir);
    }

    fn intersects_itself(&self) -> bool {
        self.fields.iter().collect::<HashSet<_>>().len() < self.fields.len()
    }

    fn rotate_head(&mut self, dir: Delta) {
        self.dir = dir;
    }

    fn render_to(&self, display: &mut Display) {
        for field in &self.fields {
            display.set(field.x as usize, field.y as usize, SNAKE_COLOR);
        }
    }

    fn len(&self) -> usize {
        self.fields.len()
    }

    fn random_fruit_pos(&self) -> Option<Pos> {
        let fields = self.fields.iter().collect::<HashSet<_>>();
        if fields.len() >= LIGHTHOUSE_SIZE {
            None
        } else {
            loop {
                let pos = rand::random();
                if !fields.contains(&pos) {
                    break Some(pos);
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct State {
    snake: Snake,
    fruit: Pos,
}

impl State {
    fn new() -> Self {
        let snake = Snake::from_initial_length(SNAKE_INITIAL_LENGTH);
        let fruit = snake.random_fruit_pos().unwrap();
        Self { snake, fruit }
    }
    
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn step(&mut self) {
        self.snake.step();

        if self.snake.head() == self.fruit {
            self.snake.grow();
            info!("Snake now has length {}", self.snake.len());
            if let Some(fruit) = self.snake.random_fruit_pos() {
                self.fruit = fruit;
            } else {
                info!("You win!");
                self.reset();
            }
        } else if self.snake.intersects_itself() {
            info!("Game over!");
            self.reset();
        }
    }

    fn render(&self) -> Display {
        let mut display = Display::empty();

        display.set(self.fruit.x as usize, self.fruit.y as usize, FRUIT_COLOR);
        self.snake.render_to(&mut display);

        display
    }
}

async fn run_updater(auth: Authentication, shared_state: Arc<Mutex<State>>) -> LighthouseResult<()> {
    let mut conn = Connection::new(auth).await?;
    info!("Connected to the Lighthouse server");

    loop {
        // Update the snake and render it
        let display = {
            let mut state = shared_state.lock().await;
            state.step();
            state.render()
        };

        // Send the rendered snake to the lighthouse
        conn.send_display(display).await?;
        debug!("Sent display");

        // Wait for a short period of time
        task::sleep(UPDATE_INTERVAL).await;
    }
}

async fn run_controller(auth: Authentication, shared_state: Arc<Mutex<State>>) -> LighthouseResult<()> {
    let mut conn = Connection::new(auth).await?;

    // Request input events
    conn.request_stream().await?;

    loop {
        // Receive a user input event from the web interface
        let event = conn.receive_input_event().await?;

        if event.is_down {
            // Map the key code to a direction vector
            let opt_dir = match event.key {
                Some(37) => Some(Delta::new(-1,  0)), // Left
                Some(38) => Some(Delta::new( 0, -1)), // Up
                Some(39) => Some(Delta::new( 1,  0)), // Right
                Some(40) => Some(Delta::new( 0,  1)), // Down
                _ => None,
            };

            // Update the snake's direction
            if let Some(dir) = opt_dir {
                debug!("Rotating snake head to {:?}", dir);
                let mut state = shared_state.lock().await;
                state.snake.rotate_head(dir);
            }
        }
    }
}

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let username = env::var("LIGHTHOUSE_USERNAME").unwrap();
    let token = env::var("LIGHTHOUSE_TOKEN").unwrap();
    let auth = Authentication::new(username.as_str(), token.as_str());
    let state = Arc::new(Mutex::new(State::new()));

    task::spawn(run_updater(auth.clone(), state.clone()));
    task::block_on(run_controller(auth, state)).unwrap();
}
