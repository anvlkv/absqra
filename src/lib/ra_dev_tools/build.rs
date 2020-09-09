extern crate build_deps;

fn main() {
    // Enumerate files in sub-folder "examples/*", being relevant for the test-generation (as example)
    // If function returns with error, exit with error message.
    build_deps::rerun_if_changed_paths( "../../../examples/*/*" ).unwrap();

    // Adding the parent directory "examples" to the watch-list will capture new-files being added
    build_deps::rerun_if_changed_paths( "../../../examples/*" ).unwrap();
}