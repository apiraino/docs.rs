//! Simple module to store files in database.
//!
//! docs.rs supports two ways of storing files: in a postgres database and in an S3 bucket.
//! It does not support storing files on disk because of the sheer number of files:
//! doing so would quickly run into file descriptor limits when running the web server.
//!
//! It's recommended that you use the S3 bucket in production to avoid running out of disk space.
//! However, postgres is still available for testing and backwards compatibility.

use crate::error::Result;
use crate::storage::{CompressionAlgorithms, Storage};

use serde_json::Value;
use std::path::{Path, PathBuf};

/// Store all files in a directory and return [[mimetype, filename]] as Json
///
/// If there is an S3 Client configured, store files into an S3 bucket;
/// otherwise, stores files into the 'files' table of the local database.
///
/// The mimetype is detected using `magic`.
///
/// Note that this function is used for uploading both sources
/// and files generated by rustdoc.
pub fn add_path_into_database<P: AsRef<Path>>(
    storage: &Storage,
    prefix: &str,
    path: P,
) -> Result<(Value, CompressionAlgorithms)> {
    let (file_list, algorithms) = storage.store_all(prefix, path.as_ref())?;
    Ok((
        file_list_to_json(file_list.into_iter().collect())?,
        algorithms,
    ))
}

fn file_list_to_json(file_list: Vec<(PathBuf, String)>) -> Result<Value> {
    let file_list: Vec<_> = file_list
        .into_iter()
        .map(|(path, name)| {
            Value::Array(vec![
                Value::String(name),
                Value::String(path.into_os_string().into_string().unwrap()),
            ])
        })
        .collect();

    Ok(Value::Array(file_list))
}
