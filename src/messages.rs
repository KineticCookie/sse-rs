pub trait EngineMessage {
    fn get_url(&self) -> &str;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterGame {
    pub game: String,
    pub game_display_name: String,
    pub icon_color_id: i16
}

impl EngineMessage for RegisterGame {
    fn get_url(&self) -> &str {
        "game_metadata"
    }
}