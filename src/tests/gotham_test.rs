#[cfg(test)]
mod tests {
    use gotham::test::TestServer;
    use crate::say_hello;


    #[test]
    fn receive_hello_world_response() {
        let test_server = TestServer::new(|| Ok(say_hello)).unwrap();
        let response = test_server.client().get("http://localhost").perform().unwrap();

        let body = response.read_body().unwrap();
        assert_eq!(&body[..], b"Hello World!");
    }

}