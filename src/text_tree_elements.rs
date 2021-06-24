/// `""`
pub const EMPTY: &str = "";

/// `"│  "`
pub const I_BRANCH: &str = "│  ";
/// `"   "`
pub const PADDING_SPACES: &str = "   ";

//pub const BRANCH: &str = "── ";
/// `"├─ "`
pub const T_BRANCH: &str = "├─ ";
/// `"└─ "`
pub const L_BRANCH: &str = "└─ ";

// XXX: Cow<> ? using String and str
/// Struct for storing item symbols for root, first, middle, last items
#[derive(Debug)]
pub struct TextTreeSymbols<'a> {
    /// root item
    pub root: &'a str,
    /// first item
    pub first: &'a str,
    /// middle item
    pub middle: &'a str,
    /// last item
    pub last: &'a str,
}

impl<'a> Default for TextTreeSymbols<'a> {
    fn default() -> Self {
        TextTreeSymbols {
            root: EMPTY,
            first: EMPTY,
            middle: EMPTY,
            last: EMPTY,
        }
    }
}

// NOTE: common info
//
// * no need for `│  ` for the last parent element, use `   ` instead
// * nothing need for the first element of the first level
// * branch symbol differs, depending from position in array
//
// ```text
// level:
// [ 0 ][ 1 ][ 2 ]
// ---------------
// [][a0]           - (root,   level: 0, index: 0, size: 1)
// [├─ ][b0]        - (first,  level: 1, index: 0, size: 4)
// [├─ ][b1]        - (middle, level: 1, index: 1, size: 4)
// [├─ ][b2]        - (middle, level: 1, index: 2, size: 4)
// [│  ][├─ ][c0]   - (first,  level: 2, index: 0, size: 5) set parent `│  ` for childeren
// [│  ][├─ ][c1]   - (middle, level: 2, index: 1, size: 5) set parent `│  ` for childeren
// [│  ][├─ ][c2]   - (middle, level: 2, index: 1, size: 5) set parent `│  ` for childeren
// [│  ][├─ ][c3]   - (middle, level: 2, index: 1, size: 5) set parent `│  ` for childeren
// [│  ][└─ ][c4]   - (last,   level: 2, index: 2, size: 5) set parent `│  ` for childeren
// [└─ ][b3]        - (last,   level: 1, index: 2, size: 4)
// [   ][├─ ][d0]   - (first,  level: 2, index: 0, size: 6)  no parent `│  ` for childeren
// [   ][├─ ][d1]   - (middle, level: 2, index: 1, size: 6)  no parent `│  ` for childeren
// [   ][├─ ][d2]   - (middle, level: 2, index: 2, size: 6)  no parent `│  ` for childeren
// [   ][├─ ][d3]   - (middle, level: 2, index: 3, size: 6)  no parent `│  ` for childeren
// [   ][├─ ][d4]   - (middle, level: 2, index: 4, size: 6)  no parent `│  ` for childeren
// [   ][└─ ][d5]   - (last,   level: 2, index: 5, size: 6)  no parent `│  ` for childeren
// ```
#[derive(Debug)]
pub struct TextTreeElements<'a> {
    /// symbols for padding and previous branches
    pub prefix: TextTreeSymbols<'a>,
    /// symbols for padding and branch
    pub branch: TextTreeSymbols<'a>,
}

///
impl<'a> Default for TextTreeElements<'a> {
    fn default() -> Self {
        TextTreeElements {
            // padding and previous branches
            prefix: TextTreeSymbols {
                root: EMPTY,
                first: EMPTY, // currently not used
                middle: I_BRANCH,
                last: PADDING_SPACES,
            },
            // current branch symbol
            branch: TextTreeSymbols {
                root: EMPTY,
                first: EMPTY, // currently not used
                middle: T_BRANCH,
                last: L_BRANCH,
            },
        }
    }
}

impl<'a> TextTreeElements<'a> {
    /// Initialize prefix and branch structures with user given symbols
    ///
    /// `let branch = [root, first, middle, last];` (default: `["", "", "├─ ", "└─ "]`)
    /// `let prefix = [root, first, middle, last];` (default: `["", "", "│  ", "   "]`)
    pub fn new(prefix: [&'a str; 4], branch: [&'a str; 4]) -> Self {
        TextTreeElements {
            prefix: TextTreeSymbols {
                root: prefix[0],
                first: prefix[1],
                middle: prefix[2],
                last: prefix[3],
            },
            branch: TextTreeSymbols {
                root: branch[0],
                first: branch[1],
                middle: branch[2],
                last: branch[3],
            },
        }
    }

    /// Function returns the current item prefix, depending on the
    /// position in the children list, and depth level
    ///
    /// * `prefixes` - all prefixes from previous level
    /// * `level` - current depth level of the tree (`0` - for the root item)
    /// * `index` - current item index (`0` - for the first item)
    /// * `size` - items num (items number on the current level), will be used for detecting of the
    /// last item
    ///
    // `let prefix = [root, first, middle, last];` (default: `["", "", "│  ", "   "]`)
    pub fn get_prefix(&self, level: usize, index: usize, size: usize) -> &'a str {
        if level == 0 {
            self.prefix.root // only root
        } else if index == size - 1 {
            self.prefix.last // last
        } else {
            self.prefix.middle // not last, other
        }
    }

    /// Function returns the current tree item branch symbol, depending on the
    /// position in the children list, and depth level
    ///
    /// * `level` - current depth level of the tree (`0` - for the root item)
    /// * `index` - current item index (`0` - for the first item)
    /// * `size` - items num (items number on the current level), will be used for detecting of the
    /// last item
    ///
    // `let branch = [root, first, middle, last];` (default: `["", "", "├─ ", "└─ "]`)
    pub fn get_branch(&self, level: usize, index: usize, size: usize) -> &'a str {
        if level == 0 {
            self.branch.root // only root
        } else if index == size - 1 {
            self.branch.last // last
        } else {
            self.branch.middle // not last, other
        }
    }

    /// Function returns a tuple containing both a prefix and a branch, depending on the
    /// position in the children list, and depth level
    ///
    /// * `level` - current depth level of the tree (`0` - for the root item)
    /// * `index` - current item index (`0` - for the first item)
    /// * `size` - items num (items number on the current level), will be used for detecting of the
    /// last item
    ///
    pub fn get_prefix_branch(&self, level: usize, index: usize, size: usize) -> (&'a str, &'a str) {
        let prefix = self.get_prefix(level, index, size);
        let branch = self.get_branch(level, index, size);
        (prefix, branch)
    }
}
