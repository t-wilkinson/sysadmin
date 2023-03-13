use std::fs;
use sysadmin::norm;
use tempdir::TempDir;

fn create_test_directory() -> TempDir {
    let mut root = TempDir::new();
    root.add_dir("/A");
    root.add_file("/A.txt");
    root
}

#[test]
fn it_normalizes_directories() -> Result<(), std::io::Error> {
    let temp_dir = create_test_directory();

    let normalized_path = norm::normalize_path(temp_dir.path.to_str().unwrap(), true);
    for entry in fs::read_dir(normalized_path)? {
        println!("{:?}", entry?.path());
    }
    Ok(())
}

mod tempdir {
    use rand::Rng;
    use std::boxed::Box;
    use std::{
        fs,
        fs::File,
        path::{Path, PathBuf},
    };
    use sysadmin::norm;

    pub struct TempDir {
        pub path: Box<Path>,
        pub children: Vec<PathBuf>, // Do we actually need to keep track of children?
    }

    impl TempDir {
        pub fn new() -> Self {
            let mut rng = rand::thread_rng();
            let path = format!("/tmp/TMP{:05}", rng.gen::<u16>());
            let _ = fs::create_dir(&path);
            let path = Path::new(&path);
            let path: Box<Path> = Box::from(path);
            TempDir {
                path,
                children: Vec::new(),
            }
        }

        pub fn add_dir(&mut self, relative_path: &str) {
            let full_path = self.path.join(Path::new(&format!(".{}", relative_path)));
            fs::create_dir(&full_path).unwrap();
            self.children.push(full_path);
        }

        pub fn add_file(&mut self, relative_path: &str) {
            let full_path = self.path.join(Path::new(&format!(".{}", relative_path)));
            let _ = File::create(&full_path).unwrap();
            self.children.push(full_path);
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let normalized_path = norm::get_normalized_path(self.path.to_str().unwrap());
            let _ = fs::remove_dir_all(normalized_path);
        }
    }
}
