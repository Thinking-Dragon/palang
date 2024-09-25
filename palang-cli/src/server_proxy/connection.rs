use std::{env, fs, path::PathBuf, time::Duration};

use serde::{Deserialize, Serialize};
use tokio::{net::TcpStream, time::timeout};

use super::ServerProxy;

#[derive(Debug, Serialize, Deserialize)]
struct Connection {
    host: String,
    port: u16,
}

impl ServerProxy {
    pub fn is_connected() -> bool {
        let connections_file = get_base_directory().join("connections.yaml");

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
        match timeout(
            Duration::from_secs(5),
            TcpStream::connect(format!("{}:{}", host, port))
        ).await {
            Ok(Ok(_)) => {
                let connection = Connection {
                    host: host.clone(),
                    port: *port,
                };

                let connections_file = get_base_directory().join("connections.yaml");
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
        let connections_file: PathBuf = get_base_directory().join("connections.yaml");
        fs::remove_file(connections_file).map_err(|e| e.to_string())
    }

    pub fn find_server() -> Result<ServerProxy, String> {
        let connections_file = get_base_directory().join("connections.yaml");

        if connections_file.exists() {
            match fs::read_to_string(&connections_file) {
                Ok(contents) => {
                    match serde_yaml::from_str::<Connection>(&contents) {
                        Ok(connection) => {
                            Ok(ServerProxy::from(connection.host, connection.port))
                        },
                        Err(e) => {
                            Err(format!("Failed to parse connection file: {}", e))
                        }
                    }
                },
                Err(e) => {
                    Err(format!("Failed to read connection file: {}", e))
                }
            }
        } else {
            Err("No connection file found. Please connect to a server first.".to_string())
        }
    }
}

fn get_base_directory() -> PathBuf {
    if let Ok(snap_user_data) = env::var("SNAP_USER_DATA") {
        PathBuf::from(snap_user_data)
    } else {
        dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")).join(".palang")
    }
}
