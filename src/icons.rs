use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Icon {
    pub file: String,
    pub display_name: String,
}

impl Icon {
    pub fn get_all() -> Vec<Icon> {
        vec![
            // do try to keep these in alphanumeric order by filepath, pls
            Icon::new("blender.png", "Blender3D"),
            Icon::new("deepnight.png", "Deepnight"),
            Icon::new("ferris-happy.svg", "Ferris"),
            Icon::new("gamasutra.ico", "Gamasutra"),
            Icon::new("gdc.svg", "GDC"),
            Icon::new("github.png", "Github"),
            Icon::new("safari.png", "Safari"),
            Icon::new("swiftlang.svg", "Swift"),
            Icon::new("youtube.svg", "Youtube"),
        ]
    }

    fn new(file: &str, display_name: &str) -> Icon {
        Icon {
            file: file.to_owned(),
            display_name: display_name.to_owned(),
        }
    }
}
