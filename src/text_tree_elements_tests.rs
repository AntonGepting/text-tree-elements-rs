#[cfg(test)]
mod tests {
    #[test]
    fn get_link() {
        use crate::TextTreeElements;

        let text_tree = TextTreeElements::default();

        let branch = text_tree.get_branch(0, 1, 1);
        assert_eq!(branch, "");
        let branch = text_tree.get_branch(1, 1, 3);
        assert_eq!(branch, "├─ ");
        let branch = text_tree.get_branch(1, 0, 1);
        assert_eq!(branch, "└─ ");
    }

    #[test]
    fn get_prefix() {
        use crate::TextTreeElements;

        let text_tree = TextTreeElements::default();

        let prefix = text_tree.get_prefix(0, 1, 1);
        assert_eq!(prefix, "");
        let prefix = text_tree.get_prefix(1, 1, 3);
        assert_eq!(prefix, "│  ");
        let prefix = text_tree.get_prefix(1, 0, 1);
        assert_eq!(prefix, "   ");
    }

    #[test]
    fn get_prefix_branch() {
        use crate::TextTreeElements;

        let text_tree = TextTreeElements::default();

        let result = text_tree.get_prefix_branch(0, 1, 1);
        assert_eq!(result, ("", ""));
        let result = text_tree.get_prefix_branch(1, 1, 3);
        assert_eq!(result, ("│  ", "├─ "));
        let result = text_tree.get_prefix_branch(1, 0, 1);
        assert_eq!(result, ("   ", "└─ "));
    }
}
