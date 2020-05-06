#[cfg(test)]
mod test {
    use syn::Field;

    #[test]
    fn test_nothing() {
        let field: Field = panic!();

        let reference = &field;

        // This works...
        let clone: Field = *reference.clone();

        let v = vec![reference];

        v
            .iter()
            // Why doesn't this works?
            .map(|f| *f.clone())
            .collect::<Vec<Field>>();
    }
}