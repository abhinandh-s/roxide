use walkdir::{DirEntry, WalkDir};

use crate::core::remove_files;
///# wild_card
///
/// recursivly remove files which matches the pattern
/// skips hidden directories
///```rust
///
///```
///
pub fn pattern(path: String, pattern: String) -> anyhow::Result<()> {
    // recursivly walks dirs while skipping hidden dirs
    let walker = WalkDir::new(path).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            //        println!("skipping dirs");
        } else {
            let condition = entry.file_name().to_str().map(|m| m.contains(&pattern.to_string())).unwrap_or(false);
            if condition {
                let item = entry.path().to_path_buf();
                let vec = vec![item];
                remove_files(vec, false, false, false).unwrap();
                println!("removed: {}", entry.path().display());
            } 
        }
    }
    Ok(())
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

#[cfg(test)]
mod test {
    #[test]
    fn glob_wild_card() {
        println!("test");
    }
}
