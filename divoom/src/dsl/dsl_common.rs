use crate::dsl::dsl_syntax::DivoomDeviceCommand;
use crate::DivoomAPIResult;
use std::sync::{Arc, Mutex};

pub struct DivoomDslOperation {
    pub command: DivoomDeviceCommand,
    pub resource_loader: Arc<Mutex<Box<dyn DivoomDslOperationResourceLoader + Send>>>,
}

impl DivoomDslOperation {
    pub fn new(
        command: DivoomDeviceCommand,
        resource_loader: Box<dyn DivoomDslOperationResourceLoader + Send>,
    ) -> Self {
        DivoomDslOperation {
            command,
            resource_loader: Arc::new(Mutex::new(resource_loader)),
        }
    }
}

pub struct DivoomDslOperationResource {
    pub name: String,
    pub data: Vec<u8>,
}

impl DivoomDslOperationResource {
    pub(crate) fn new(name: &str, data: Vec<u8>) -> Self {
        DivoomDslOperationResource {
            name: name.to_string(),
            data,
        }
    }
}

pub trait DivoomDslOperationResourceLoader {
    fn next(&mut self) -> DivoomAPIResult<Arc<DivoomDslOperationResource>>;
}
