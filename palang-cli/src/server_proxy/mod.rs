use serde::{de::DeserializeOwned, Serialize};

pub mod connection;
pub mod models;
pub mod handlers;

pub struct ServerProxy {
    pub host: String,
    pub port: u16,
}

impl ServerProxy {
    pub fn from(host: String, port: u16) -> Self {
        ServerProxy { host, port }
    }

    fn make_url(&self, route: &str) -> String {
        format!("{}:{}/{}", self.host, self.port, route)
    }

    fn get<T>(&self, route: &str) -> Result<T, String>
        where T: DeserializeOwned
    {
        let url = self.make_url(route);
        reqwest::blocking::get(url)
            .map_err(|e| e.to_string())?
            .json()
            .map_err(|e| e.to_string())
    }

    fn post<T, U>(&self, route: &str, body: &T) -> Result<U, String>
        where T: Serialize, U: DeserializeOwned
    {
        let url = self.make_url(route);
        reqwest::blocking::Client::new()
            .post(url)
            .json(body)
            .send()
            .map_err(|e| e.to_string())?
            .json()
            .map_err(|e| e.to_string())
    }
}
