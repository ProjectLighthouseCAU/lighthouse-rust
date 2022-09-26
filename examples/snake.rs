use async_std::{task, sync::Mutex, stream::StreamExt};
use futures::Stream;
use lighthouse_client::{Lighthouse, Authentication, Result, Frame, LIGHTHOUSE_SIZE, GREEN, Color, RED, Pos, Delta, Payload, ServerMessage, AsyncStdWebSocket};
use tracing::{info, debug};
use tracing_subscriber::EnvFilter;
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
        let mut pos: Pos = rand::random();
        let dir = Delta::random_direction();

        let mut fields = VecDeque::new();
        for _ in 0..length {
            fields.push_back(pos);
            pos = pos.sub_wrapping(dir);
        }

        Self { fields, dir }
    }

    fn head(&self) -> Pos { *self.fields.front().unwrap() }

    fn back(&self) -> Pos { *self.fields.back().unwrap() }

    fn grow(&mut self) {
        self.fields.push_back(self.back().sub_wrapping(self.dir));
    }

    fn step(&mut self) {
        let head = self.head();
        self.fields.pop_back();
        self.fields.push_front(head.add_wrapping(self.dir));
    }

    fn intersects_itself(&self) -> bool {
        self.fields.iter().collect::<HashSet<_>>().len() < self.fields.len()
    }

    fn rotate_head(&mut self, dir: Delta) {
        self.dir = dir;
    }

    fn render_to(&self, frame: &mut Frame) {
        for field in &self.fields {
            frame[*field] = SNAKE_COLOR;
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
            let length = self.snake.len();
            info! { %length, "Snake grew" };
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

    fn render(&self) -> Frame {
        let mut frame = Frame::empty();

        frame[self.fruit] = FRUIT_COLOR;
        self.snake.render_to(&mut frame);

        frame
    }
}

async fn run_updater(mut lh: Lighthouse<AsyncStdWebSocket>, shared_state: Arc<Mutex<State>>) -> Result<()> {
    loop {
        // Update the snake and render it
        let frame = {
            let mut state = shared_state.lock().await;
            state.step();
            state.render()
        };

        // Send the rendered snake to the lighthouse
        lh.put_model(frame).await?;
        debug!("Sent frame");

        // Wait for a short period of time
        task::sleep(UPDATE_INTERVAL).await;
    }
}

async fn run_controller(mut stream: impl Stream<Item = ServerMessage> + Unpin, shared_state: Arc<Mutex<State>>) -> Result<()> {
    while let Some(msg) = stream.next().await {
        if let Payload::InputEvent(event) = msg.payload {
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

    Ok(())
}

#[async_std::main]
async fn main() {
    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let username = env::var("LIGHTHOUSE_USER").unwrap();
    let token = env::var("LIGHTHOUSE_TOKEN").unwrap();
    let auth = Authentication::new(username.as_str(), token.as_str());
    let state = Arc::new(Mutex::new(State::new()));

    let mut lh = Lighthouse::connect_with_async_std(auth).await.unwrap();
    info!("Connected to the Lighthouse server");

    let stream = lh.stream_model().await.unwrap();

    task::spawn(run_updater(lh, state.clone()));
    task::block_on(run_controller(stream, state)).unwrap();
}
