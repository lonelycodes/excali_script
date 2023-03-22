// #[macro_use]

use std::{
    fs::File,
    io::{BufWriter, Write},
};

use uuid::Uuid;

pub struct ExcalidrawDocument {
    pub r#type: String,
    pub version: String,
    pub source: String,
    pub elements: Vec<ExcalidrawElement>,
    pub appState: ExcalidrawAppState,
}

impl ExcalidrawDocument {
    pub fn new() -> Self {
        Self {
            r#type: "excalidraw".to_string(),
            version: "2.0.0".to_string(),
            source: "https://excalidraw.com".to_string(),
            elements: Vec::new(),
            appState: ExcalidrawAppState::new(),
        }
    }

    pub fn to_json(&self) -> String {
        format!(
            r#"{{
            "type": "{}",
            "version": "{}",
            "source": "{}",
            "elements": [{}],
            "appState": {}
        }}"#,
            self.r#type,
            self.version,
            self.source,
            self.elements
                .iter()
                .map(|e| e.to_json())
                .collect::<Vec<String>>()
                .join(","),
            self.appState.to_json()
        )
    }

    pub fn save(&self, path: &str) {
        let file = File::create(path).unwrap();
        let mut writer = BufWriter::new(file);
        writer.write_all(self.to_json().as_bytes()).unwrap();
    }
}

pub struct ExcalidrawAppState {
    pub viewBackgroundColor: String,
}

impl ExcalidrawAppState {
    pub fn new() -> Self {
        Self {
            viewBackgroundColor: "#ffffff".to_string(),
        }
    }

    pub fn to_json(&self) -> String {
        format!(
            r#"{{
            "viewBackgroundColor": "{}"
        }}"#,
            self.viewBackgroundColor
        )
    }
}

pub struct ExcalidrawElement {
    pub id: String,
    pub r#type: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub text: String,
    pub boundElements: Vec<ExcalidrawBoundElement>,
    pub points: Vec<ExcalidrawPoint>,
    pub startBinding: ExcalidrawBinding,
    pub endBinding: ExcalidrawBinding,
}

impl ExcalidrawElement {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            r#type: "rectangle".to_string(),
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            text: "".to_string(),
            boundElements: Vec::new(),
            points: Vec::new(),
            startBinding: ExcalidrawBinding::new(),
            endBinding: ExcalidrawBinding::new(),
        }
    }

    pub fn to_json(&self) -> String {
        format!(
            r#"{{
            "id": "{}",
            "type": "{}",
            "x": {},
            "y": {},
            "width": {},
            "height": {},
            "text": "{}",
            "boundElements": [{}],
            "points": [{}],
            "startBinding": {},
            "endBinding": {}
        }}"#,
            self.id,
            self.r#type,
            self.x,
            self.y,
            self.width,
            self.height,
            self.text,
            self.boundElements
                .iter()
                .map(|e| e.to_json())
                .collect::<Vec<String>>()
                .join(","),
            self.points
                .iter()
                .map(|p| p.to_json())
                .collect::<Vec<String>>()
                .join(","),
            self.startBinding.to_json(),
            self.endBinding.to_json()
        )
    }
}

pub struct ExcalidrawPoint {
    pub x: f64,
    pub y: f64,
}

impl ExcalidrawPoint {
    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn to_json(&self) -> String {
        format!(
            r#"{{
            "x": {},
            "y": {}
        }}"#,
            self.x, self.y
        )
    }
}

pub struct ExcalidrawBoundElement {
    pub id: String,
    pub r#type: String,
}

impl ExcalidrawBoundElement {
    pub fn new() -> Self {
        Self {
            id: "".to_string(),
            r#type: "".to_string(),
        }
    }

    pub fn to_json(&self) -> String {
        format!(
            r#"{{
            "id": "{}",
            "type": "{}"
        }}"#,
            self.id, self.r#type
        )
    }
}

pub struct ExcalidrawBinding {
    pub elementId: String,
    pub focus: f64,
    pub gap: f64,
}

impl ExcalidrawBinding {
    pub fn new() -> Self {
        Self {
            elementId: "".to_string(),
            focus: 0.0,
            gap: 0.0,
        }
    }

    pub fn to_json(&self) -> String {
        format!(
            r#"{{
            "elementId": "{}",
            "focus": {},
            "gap": {}
        }}"#,
            self.elementId, self.focus, self.gap
        )
    }
}
