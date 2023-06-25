mod excalidraw;
mod jsops;

use core::panic;
use excalidraw::{ExcalidrawDocument, ExcalidrawElement, ExcalidrawPoint};
use jsops::FileNode;
use std::{collections::BTreeMap, path::Path};
use walkdir::WalkDir;

fn main() {
    let directory = get_directory();
    let files = get_all_js_files(directory.clone());

    let dependency_map = build_dependency_map(directory.as_str(), files);

    tmp_excalidraw_playground(dependency_map);
}

fn tmp_excalidraw_playground(dependency_map: BTreeMap<String, Vec<FileNode>>) {
    dbg!(dependency_map.clone());
    let mut document = ExcalidrawDocument::new();

    let mut y_value = 0.0;
    let y_increment = 50.0;

    // let arrow_x_offset = -10.0;
    let arrow_y_offset = 10.0;

    dependency_map.iter().for_each(|(k, _)| {
        let element = ExcalidrawElement::new_text(k, 0.0, y_value, k);
        y_value += y_increment;
        document.add_element(element);
    });

    // add arrow from file to dependency
    y_value = -0.0;

    dependency_map.iter().for_each(|(_k, v)| {
        // let mut x_value = 0.0;
        // let x_increment = 50.0;

        v.iter().for_each(|_d| {
            let arrow = build_dependency_arrow(
                0.0, // x_value + arrow_x_offset,
                y_value + arrow_y_offset,
                50.0,
            );
            document.add_element(arrow);
            // x_value += x_increment;
        });

        y_value += y_increment;
    });

    document.save("out.excalidraw");
}

fn build_dependency_arrow(x: f64, y: f64, length: f64) -> ExcalidrawElement {
    ExcalidrawElement::new_arrow(
        vec![
            ExcalidrawPoint::new(0.0, 0.0),
            ExcalidrawPoint::new(-50.0, 0.0),
            ExcalidrawPoint::new(-50.0, length),
            ExcalidrawPoint::new(0.0, length),
        ],
        x,
        y,
    )
}

fn build_dependency_map(dir: &str, files: Vec<String>) -> BTreeMap<String, Vec<FileNode>> {
    let mut dependency_map: BTreeMap<String, Vec<FileNode>> = BTreeMap::new();

    let dependencies = files
        .iter()
        .map(|f| jsops::parse(f))
        .map(|m| jsops::get_dependencies(dir, &m))
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
    path.canonicalize().unwrap().display().to_string()
}

fn get_all_js_files(path: String) -> Vec<String> {
    let mut files = Vec::new();
    WalkDir::new(path).into_iter().for_each(|entry| {
        let entry = entry.unwrap();
        let current_path = entry.path().canonicalize().unwrap();
        if current_path.is_file() {
            if let Some(ext) = current_path.extension() {
                if ext == "js" || ext == "jsx" || ext == "ts" || ext == "tsx" {
                    files.push(current_path.canonicalize().unwrap().display().to_string());
                }
            }
        }
    });
    files
}
