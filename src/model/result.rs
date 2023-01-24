use chrono::{Date, Utc};

#[derive(Debug)]
pub struct Result<'a> {
    pub category: &'a str,
    pub name: &'a str,
    pub download_link: &'a str,
    pub magnet_link: &'a str,
    pub size: f32,
    pub date: Date<Utc>,
    pub seeders: u16,
    pub leechers: u16,
    pub completed: u32,
}

impl PartialEq for Result<'_>{
    fn eq(&self, other: &Self) -> bool {
        self.category == other.category &&
        self.name == other.name &&
        self.download_link == other.download_link &&
        self.magnet_link == other.magnet_link &&
        self.size == other.size &&
        self.date == other.date &&
        self.seeders == other.seeders &&
        self.leechers == other.leechers &&
        self.completed == other.completed
    }
}

