use serde::Deserialize;

#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Version {
    pub name_clean: String,
    pub protocol: u32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Players {
    pub online: u32,
    pub max: u32,
    pub list: Vec<Player>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Player {
    pub uuid: String,
    pub name_clean: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Motd {
    pub clean: String,
}