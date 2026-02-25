// a helper function to build paths safely from strings rather than just concat

use std::path::Path;
use std::path::PathBuf;

// safe path joining
// combines a base dir, optional sub-folder, and a filename
pub(crate) fn path_builder(
    base: impl AsRef<Path>,
    folder: impl AsRef<Path>,
    sub_folder: impl AsRef<Path>,
    file: &str,
) -> PathBuf {
    let mut path = PathBuf::from(base.as_ref());

    // This works for &str, String, OR your DataDir enum!
    let folder_ref = folder.as_ref();
    let sub_folder_ref = sub_folder.as_ref();
    path.push(folder_ref);
    path.push(sub_folder_ref);

    path.push(file);
    path.set_extension("ron");
    path
}
