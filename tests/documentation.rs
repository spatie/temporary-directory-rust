//! Executes the Markdown examples verbatim, with application context outside the snippets.
macro_rules! documentation_example {
    ($test:ident, ($directory:ident, $name:ident, $location:ident), { $($code:tt)* }) => {
        #[test]
        #[allow(unused_imports, unused_variables)]
        fn $test() -> Result<(), spatie_temporary_directory::Error> {
            let parent = tempfile::tempdir()?;
            let $location = parent.path();
            let unique_name = format!("{}-example", parent.path().file_name().unwrap().to_string_lossy());
            let $name = unique_name.as_str();
            let $directory = spatie_temporary_directory::TemporaryDirectory::new()
                .location(parent.path())
                .delete_when_destroyed(true)
                .create()?;

            $($code)*

            $directory.delete()?;
            assert!(!$directory.exists());
            Ok(())
        }
    };
}

include!(concat!(env!("OUT_DIR"), "/documentation.rs"));
