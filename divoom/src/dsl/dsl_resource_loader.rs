use crate::dsl::{DivoomDslOperationResource, DivoomDslOperationResourceLoader};
use crate::{DivoomAPIError, DivoomAPIResult};
use std::fs;
use std::sync::{Arc, Mutex};

pub(crate) struct DivoomDslOperationNoOpResourceLoader {}

impl DivoomDslOperationNoOpResourceLoader {
    pub fn new() -> Box<dyn DivoomDslOperationResourceLoader + Send> {
        Box::new(DivoomDslOperationNoOpResourceLoader {})
    }
}

impl DivoomDslOperationResourceLoader for DivoomDslOperationNoOpResourceLoader {
    fn next(&mut self) -> DivoomAPIResult<Arc<DivoomDslOperationResource>> {
        Err(DivoomAPIError::ResourceLoadError {
            source: std::io::Error::from(std::io::ErrorKind::Unsupported),
        })
    }
}

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
