use std::path::PathBuf;
use std::process::Command;
use std::str::Split;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]

/// Represents a ZFS dataset.
pub struct Dataset {
    /// The name of the dataset, including the pool name.
    pub name: PathBuf,
}

impl Dataset {
    /// Return the list of all datasets.
    ///
    /// # Examples
    ///
    /// ```
    /// use calldown::zfs::Dataset;
    /// for dataset in &Dataset::all() {
    ///     println!("{}", dataset.name());
    /// }
    /// ```
    ///
    /// ```
    /// use calldown::zfs::Dataset;
    /// for dataset in Dataset::all().iter() {
    ///     println!("{}", dataset.name());
    /// }
    /// ```
    pub fn all() -> Datasets {
        Datasets::default()
    }

    /// Get the name of a dataset.
    pub fn name(&self) -> String {
        self.name.to_string_lossy().to_string()
    }
}

#[derive(Debug)]
/// Allows iteration over datasets.
pub struct Datasets {
    dataset_list: String,
}

/// A list of Datasets below a given `root`
impl Datasets {
    pub fn new(root: PathBuf) -> Datasets {
        let dataset_list = Command::new("zfs")
            .arg("list")
            .arg("-H")
            .arg("-r")
            .arg("-o")
            .arg("name")
            .arg(root.to_str().expect("invalid dataset name"))
            .output()
            .expect("failed to run zpool command")
            .stdout;

        let dataset_list = String::from_utf8_lossy(&dataset_list).trim().to_string();

        Datasets { dataset_list }
    }

    /// Return an Iterator over the Datasets list
    pub fn iter(&self) -> DatasetIter {
        self.into_iter()
    }
}

impl Default for Datasets {
    fn default() -> Datasets {
        Datasets::new(PathBuf::default())
    }
}

impl<'a> IntoIterator for &'a Datasets {
    type Item = Dataset;
    type IntoIter = DatasetIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DatasetIter(self.dataset_list.split("\n"))
    }
}

pub struct DatasetIter<'a>(Split<'a, &'a str>);

impl<'a> Iterator for DatasetIter<'a> {
    type Item = Dataset;

    fn next(&mut self) -> Option<Dataset> {
        self.0.next().map(|it| Dataset {
            name: it.into(),
        })
    }
}
