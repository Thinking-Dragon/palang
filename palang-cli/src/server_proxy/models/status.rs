use serde::Deserialize;
use tabled::Tabled;

#[derive(Debug, Deserialize, Tabled)]
pub struct Status {

}

impl Status {
    pub fn to_pretty_format(&self) -> String {
        String::new()
    }
}
