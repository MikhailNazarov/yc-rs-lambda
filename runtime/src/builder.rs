use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{runtime::YcRuntime, ComponentRef, RuntimeResult};

pub struct RuntimeBuilder {
    pub(crate) components: Vec<ComponentRef>,
}

impl RuntimeBuilder {
    fn new() -> Self {
        Self {
            components: Self::default_components(),
        }
    }

    fn default_components() -> Vec<ComponentRef> {
        vec![]
    }

    pub(crate) fn add_component(mut self, component: ComponentRef) -> Self {
        self.components.push(component);
        self
    }
}

pub fn builder() -> RuntimeBuilder {
    RuntimeBuilder::new()
}

impl RuntimeBuilder {
    pub fn build(self) -> RuntimeResult<YcRuntime> {
        Ok(YcRuntime {
            components: Arc::new(Mutex::new(self.components)),
        })
    }
}
