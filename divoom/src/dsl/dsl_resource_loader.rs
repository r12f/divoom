use std::cmp::min;
use crate::dsl::{DivoomDslOperationResource, DivoomDslOperationResourceLoader};
use crate::{DivoomAPIError, DivoomAPIResult};
use std::fs;
use std::mem::swap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use log::{debug, warn};
use rand;
use rand::Rng;

/// No op resource loader.
/// - It doesn't load anything and always return failure when being called.
pub(crate) struct DivoomDslOperationNoOpResourceLoader {}

impl DivoomDslOperationNoOpResourceLoader {
    pub fn new() -> Box<dyn DivoomDslOperationResourceLoader + Send> {
        Box::new(DivoomDslOperationNoOpResourceLoader {})
    }
}

impl DivoomDslOperationResourceLoader for DivoomDslOperationNoOpResourceLoader {
    fn next(&mut self) -> DivoomAPIResult<Arc<DivoomDslOperationResource>> {
        Err(DivoomAPIError::ResourceLoadError {
            source: std::io::Error::new(std::io::ErrorKind::Unsupported, "This operation doesn't support loading any resources."),
        })
    }
}

/// Single file resource loader.
/// - Only load and catch a single file.
/// - Lazy load when first time being called.
pub(crate) struct DivoomDslOperationFileResourceLoader {
    file_path: String,
    file_content: Mutex<Option<Arc<DivoomDslOperationResource>>>,
}

impl DivoomDslOperationFileResourceLoader {
    pub fn new(file_path: &str) -> Box<dyn DivoomDslOperationResourceLoader + Send> {
        Box::new(DivoomDslOperationFileResourceLoader {
            file_path: file_path.to_string(),
            file_content: Mutex::new(None),
        })
    }
}

impl DivoomDslOperationResourceLoader for DivoomDslOperationFileResourceLoader {
    fn next(&mut self) -> DivoomAPIResult<Arc<DivoomDslOperationResource>> {
        let mut file_content = self.file_content.lock().unwrap();

        if file_content.is_none() {
            *file_content = Some(Arc::new(DivoomDslOperationResource::new(
                &self.file_path,
                fs::read(&self.file_path)?,
            )));
        }

        Ok(file_content.as_ref().unwrap().clone())
    }
}

/// Folder resource folder.
/// - Lazy load when first time being called.
/// - Load images in batch, specified by prefetch count.
pub(crate) struct DivoomDslOperationGlobResourceLoader {
    file_pattern: String,
    random: bool,
    prefetch_count: usize,
    file_path_candidates: Mutex<Vec<PathBuf>>,

    // File resources are loaded in reversed order, so we can return them one by one by getting the last one.
    file_resources: Mutex<Vec<Arc<DivoomDslOperationResource>>>,
}

impl DivoomDslOperationGlobResourceLoader {
    pub fn new(file_pattern: String, random: bool, prefetch_count: usize) -> Box<dyn DivoomDslOperationResourceLoader + Send> {
        Box::new(DivoomDslOperationGlobResourceLoader {
            file_pattern,
            random,
            prefetch_count,
            file_path_candidates: Mutex::new(Vec::new()),
            file_resources: Mutex::new(Vec::new()),
        })
    }

    fn load_next_file_content_batch(
        file_pattern: &str,
        random: bool,
        fetch_count: usize,
        file_path_candidates: &mut Vec<PathBuf>,
    ) -> DivoomAPIResult<Vec<Arc<DivoomDslOperationResource>>> {
        debug!("Loading new batch of file paths: Pattern = {}, Random = {}, FetchCount = {}", file_pattern, random, fetch_count);

        // If we don't have any more candidates, we scan the files and load the latest status again.
        if file_path_candidates.is_empty() {
            debug!("File path candidates are running out, rescanning disk.");

            let glob_matches = match glob::glob(file_pattern) {
                Err(e) => return Err(DivoomAPIError::ParameterError(e.to_string())),
                Ok(v) => v,
            };

            for file_entry in glob_matches {
                if let Ok(file_path) = file_entry {
                    file_path_candidates.push(file_path);
                }
            }

            debug!("{} files found with file pattern: {}", file_path_candidates.len(), file_pattern);
        }

        // If we still cannot find any new candidate, it means, we don't have a match, hence return here.
        if file_path_candidates.is_empty() {
            return Ok(Vec::new());
        }

        // If we do, then we select the next batch to load and return them in a list.
        let mut files_to_fetch: Vec<PathBuf>;
        if !random {
            let file_to_fetch_count = min(file_path_candidates.len(), fetch_count as usize);
            files_to_fetch = file_path_candidates.split_off(file_to_fetch_count);
            swap(&mut files_to_fetch, file_path_candidates);
        } else {
            files_to_fetch = Vec::new();

            let mut rng = rand::thread_rng();
            while !file_path_candidates.is_empty() && files_to_fetch.len() < fetch_count {
                let index = rng.gen_range(0..file_path_candidates.len());
                let file_path = file_path_candidates.swap_remove(index);
                files_to_fetch.push(file_path);
            }
        }

        debug!("Selected {} files to load.", files_to_fetch.len());

        let mut file_resources = Vec::new();
        for file_path in files_to_fetch {
            let file_content = match fs::read(&file_path) {
                Err(e) => {
                    warn!("Failed to load file, skip failed and continue loading more: Path = {:?}, Error = {:?}", file_path, e);
                    continue
                },
                Ok(v) => v,
            };

            let file_resource = Arc::new(DivoomDslOperationResource::new(
                file_path.to_str().unwrap(),
                file_content
            ));

            file_resources.push(file_resource);
        }

        // We reverse the loaded file resources, so we can reduce the cost by returning from the last one.
        file_resources.reverse();

        Ok(file_resources)
    }
}

impl DivoomDslOperationResourceLoader for DivoomDslOperationGlobResourceLoader {
    fn next(&mut self) -> DivoomAPIResult<Arc<DivoomDslOperationResource>> {
        let mut guarded_file_resources = self.file_resources.lock().unwrap();
        if guarded_file_resources.is_empty() {
            let mut guarded_file_path_candidates = self.file_path_candidates.lock().unwrap();
            *guarded_file_resources = DivoomDslOperationGlobResourceLoader::load_next_file_content_batch(&self.file_pattern, self.random, self.prefetch_count, &mut guarded_file_path_candidates)?;
        }

        if guarded_file_resources.is_empty() {
            return Err(DivoomAPIError::ResourceLoadError {
                source: std::io::Error::new(std::io::ErrorKind::NotFound, "Unable to load any resources, no files are found."),
            });
        }

        let last_file_index = guarded_file_resources.len() - 1;
        let resource = guarded_file_resources.remove(last_file_index);
        Ok(resource)
    }
}