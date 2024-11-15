#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use dirs::data_dir;
    use rid::core::remove_files;
    use rid::utils::trash_dir;


    #[test]
    fn github_test() {
        if !data_dir().expect("Cant get data dir").join("rid").exists() {
            panic!("data dir exists {}", data_dir().unwrap().join("rid").display());
        }
        if trash_dir().exists() {
            let v0 = PathBuf::from("temp_file_for_single_file_text01.txt");
            let v1 = PathBuf::from("temp_file_for_single_file_text02.txt");
            let v2 = PathBuf::from("temp_file_for_single_file_text03.txt");
            let v3 = PathBuf::from("temp_file_for_single_file_text04.txt");
            let single_files = vec![v0, v1, v2, v3];
            for i in &single_files {
                fs::write(i, "some contents for the files").expect("Cant create files");
                assert!(fs::exists(i).expect("Can't check existence of file tmp_file.txt"));
            if !i.exists() {
                    panic!("{} doesnt exists", i.display());
                }
            }
            remove_files(single_files, false, true).expect("Test failed");
            // panic!("trash dir exists! {}", trash_dir().display());
        } else {
            panic!("trash dir doesn't exists");
        }


        // fs::write("file_for_github.txt", "some contents for the files").expect("Cant create files for github");
        // fs::create_dir_all("some/dir/for/testing").expect("Cant create files for github");

        // remove_files("file_for_github.txt", false, true);
        // remove_files("some", true, false);
    }
}
