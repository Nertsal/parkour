use super::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct Level {
    pub spawn_point: vec2<Coord>,
    pub surfaces: Vec<Surface>,
}

impl Level {
    pub fn new() -> Self {
        Self {
            spawn_point: vec2::ZERO,
            surfaces: default(),
        }
    }

    pub fn save(&self, path: impl AsRef<std::path::Path>) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            serde_json::to_writer_pretty(
                std::fs::File::create(path).expect("Failed to create a file"),
                &self,
            )
            .expect("Failed to serialize the level");
            log::info!("Level saved");
        }
    }

    pub fn load(path: impl AsRef<std::path::Path>) -> Option<Self> {
        #[cfg(target_arch = "wasm32")]
        {
            None
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Some(
                serde_json::from_reader(std::fs::File::open(path).expect("Failed to open a file"))
                    .expect("Failed to deserialize the level"),
            )
        }
    }
}

impl Default for Level {
    fn default() -> Self {
        Self::new()
    }
}
