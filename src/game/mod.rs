#[derive(Debug)]
pub struct Game {
    pub name: String,
    pub icon: String,
    pub thumbnail: String,
}

impl Game {
    pub fn new(name: &str, icon: &str, thumbnail: &str) -> Self {
        Self {
            name: name.to_string(),
            icon: icon.to_string(),
            thumbnail: thumbnail.to_string(),
        }
    }
}
