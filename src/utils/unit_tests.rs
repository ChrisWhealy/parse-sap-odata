use std::path::Path;

use super::copy_src_tree;

#[test]
fn should_copy_single_file() {
    match copy_src_tree(Path::new("./test_data/association_set.xml"), Path::new("./test_output")) {
        Ok(_copied_bytes) => assert!(true),
        Err(err) => assert!(1 == 2, "{}", format!("{:#?}", err)),
    }
}

#[test]
fn should_copy_recursive_dir_structure() {
    match copy_src_tree(Path::new("./test_data/test_tree/"), Path::new("./test_output/test_tree/")) {
        Ok(_copied_bytes) => assert!(true),
        Err(err) => assert!(1 == 2, "{}", format!("{:#?}", err)),
    }
}
