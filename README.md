# Text Tree Elements

[![Build Status](https://github.com/AntonGepting/text-tree-elements-rs/actions/workflows/actions.yml/badge.svg)](https://github.com/AntonGepting/text-tree-elements-rs/actions)

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

## Customization

Use custom symbols:
```
let tree = TextTreeElements::new(["", "", "|  ", "   "], ["", "", "|- ", "'- "]);
```

## Examples

Default (classic branch glyphs):

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


Custom (user given branch glyphs):
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
