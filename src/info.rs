use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Info {
    online: bool,
    host: String,
    port: u32,
    ip_address: String,
    version: Version,
    players: Players,
    motd: Motd
}

#[derive(Debug, Deserialize)]
pub struct Version {
    pub name_clean: String,
    pub protocol: u32,
}

#[derive(Debug, Deserialize)]
pub struct Players {
    pub online: u32,
    pub max: u32,
    pub list: Vec<Player>,
}

#[derive(Debug, Deserialize)]
pub struct Player {
    pub uuid: String,
    pub name_clean: String,
}

#[derive(Debug, Deserialize)]
pub struct Motd {
    pub clean: String,
}