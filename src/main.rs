/**
 * TODO
 * current plan is to run like this:
 *      cargo run [path]
 *  and as output get something like this:
 *      dependency tree of all the js/jsx/ts/tsx files as .excalidraw file
 *  the algorithm should therefore do the following:
 *      1. input validation
 *      2. get paths for all js/jsx/ts/tsx file
 *      3. for each file path get its dependencies
 *         NOTE: can simply use a hashmap (path -> [dependency_path]) for internal representation
 *      4. create an excalidraw json file showing the dependency graph
 **/
mod module;
use walkdir::WalkDir;

fn main() {
    foobar();

    let file_name = get_file_name().expect("No file found.");

    let visited = Vec::new();
    let root = module::parse(&file_name);
    let dependencies = module::get_dependencies(&root);
    dbg!(dependencies.clone());

    let unseen: Vec<&module::FileNode> = dependencies
        .iter()
        .filter(|d| visited.contains(d))
        .collect();

    let x: Vec<&module::FileNode> = dependencies
        .iter()
        .filter(|d| !visited.contains(d))
        .collect();
    dbg!(x);

    let new_deps: Vec<module::FileNode> = dependencies
        .iter()
        .filter(|d| !visited.contains(d))
        .map(|d| module::parse(&d.source))
        .map(|m| module::get_dependencies(&m))
        .flatten()
        .collect();

    let new_dependencies = [dependencies, new_deps].concat();

    //let visited = [visited, unseen].concat();

    new_dependencies
        .iter()
        .for_each(|d| print!("{}, {}, {}", d.name, d.source, d.dependencies.len()));
}

fn foobar() {
    for e in WalkDir::new("samples") {
        println!("{}", e.unwrap().path().display())
    }
}

fn get_file_name() -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: excali_ts <file>");
        return None;
    }

    let file_name = args[1].to_owned();
    if !file_name.ends_with(".js")
        && !file_name.ends_with(".jsx")
        && !file_name.ends_with(".ts")
        && !file_name.ends_with(".tsx")
    {
        println!("File must be a .js, .jsx, .ts, or .tsx file");
        return None;
    }
    Some(file_name)
}
