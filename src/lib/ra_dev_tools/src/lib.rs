pub extern crate insta;
pub use files_iterator::for_each_ra_example_file;

#[allow(unused_attributes)]
#[macro_use] pub use example_files_macro::make_example_tests;

#[cfg(test)]
mod test {
    use super::make_example_tests;

    #[make_example_tests]
    #[test]
    fn it_works_with(content: String, file_name: String) {
        println!("didn't panic with content: \n[{}] of file_name: {}", content, file_name);
    }
}