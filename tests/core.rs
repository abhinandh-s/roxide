#[cfg(test)]
mod tests {
    use std::fs::remove_dir_all;
    use std::path::{Path, PathBuf};
    use std::{fs, path};

    use rid::core::remove_files;

    #[test]
    fn glob_test() {
        // FIX: need fix, not an good test
        let files_for_glob_test = vec![
            PathBuf::from("glob_test_file_001.txt"),
            PathBuf::from("glob_test_file_002.txt"),
            PathBuf::from("glob_test_file_003.txt"),
            PathBuf::from("glob_test_file_004.txt"),
            PathBuf::from("glob_test_file_005.txt"),
            PathBuf::from("glob_test_file_006.txt"),
            PathBuf::from("glob_test_file_007.txt"),
            PathBuf::from("glob_test_file_008.txt"),
        ];
        for i in &files_for_glob_test {
            println!("{}", i.display());
            fs::write(i, "some contents for the files").expect("Cant create files");
            assert!(fs::exists(i).expect("Can't check existence of file glob_test_file_00x.txt"));
        }
        remove_files(files_for_glob_test.clone(), false, true).unwrap();
        for i in files_for_glob_test {
            assert!(!fs::exists(i).expect("Can't check existence of file tmp_file.txt"));
        }
    }

    #[test]
    fn single_file_test() {
        let v0 = PathBuf::from("temp_file_for_single_file_text01.txt");
        let v1 = PathBuf::from("temp_file_for_single_file_text02.txt");
        let v2 = PathBuf::from("temp_file_for_single_file_text03.txt");
        let v3 = PathBuf::from("temp_file_for_single_file_text04.txt");
        let single_files = vec![v0, v1, v2, v3];
        for i in &single_files {
            fs::write(i, "some contents for the files").expect("Cant create files");
            assert!(fs::exists(i).expect("Can't check existence of file tmp_file.txt"));
        }
        remove_files(single_files.clone(), false, true).unwrap();
        for i in &single_files {
            assert!(!fs::exists(i).expect("Can't check existence of file tmp_file.txt"));
        }
    }
    #[test]
    fn single_hidden_file_test() {
        let v0 = PathBuf::from(".tmp_hidden_file_for_single_file_text01.txt");
        let v1 = PathBuf::from(".tmp_hidden_file_for_single_file_text02.txt");
        let v2 = PathBuf::from(".tmp_hidden_file_for_single_file_text03.txt");
        let v3 = PathBuf::from(".tmp_hidden_file_for_single_file_text04.txt");
        let single_files = vec![v0, v1, v2, v3];
        for i in &single_files {
            fs::write(i, "some contents for the files").expect("Cant create files");
            assert!(fs::exists(i).expect("Can't check existence of file tmp_file.txt"));
            pretty_assertions::assert_eq!(1, 1);
        }
        remove_files(single_files.clone(), false, true).unwrap();
        for i in &single_files {
            assert!(!fs::exists(i).expect("Can't check existence of file tmp_file.txt"));
        }
    }
    #[test]
    fn remove_file_from_dir_test() {
        let s = Path::new("some_other").exists();
        if s {
            remove_dir_all("some_other").unwrap();
        }
        fs::create_dir_all("some_other/dir").unwrap();
        fs::write("some_other/dir/test.txt", "some contents for testing").unwrap();
        let v3 = PathBuf::from("some_other/dir/test.txt");
        let single_files = vec![v3];
        remove_files(single_files, false, true).expect("Err with my function");
        assert!(!fs::exists("some_other/dir/test.txt")
            .expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
        remove_files(
            vec![path::Path::new("some_other").to_path_buf()],
            true,
            true,
        )
        .expect("Err with my function");
        assert!(!fs::exists("some_other")
            .expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
    }
    // TEST: done
    #[test]
    fn remove_file_from_hidden_dir_test() {
        let s = Path::new(".some_hidden").exists();
        if s {
            remove_dir_all(".some_hidden").expect("Failed to remove existing .some_hidden dir");
        } else {
            // creates a hidden dir and a file
            fs::create_dir_all(".some_hidden/dir").expect("Failed to create .some_hidden/dir for remove_file_from_hidden_dir_test test");
            fs::write(".some_hidden/dir/test.txt", "some contents for testing").unwrap();
            let v3 = PathBuf::from(".some_hidden/dir/test.txt");
            let a_single_file_from_hidden_dir = vec![v3];
            remove_files(a_single_file_from_hidden_dir, false, true).expect("Err with my function remove_file_from_hidden_dir_test");
            assert!(!fs::exists(".some_hidden/dir/test.txt").expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
            remove_files(
                vec![path::Path::new(".some_hidden").to_path_buf()],
                true,
                true,
            )
                .expect("Err with my function remove_files in remove_file_from_hidden_dir_test");
            assert!(!fs::exists(".some_hidden")
                .expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
        }
    }
    #[test]
    fn recursive_remove_test() {
        let s = Path::new("some").exists();
        if s {
            remove_dir_all("some").unwrap();
        }
        fs::create_dir_all("some/dir/for/testing").unwrap();
        let test_dir = vec![PathBuf::from("some")];
        fs::write("some/test.txt", "some contents for testing").unwrap();
        remove_files(test_dir, true, true).expect("Err with my function");
        assert!(!fs::exists("some")
            .expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
        assert!(!fs::exists("some/dir/for")
            .expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
        assert!(!fs::exists("some/dir/for/testing")
            .expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
    }
    #[test]
    fn recursive_hidden_dir_remove_test() {
        let s = Path::new(".some").exists();
        if s {
            remove_dir_all(".some").unwrap();
        }
        fs::create_dir_all(".some/dir/for/testing").unwrap();
        let test_dir = vec![PathBuf::from(".some")];
        fs::write(".some/test.txt", "some contents for testing").unwrap();
        remove_files(test_dir, true, true).expect("Err with my function");
        assert!(!fs::exists(".some")
            .expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
        assert!(!fs::exists(".some/dir/for")
            .expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
        assert!(!fs::exists(".some/dir/for/testing")
            .expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
    }
}
