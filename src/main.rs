mod excalidraw;
/**
 * TODO
 * current plan is to run like this:
 *      cargo run [path]
 *  and as output get something like this:
 *      dependency tree of all the js/jsx/ts/tsx files as .excalidraw file
 *  the algorithm should therefore do the following:
 *      1. input validation
 *          -> works well enough for now
 *             could use clap to make it more robust
 *      2. get paths for all js/jsx/ts/tsx files
 *          -> works
 *      3. for each file path get its dependencies
 *          -> works well enough for now
 *            could use fancier data structures to make it more robust
 *          NOTE: can simply use a hashmap (file -> [dependency]) for internal representation
 *      4. create an excalidraw json file showing the dependency graph
 *         -> not implemented yet
 *         probably needs its own module
 *         
 *         create empty excalidraw file
 *         
 *         do something like this:
 *         decide on a layout algorithm
 *           -> probably needs its own module??
 *         calculate positions for each file
 *         for each file in the hashmap
 *            add a box for the file
 *            for each dependency of the file
 *              make a line from the file to the dependency
 *         NOTE: could use a graph library for this
 **/
mod jsops;

use core::panic;
use excalidraw::ExcalidrawDocument;
use jsops::FileNode;
use std::{collections::HashMap, path::Path};
use walkdir::WalkDir;

fn main() {
    let directory = get_directory();
    let files = get_all_js_files(directory);

    let dependency_map = build_dependency_map(files);

    dbg!(dependency_map);


    tmp_excalidraw_playground();
}

fn tmp_excalidraw_playground() {
    let excalidraw_document = ExcalidrawDocument::new();

    excalidraw_document.save("test.excalidraw");
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
