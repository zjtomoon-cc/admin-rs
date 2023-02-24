use gotham::state::State;

const HELLO_WORLD:&str = "Hello World!";

fn say_hello(state:State) -> (State,&'static str) {
    (state,HELLO_WORLD)
}
fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for request at http://{}",addr);
    gotham::start(addr, || Ok(say_hello))
}

#[cfg(test)]
mod tests {
    use super::*;
    use gotham::test::TestServer;
    

    #[test]
    fn receive_hello_world_response() {
        let test_server = TestServer::new(|| Ok(say_hello)).unwrap();
        let response = test_server.client().get("http://localhost").perform().unwrap();

        let body = response.read_body().unwrap();
        assert_eq!(&body[..],b"Hello World!");
    }
}