use async_std::task;

async fn say_hello() {
    println!("Hello");
}

fn main() {
    // TODO
    task::block_on(say_hello());
}
