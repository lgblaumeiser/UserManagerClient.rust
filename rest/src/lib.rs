pub mod client {
    #[derive(Debug)]
    pub struct Client {
        base_url: String,
        token: String,
        client: reqwest::blocking::Client
    }

    pub impl Client {
        pub fn new(base: &str, token: &str) -> Client {
            Client {
                base_url base.to_string(),
                token: token.to_string(),
                client: reqwest::blocking::client::new()
            }
        }

        pub fn post(&self, route: &String, body: &String, headers: &String) -> Result<String,  {
            let url = format!("{}/{}", base_url, *route)
            let res = client.post(url)
                .headers(*headers)
                .body(*body)
                .send()?;

        }
    }

}
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
