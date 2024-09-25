use std::{env, fs, path::PathBuf, time::Duration};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::{net::TcpStream, time::timeout};

pub mod models;
pub mod handlers;

pub struct ServerProxy {
    host: String,
    port: u16,
}

#[derive(Serialize, Deserialize)]
struct Connection {
    host: String,
    port: u16,
}

impl ServerProxy {
    pub fn from(host: String, port: u16) -> Self {
        ServerProxy { host, port }
    }

    pub fn find_server() -> Result<ServerProxy, String> {
        Ok(ServerProxy::from("127.0.0.1".to_string(), 8242))
    }

    fn get_base_directory() -> PathBuf {
        if let Ok(snap_user_data) = env::var("SNAP_USER_DATA") {
            PathBuf::from(snap_user_data)
        } else {
            dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")).join(".palang")
        }
    }

    pub fn is_connected() -> bool {
        let connections_file = Self::get_base_directory().join("connections.yaml");

        if !connections_file.exists() {
            return false;
        }

        match fs::read_to_string(&connections_file) {
            Ok(contents) => {
                match serde_yaml::from_str::<Connection>(&contents) {
                    Ok(_) => true,
                    Err(_) => false,
                }
            },
            Err(_) => false,
        }
    }

    pub async fn connect(host: &String, port: &u16) -> Result<(), String> {
        match timeout(Duration::from_secs(5), TcpStream::connect(format!("{}:{}", host, port))).await {
            Ok(Ok(_)) => {
                let connection = Connection {
                    host: host.clone(),
                    port: *port,
                };

                let connections_file = Self::get_base_directory().join("connections.yaml");
                fs::create_dir_all(connections_file.parent().unwrap())
                    .map_err(|e| format!("Failed to create directory: {}", e))?;

                let yaml = serde_yaml::to_string(&connection)
                    .map_err(|e| format!("Failed to serialize connection: {}", e))?;

                fs::write(&connections_file, yaml)
                    .map_err(|e| format!("Failed to write connection file: {}", e))?;

                Ok(())
            },
            Ok(Err(e)) => Err(format!("Failed to connect: {}", e)),
            Err(_) => Err("Connection timed out".to_string()),
        }
    }

    pub fn disconnect() -> Result<(), String> {
        let connections_file: PathBuf = Self::get_base_directory().join("connections.yaml");
        fs::remove_file(connections_file).map_err(|e| e.to_string())
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
