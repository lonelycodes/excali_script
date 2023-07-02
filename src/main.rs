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
    let mut document = ExcalidrawDocument::new();
    let mut y_value = 0.0;
    let y_increment = 50.0;

    let mut files_locations: BTreeMap<String, ExcalidrawPoint> = BTreeMap::new();

    dependency_map.iter().for_each(|(k, _)| {
        let point = ExcalidrawPoint::new(0.0, y_value);
        files_locations.insert(k.to_string(), point);
        let element = ExcalidrawElement::new_text(k, 0.0, y_value, k);
        document.add_element(element);
        y_value += y_increment;
    });
    dependency_map.iter().for_each(|(_, v)| {
        v.iter().for_each(|f| {
            if files_locations.get(&f.source).is_none() {
                let point = ExcalidrawPoint::new(0.0, y_value);
                files_locations.insert(f.source.to_string(), point);
                let element = ExcalidrawElement::new_text(&f.source, 0.0, y_value, &f.source);
                document.add_element(element);
                y_value += y_increment;
            };
        })
    });

    let mut size_multiplier = 1.0;
    dependency_map.iter().for_each(|(k, v)| {
        let start_point = files_locations.get(k).unwrap();
        v.iter().for_each(|f| {
            let end_point = files_locations.get(&f.source).unwrap();
            let arrow = build_dependency_arrow(start_point, end_point, size_multiplier);
            document.add_element(arrow);
            size_multiplier += 0.25;
        });
    });

    document.save("out.excalidraw");
}

fn build_dependency_arrow(
    a: &ExcalidrawPoint,
    b: &ExcalidrawPoint,
    size_multiplier: f64,
) -> ExcalidrawElement {
    if a.y <= b.y {
        ExcalidrawElement::new_arrow(
            vec![
                ExcalidrawPoint::new(a.x, a.y),
                ExcalidrawPoint::new(a.x - 50.0 * size_multiplier, a.y),
                ExcalidrawPoint::new(b.x - 50.0 * size_multiplier, b.y),
                ExcalidrawPoint::new(b.x, b.y),
            ],
            a.x,
            a.y,
        )
    } else {
        ExcalidrawElement::new_arrow(
            vec![
                ExcalidrawPoint::new(a.x, a.y),
                ExcalidrawPoint::new(a.x - 50.0 * size_multiplier, a.y),
                ExcalidrawPoint::new(b.x - 50.0 * size_multiplier, b.y),
                ExcalidrawPoint::new(b.x, b.y),
            ],
            0.0,
            0.0,
        )
    }
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
