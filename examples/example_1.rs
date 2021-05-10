use std::fs;
use std::path::Path;
use std::path::PathBuf;
use text_tree_elements::TextTreeElements;

// recursive reading files and directories, print tree
//
// * path - current path
// * ignore - ignore list
// * tree - text_tree_elements structure containing tree branch symbols
// * prefixes - accumulator for previous prefixes
// * level - current depth level of the tree
// * index - current item number
// * size - max items number
fn list_files(
    path: &Path,
    ignore: Option<&Vec<&str>>,
    tree: &TextTreeElements,
    prefixes: &str,
    level: usize,
    index: usize,
    size: usize,
) {
    let (prefix, branch) = tree.get_prefix_branch(level, index, size);

    // skip "." path, doesn't have file name
    if let Some(file_name) = path.file_name() {
        let file_name = file_name.to_str().unwrap();

        // skip if in ignore list
        if let Some(v) = ignore {
            if v.contains(&file_name) {
                return;
            }
        }

        println!("{}{}{}", prefixes, branch, file_name);
    }

    // if dir contains files?
    if let Ok(files) = fs::read_dir(path) {
        let files = files.collect::<Result<Vec<_>, _>>().unwrap();

        let prefixes = format!("{}{}", prefixes, prefix);
        let size = files.len();

        // for all children files
        for (i, file) in files.iter().enumerate() {
            let dir = file.path();
            // recursive call
            list_files(&dir, ignore, tree, &prefixes, level + 1, i, size)
        }
    }
}

fn main() {
    let path = PathBuf::from(".");
    let path_str = path.as_os_str().to_str().unwrap();
    let prefixes = "";
    let ignore = vec![".git", "target"];

    // print root dir
    println!("\n\n{}", path_str);
    // default symbols
    let tree = TextTreeElements::default();
    // 0 - root, 0 - first item of 1 - max items
    list_files(&path, Some(&ignore), &tree, prefixes, 0, 0, 1);

    // print root dir
    println!("\n\n{}", path_str);
    // custom symbols
    let tree = TextTreeElements::new(["", "", "|  ", "   "], ["", "", "|- ", "'- "]);
    // 0 - root, 0 - first item of 1 - max items
    list_files(&path, Some(&ignore), &tree, prefixes, 0, 0, 1);

    // print root dir
    println!("\n\n{}", path_str);
    // custom symbols
    let tree = TextTreeElements::new(["", "", "   ", "   "], ["", "", "   ", "   "]);
    // 0 - root, 0 - first item of 1 - max items
    list_files(&path, Some(&ignore), &tree, prefixes, 0, 0, 1);
}
