#[cfg(test)]
mod test {
    use syn::Field;

    #[test]
    fn test_nothing() {
        let field: Field = panic!();

        let reference = &field;

        // Why isn't this of type 'Field'?
        let clone: Field = reference.clone();
    }
}