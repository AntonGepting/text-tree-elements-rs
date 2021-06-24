#[cfg(test)]
mod tests {
    #[test]
    fn get_link() {
        use crate::TextTreeElements;

        let text_tree = TextTreeElements::default();

        // root, second item
        // level: 0, item: 1, items: 1
        let branch = text_tree.get_branch(0, 1, 1);
        assert_eq!(branch, "");

        // child, middle item
        // level: 1, item: 1, items: 3
        let branch = text_tree.get_branch(1, 1, 3);
        assert_eq!(branch, "├─ ");

        // child, first and last item, (bc. only one item)
        // level: 1, item: 0, items: 1
        let branch = text_tree.get_branch(1, 0, 1);
        assert_eq!(branch, "└─ ");
    }

    #[test]
    fn get_prefix() {
        use crate::TextTreeElements;

        let text_tree = TextTreeElements::default();

        // root, second item
        // level: 0, item: 1, items: 1
        let prefix = text_tree.get_prefix(0, 1, 1);
        assert_eq!(prefix, "");

        // child, middle item
        // level: 1, item: 1, items: 3
        let prefix = text_tree.get_prefix(1, 1, 3);
        assert_eq!(prefix, "│  ");

        // child, first and last item, (bc. only one item)
        // level: 1, item: 0, items: 1
        let prefix = text_tree.get_prefix(1, 0, 1);
        assert_eq!(prefix, "   ");
    }

    #[test]
    fn get_prefix_branch() {
        use crate::TextTreeElements;

        let text_tree = TextTreeElements::default();

        // root, second item
        // level: 0, item: 1, items: 1
        let result = text_tree.get_prefix_branch(0, 1, 1);
        assert_eq!(result, ("", ""));

        // child, middle item
        // level: 1, item: 1, items: 3
        let result = text_tree.get_prefix_branch(1, 1, 3);
        assert_eq!(result, ("│  ", "├─ "));

        // child, first and last item, (bc. only one item)
        // level: 1, item: 0, items: 1
        let result = text_tree.get_prefix_branch(1, 0, 1);
        assert_eq!(result, ("   ", "└─ "));
    }
}
