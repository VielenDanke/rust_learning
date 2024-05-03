use std::{fs, path};
use std::error::Error;
use std::fs::Permissions;

trait FileMetadata {
    fn exists(&self) -> bool;

    fn is_writeable(&self) -> bool;

    fn is_readable(&self) -> bool;

    fn fetch_permissions(&self) -> Result<Permissions, Box<dyn Error>>;
}

impl FileMetadata for path::Path {
    fn exists(&self) -> bool {
        self.exists()
    }

    fn is_writeable(&self) -> bool {
        if let Ok(perm) = self.fetch_permissions() {
            if perm.readonly() {
                return false;
            }
            return true;
        }
        false
    }

    fn is_readable(&self) -> bool {
        if let Ok(_) = self.fetch_permissions() {
            return true
        }
        false
    }

    fn fetch_permissions(&self) -> Result<Permissions, Box<dyn Error>> {
        Ok(fs::metadata(self)?.permissions())
    }
}

#[test]
fn writeable() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    assert!(f.path().is_writeable());

    fs::remove_file(f.path()).unwrap();
}

#[test]
fn read_only() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    let mut perms = fs::metadata(f.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(f.path(), perms).unwrap();
    assert_eq!(f.path().is_writeable(), false);

    fs::remove_file(f.path()).unwrap();
}
