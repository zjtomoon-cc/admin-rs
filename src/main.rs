use gotham::state::State;

mod config;
mod tests;

const HELLO_WORLD: &str = "Hello World!";

pub fn say_hello(state: State) -> (State, &'static str) {
    (state, HELLO_WORLD)
}

fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for request at http://{}", addr);
    gotham::start(addr, || Ok(say_hello))
}

