# Project Plan

## Goal

run like this: `cargo run [path]`
as output get dependency tree of all the js/jsx/ts/tsx files as `.excalidraw` file

## TODOs

the algorithm should do the following:

### input validation

- works well enough for now
- could use `clap` to make it more robust

### get paths for all js/jsx/ts/tsx files

- works

### get dependencies for files

- works well enough for now
- simply uses a hashmap (file -> [dependency]) for internal representation
- could use fancier data structures to make it more robust

### excalidraw drawing module

-> in progress

- [x] create texts
- [x] create arrow between texts
- [x] create multi-point arrow between texts
- [x] save to file
- [ ] (optional) create a box around the texts

### visualize dependency graph

not implemented yet. do something like this:

- decide on a layout algorithm -> probably needs its own module??
- calculate positions for each file
- for each file in the hashmap
  - add a box for the file
  - for each dependency of the file \* make a line from the file to the dependency
    NOTE: could use a graph library for this??

this one is a bit tricky  
might have to represent as DAG and then use sth like https://graphviz.org/
