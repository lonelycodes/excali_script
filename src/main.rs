mod excalidraw;
mod jsops;

use core::panic;
use excalidraw::{ExcalidrawDocument, ExcalidrawElement, ExcalidrawPoint};
use jsops::FileNode;
use std::{collections::HashMap, path::Path};
use walkdir::WalkDir;

fn main() {
    let directory = get_directory();
    let files = get_all_js_files(directory);

    let dependency_map = build_dependency_map(files);

    // dbg!(dependency_map.clone());

    tmp_excalidraw_playground(dependency_map);
}

fn tmp_excalidraw_playground(dependency_map: HashMap<String, Vec<FileNode>>) {
    let mut document = ExcalidrawDocument::new();

    let mut y_value = 0.0;
    let y_increment = 50.0;

    let arrow_x_offset = -10.0;
    let arrow_y_offset = 10.0;

    dependency_map.iter().for_each(|(k, _v)| {
        let mut element = ExcalidrawElement::new_text(k, 0.0, y_value);
        y_value += y_increment;
        element.text = k.to_string();
        document.add_element(element);
    });

    let multi_point_arrow = ExcalidrawElement::new_arrow(
        vec![
            ExcalidrawPoint::new(0.0, 0.0),
            ExcalidrawPoint::new(-50.0, 0.0),
            ExcalidrawPoint::new(-50.0, 50.0),
            ExcalidrawPoint::new(0.0, 50.0),
        ],
        arrow_x_offset,
        arrow_y_offset,
    );
    document.add_element(multi_point_arrow);

    document.save("out.excalidraw");
}

fn build_dependency_map(files: Vec<String>) -> HashMap<String, Vec<FileNode>> {
    let mut dependency_map: HashMap<String, Vec<FileNode>> = HashMap::new();

    let dependencies = files
        .iter()
        .map(|f| jsops::parse(f))
        .map(|m| jsops::get_dependencies(&m))
        .collect::<Vec<Vec<jsops::FileNode>>>();

    files.iter().zip(dependencies.iter()).for_each(|(f, d)| {
        dependency_map.insert(f.to_string(), d.to_vec());
    });
    dependency_map
}

fn get_directory() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Usage: excali_ts <path>");
    }

    let path = Path::new(&args[1]);
    if !path.exists() {
        panic!("{} does not exist!", args[1]);
    }

    args[1].to_string()
}

fn get_all_js_files(path: String) -> Vec<String> {
    let mut files = Vec::new();
    WalkDir::new(path).into_iter().for_each(|entry| {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "js" || ext == "jsx" || ext == "ts" || ext == "tsx" {
                    files.push(path.display().to_string());
                }
            }
        }
    });
    files
}
