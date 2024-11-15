use walkdir::{DirEntry, WalkDir};
///# wild_card
///
/// recursivly remove files which matches the pattern
/// skips hidden directories
///```rust
///
///```
///
pub fn wild_card(path: String, pattern: String) -> anyhow::Result<()> {
    // recursivly walks dirs while skipping hidden dirs
    let walker = WalkDir::new(path).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            //        println!("skipping dirs");
        } else {
            let condition = entry.file_name().to_str().map(|m| m.contains(&pattern.to_string())).unwrap_or(false);
            if condition {
                println!("{}", entry.path().display());
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
