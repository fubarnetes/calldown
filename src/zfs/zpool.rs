use csv;
use std::collections::HashMap;
use std::process::Command;
use std::str::Split;

use zfs::Datasets;

/// Represents a `zpool`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    /// Get the properties of a zpool as a HashMap
    ///
    /// # Example
    ///
    /// ```
    /// # use calldown::zfs::ZPool;
    /// # for zpool in ZPool::all().iter() {
    /// let props = zpool.properties();
    /// println!("{:#?}", props);
    /// # assert!(props.contains_key("version"));
    /// # }
    /// ```
    pub fn properties(&self) -> HashMap<String, String> {
        let output = Command::new("zpool")
            .arg("get")
            .arg("-H")
            .arg("-p")
            .arg("all")
            .arg(&self.name)
            .output()
            .expect("Could not run `zpool get` command");

        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .has_headers(false)
            .from_reader(&output.stdout[..]);

        let mut props: HashMap<String, String> = HashMap::new();

        for record in rdr.records().filter_map(|r| r.ok()) {
            props.insert(record[1].into(), record[2].into());
        }

        props
    }

    /// Return an Iterator over all datasets in this zpool.
    ///
    /// # Example
    /// ```
    /// # use calldown::zfs::ZPool;
    /// # for zpool in ZPool::all().iter() {
    /// for dataset in zpool.datasets().iter() {
    ///     println!("{}", dataset.name.to_str().unwrap());
    /// }
    /// # }
    /// ```
    pub fn datasets(&self) -> Datasets {
        Datasets::new(self.name.clone().into())
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

        let zpool_list = String::from_utf8_lossy(&zpool_list).trim().to_string();

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
