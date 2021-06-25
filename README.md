# Text Tree Elements

[![Build Status](https://github.com/AntonGepting/text-tree-elements-rs/actions/workflows/actions.yml/badge.svg)](https://github.com/AntonGepting/text-tree-elements-rs/actions)
[![Crates.io](https://img.shields.io/crates/v/text_tree_elements.svg)](https://crates.io/crates/text_tree_elements)
[![Documentation](https://docs.rs/text_tree_elements/badge.svg)](https://docs.rs/text_tree_elements)


`Text Tree Elements` is a small Rust language library, containing few tiny
functions for preparing tree like structured items for displaying them in TUI
(but not building these trees). For example it can be used for directory
listing output similar to `tree` command.

## Quick Start

1. Initialize text tree elements with default symbols:

    ```
    let tree = TextTreeElements::default();
    ```

2. Get prefix and branch strings for current item of the tree in loop:

    ```
    let (prefix, branch) = tree.get_prefix_branch(level, index, size);
    let prefixes = format!("{}{}", prefixes, prefix);
    ```

    `prefix` - variable contains current item's parent branches (by default one
    of: `["", "", "│  ", "   "]` for: `[root, first, middle, last]` item
    respectively)

    `branch` - variable contains current item branch symbol, depending on
    position (by default one of: `["", "", "├─ ", "└─ "]` for: `[root, first,
    middle, last]` item respectively)

    `prefixes` - variable contains all previous concatenated parents branches


3. Output current tree item in loop:
    ```
    println!("{}{}{}", prefixes, branch, file_name);
    ```

    Output example:
    ```
    root
    ├─ home
    │  └─ user
    ├─ tmp
    └─ ...
    ```

## Examples

In examples directory can be found recursive files list program example with
some variations for demonstration of different output styles.

### Default

Default init (classic branch glyphs):
```
// init branch and prefix arrays:
// `[root, first, middle, last]`, `[root, first, middle, last]`
// `["", "", "├─ ", "└─ "]`, `["", "", "│  ", "   "]`
let tree = TextTreeElements::default();
```

Output:
```
.
├─ TODO.md
├─ README.md
├─ src
│  ├─ lib.rs
│  ├─ text_tree_elements.rs
│  └─ text_tree_elements_tests.rs
├─ Cargo.lock
├─ LICENSE.md
├─ examples
│  └─ example_1.rs
├─ .github
│  └─ workflows
│     └─ actions.yml
├─ .gitignore
└─ Cargo.toml
```

### Custom

Custom init (user given branch glyphs):

```
// init custom branch and prefix arrays:
// `[root, first, middle, last]`, `[root, first, middle, last]`
// `["", "", "|- ", "'- "]`, `["", "", "|  ", ",  "]`
let tree = TextTreeElements::new(["", "", "|  ", "   "], ["", "", "|- ", "'- "]);
```

Output:

```
.
|- TODO.md
|- README.md
|- src
|  |- lib.rs
|  |- text_tree_elements.rs
|  '- text_tree_elements_tests.rs
|- Cargo.lock
|- LICENSE.md
|- examples
|  '- example_1.rs
|- .github
|  '- workflows
|     '- actions.yml
|- .gitignore
'- Cargo.toml
```

## License

`text-tree-elements` library is licensed under the MIT license. Please read the
[license file](LICENSE.md) in the repository for more information.


## See also

- [Rust programming language](https://www.rust-lang.org/)
- [crates.io](https://www.crates.io/)
- [docs.rs](https://www.docs.rs/)
- [rust-clippy](https://github.com/rust-lang/rust-clippy)
