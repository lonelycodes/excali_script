// #[macro_use]

use std::{
    fs::File,
    io::{BufWriter, Write},
};

pub struct ExcalidrawDocument {
    pub r#type: String,
    pub version: String,
    pub source: String,
    pub elements: Vec<ExcalidrawElement>,
    pub app_state: ExcalidrawAppState,
}

impl ExcalidrawDocument {
    pub fn new() -> Self {
        Self {
            r#type: "excalidraw".to_string(),
            version: "2.0.0".to_string(),
            source: "https://excalidraw.com".to_string(),
            elements: Vec::new(),
            app_state: ExcalidrawAppState::new(),
        }
    }

    pub fn add_element(&mut self, element: ExcalidrawElement) {
        self.elements.push(element);
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
            self.app_state.to_json()
        )
    }

    pub fn save(&self, path: &str) {
        let file = File::create(path).unwrap();
        let mut writer = BufWriter::new(file);
        writer.write_all(self.to_json().as_bytes()).unwrap();
    }
}

#[derive(Default)]
pub struct ExcalidrawAppState {
    pub view_background_color: String,
}

impl ExcalidrawAppState {
    pub fn new() -> Self {
        Self {
            view_background_color: "#ffffff".to_string(),
        }
    }

    pub fn to_json(&self) -> String {
        format!(
            r#"{{
            "viewBackgroundColor": "{}"
        }}"#,
            self.view_background_color
        )
    }
}
#[derive(Default)]
pub struct ExcalidrawElement {
    pub id: String,
    pub r#type: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub text: String,
    pub bound_elements: Vec<ExcalidrawBoundElement>,
    pub points: Vec<ExcalidrawPoint>,
    pub start_binding: ExcalidrawBinding,
    pub end_binding: ExcalidrawBinding,
    pub stroke_color: String,
}
impl ExcalidrawElement {
    pub fn new_text(text: &str, x: f64, y: f64, id: &str) -> Self {
        Self {
            id: id.to_string(),
            r#type: "text".to_string(),
            x,
            y,
            width: 1.0,
            height: 1.0,
            text: text.to_string(),
            bound_elements: Vec::new(),
            points: Vec::new(),
            start_binding: ExcalidrawBinding::new(),
            end_binding: ExcalidrawBinding::new(),
            stroke_color: "#000000".to_string(),
        }
    }

    pub fn new_arrow(
        points: Vec<ExcalidrawPoint>,
        x: f64,
        y: f64,
        color: String,
        // start_element: ExcalidrawBoundElement,
        // end_element: ExcalidrawBoundElement,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            r#type: "arrow".to_string(),
            x,
            y,
            width: 2.0,
            height: 2.0,
            text: "".to_string(),
            bound_elements: vec![],
            points,
            start_binding: ExcalidrawBinding::new(),
            end_binding: ExcalidrawBinding::new(),
            stroke_color: color,
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
            "strokeColor": "{}",
            "boundElements": [{}],
            "points": [{}],
            "startBinding": {},
            "endBinding": {},
            "fontFamily": 3,
            "fontSize": 20
        }}"#,
            self.id,
            self.r#type,
            self.x,
            self.y,
            self.width,
            self.height,
            self.text,
            self.stroke_color,
            self.bound_elements
                .iter()
                .map(|e| e.to_json())
                .collect::<Vec<String>>()
                .join(","),
            self.points
                .iter()
                .map(|p| p.to_json())
                .collect::<Vec<String>>()
                .join(","),
            self.start_binding.to_json(),
            self.end_binding.to_json()
        )
    }
}

#[derive(Default, Debug, Clone)]
pub struct ExcalidrawPoint {
    pub x: f64,
    pub y: f64,
}

impl ExcalidrawPoint {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn to_json(&self) -> String {
        format!(
            r#"[
            {},
            {}
        ]"#,
            self.x, self.y
        )
    }
}

pub struct ExcalidrawBoundElement {
    pub id: String,
    pub r#type: String,
}

impl ExcalidrawBoundElement {
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

#[derive(Default)]
pub struct ExcalidrawBinding {
    pub element_id: String,
    pub focus: f64,
    pub gap: f64,
}

impl ExcalidrawBinding {
    pub fn new() -> Self {
        Self {
            element_id: "".to_string(),
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
            self.element_id, self.focus, self.gap
        )
    }
}
