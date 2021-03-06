use std::io;
use std::path::{Path, PathBuf};

use regex;
use rustc_serialize::json;
use scan_dir;
use git2;

use path_filter;


quick_error! {
    /// Versioning error
    #[derive(Debug)]
    pub enum Error {
        /// Hash sum can't be calculated because some files need to be
        /// generated during build
        New {
            description("dependencies are not ready")
        }
        /// Some error occured. Unfortunately all legacy errors are strings
        String(s: String) {
            from()
            description("error versioning dependencies")
            display("version error: {}", s)
        }
        Regex(e: regex::Error) {
            from()
            description("can't compile regex")
            display("regex compilation error: {}", e)
        }
        PathFilter(errors: Vec<path_filter::FilterError>) {
            from()
            description("can't read directory")
            display("error reading directory: {:?}", errors)
        }
        ScanDir(errors: Vec<scan_dir::Error>) {
            from()
            description("can't read directory")
            display("error reading directory: {:?}", errors)
        }
        /// I/O error
        Io(err: io::Error, path: PathBuf) {
            cause(err)
            description("io error")
            display("Error reading {:?}: {}", path, err)
            context(p: &'a Path, err: io::Error) -> (err, p.to_path_buf())
            context(p: &'a PathBuf, err: io::Error) -> (err, p.to_path_buf())
        }
        /// Container needed for build is not found
        ContainerNotFound(name: String) {
            description("container not found")
            display("container {:?} not found", name)
        }
        /// Some step of subcontainer failed
        SubStepError(step: String, err: Box<Error>) {
            from(tuple: (String, Error)) -> (tuple.0, Box::new(tuple.1))
        }
        /// Error reading package.json
        Json(err: json::BuilderError, path: PathBuf) {
            description("can't read json")
            display("error reading json {:?}: {:?}", path, err)
            context(p: &'a PathBuf, err: json::BuilderError)
                -> (err, p.to_path_buf())
        }
        GitError(err: git2::Error) {
            from()
            display("{}", err)
        }
    }
}
