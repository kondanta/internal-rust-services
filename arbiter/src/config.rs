use color_eyre::Result;
use serde::{Deserialize, Serialize};

use serde_yaml::from_reader;

trait Widget {
    fn render(&self);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Widgets {
    pub widgets: Vec<WidgetConfig>,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct SearchWidget {
    pub placeholder: String,
    pub engine: String,
    pub shortcut: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum WidgetConfig {
    Search(SearchWidget),
}

impl Widget for SearchWidget {
    fn render(&self) {
        tracing::debug!(
            "Rendering Search Widget with placeholder: {}",
            self.placeholder
        );
    }
}

impl WidgetConfig {
    pub fn render(&self) {
        match self {
            WidgetConfig::Search(w) => w.render(),
        }
    }
}

impl Widgets {
    pub fn from_file(path: &str) -> Result<Self> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let config = from_reader(reader)?;

        Ok(config)
    }
}
