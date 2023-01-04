pub mod get;
pub mod load;
pub mod vote;

#[cfg(test)]
mod tests {
    use crate::endpoints::rec::prefix::get::Output;
    use crate::entry::Entry;

    use crate::tree::Tree;

    use crate::procedures::get::tree::Get;

    use super::load::tree::Load;
    use super::vote::tree::Vote;

    fn all_entries() -> Vec<Entry> {
        Vec::from([
            Entry::new("a".into(), 5),
            Entry::new("ab".into(), 50),
            Entry::new("aa".into(), 12),
            Entry::new("ace".into(), 33),
            Entry::new("bb".into(), 44),
            Entry::new("abc".into(), 100),
            Entry::new("cb".into(), 12),
            Entry::new("ba".into(), 44),
            Entry::new("acc".into(), 51),
            Entry::new("cb".into(), 12),
            Entry::new("bbc".into(), 5),
            Entry::new("ddd".into(), 6),
        ])
    }

    async fn tree() -> Tree {
        let entries: Vec<Entry> = all_entries();
        let tree: Tree = Tree::new_empty(5).await;

        for entry in entries {
            match tree.include(entry.clone()) {
                Ok(_) => {}
                Err(err) => {
                    panic!("Error including initial entries. Entry: {entry}, error: {err} .")
                }
            };
        }

        tree
    }

    fn get_entry(tree: &Tree, name: &str) -> Entry {
        let top: Vec<Entry> = match tree.get_top(name) {
            Err(err) => panic!("{err}"),
            Ok(value) => match value {
                Output::Values(values) => values,
            },
        };

        top.first()
            .expect("Failed to get entry for that name")
            .clone()
    }

    fn validate_get(tree: &Tree, expected: &Entry) {
        let prefix: &str = expected.get_name();
        let entry: Entry = get_entry(tree, prefix);

        assert_eq!(
            entry, *expected,
            "Comparison failure, expected {expected}, got {entry}"
        );
    }

    #[tokio::test]
    async fn get() {
        let tree: Tree = tree().await;

        let entries: Vec<Entry> = all_entries();

        for expected in &entries {
            validate_get(&tree, expected);
        }
    }

    fn validate_get_all(tree: &Tree, prefix: &str, expected: Vec<Entry>) {
        let top: Vec<Entry> = match tree.get_top(prefix) {
            Err(err) => panic!("{err}"),
            Ok(value) => match value {
                Output::Values(values) => values,
            },
        };
        println!("Entries expected:");
        for i in &expected {
            println!("{i}")
        }
        println!("-----");

        println!("Entries retrieved");
        for i in &top {
            println!("{i}")
        }
        println!("--------");

        for i in 0..top.len() {
            let current: &Entry = &top[i];
            let expected: &Entry = &expected[i];

            assert_eq!(*current, *expected, "Expected {expected}, got {current}");
        }
    }

    #[tokio::test]
    async fn get_all() {
        let tree: Tree = tree().await;

        let expected: Vec<Entry> = Vec::from([
            Entry::new("abc".into(), 100),
            Entry::new("acc".into(), 51),
            Entry::new("ab".into(), 50),
            Entry::new("ba".into(), 44),
            Entry::new("bb".into(), 44),
        ]);

        validate_get_all(&tree, "", expected);

        let expected: Vec<Entry> = Vec::from([
            Entry::new("a".into(), 5),
            Entry::new("abc".into(), 100),
            Entry::new("acc".into(), 51),
            Entry::new("ab".into(), 50),
            Entry::new("ace".into(), 33),
            Entry::new("aa".into(), 12),
        ]);

        validate_get_all(&tree, "a", expected);

        let expected: Vec<Entry> =
            Vec::from([Entry::new("ab".into(), 50), Entry::new("abc".into(), 100)]);

        validate_get_all(&tree, "ab", expected);

        let expected: Vec<Entry> =
            Vec::from([Entry::new("acc".into(), 51), Entry::new("ace".into(), 33)]);

        validate_get_all(&tree, "ac", expected);

        let expected: Vec<Entry> = Vec::from([Entry::new("acc".into(), 51)]);

        validate_get_all(&tree, "acc", expected);
    }

    #[tokio::test]
    async fn get_all_with_different_capitalization() {
        let tree: Tree = tree().await;

        let expected: Vec<Entry> = Vec::from([
            Entry::new("aBC".into(), 100),
            Entry::new("ACC".into(), 51),
            Entry::new("ab".into(), 50),
            Entry::new("Ba".into(), 44),
            Entry::new("BB".into(), 44),
        ]);

        validate_get_all(&tree, "", expected);

        let expected: Vec<Entry> = Vec::from([
            Entry::new("A".into(), 5),
            Entry::new("aBc".into(), 100),
            Entry::new("ACC".into(), 51),
            Entry::new("Ab".into(), 50),
            Entry::new("aCE".into(), 33),
            Entry::new("AA".into(), 12),
        ]);

        validate_get_all(&tree, "A", expected);

        let expected: Vec<Entry> =
            Vec::from([Entry::new("aB".into(), 50), Entry::new("ABc".into(), 100)]);

        validate_get_all(&tree, "aB", expected);

        let expected: Vec<Entry> =
            Vec::from([Entry::new("aCC".into(), 51), Entry::new("ACe".into(), 33)]);

        validate_get_all(&tree, "AC", expected);

        let expected: Vec<Entry> = Vec::from([Entry::new("aCC".into(), 51)]);

        validate_get_all(&tree, "AcC", expected);
    }

    /// Vote in given name 2 times and asserts that it updated properly
    fn validate_vote(tree: &Tree, prefix: &str) {
        let before: Entry = get_entry(tree, prefix);
        let expected: Entry = Entry::new(before.get_name().into(), before.get_times() + 2);

        tree.vote(prefix);
        tree.vote(prefix);

        let after: Entry = get_entry(tree, prefix);

        assert_eq!(
            expected, after,
            "Failed vote. Expected {expected}. Got {after}"
        );
    }

    #[tokio::test]
    async fn vote() {
        let tree: Tree = tree().await;

        for entry in all_entries() {
            validate_vote(&tree, entry.get_name());
        }
    }
}
