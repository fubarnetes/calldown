use std::process::Command;
use std::str::Split;

/// Represents a `zpool`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZPool {
    pub name: String,
}

impl ZPool {
    /// Return an iterable list of ZPools.
    ///
    /// `IntoIter` is implemented for &'a ZPools, and ZPools implements an
    /// `iter()` method to allow for iteration.
    ///
    /// # Examples
    /// ```
    /// use calldown::zfs::ZPool;
    /// for zpool in &ZPool::all() {
    ///     println!("{}", zpool.name);
    /// }
    /// ```
    ///
    /// ```
    /// use calldown::zfs::ZPool;
    /// for zpool in ZPool::all().iter() {
    ///     println!("{}", zpool.name);
    /// }
    /// ```
    pub fn all() -> ZPools {
        ZPools::default()
    }
}

pub struct ZPools {
    zpool_list: String,
}

impl Default for ZPools {
    fn default() -> ZPools {
        let zpool_list = Command::new("zpool")
            .arg("list")
            .arg("-H")
            .arg("-o")
            .arg("name")
            .output()
            .expect("failed to run zpool command")
            .stdout;

        let zpool_list = String::from_utf8_lossy(&zpool_list).to_string();

        ZPools { zpool_list }
    }
}

impl ZPools {
    pub fn iter(&self) -> ZPoolIter {
        self.into_iter()
    }
}

impl<'a> IntoIterator for &'a ZPools {
    type Item = ZPool;
    type IntoIter = ZPoolIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ZPoolIter(self.zpool_list.split("\n"))
    }
}

pub struct ZPoolIter<'a>(Split<'a, &'a str>);

impl<'a> Iterator for ZPoolIter<'a> {
    type Item = ZPool;

    fn next(&mut self) -> Option<ZPool> {
        self.0.next().map(|it| ZPool {
            name: it.to_string(),
        })
    }
}
