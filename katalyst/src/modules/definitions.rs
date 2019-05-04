use crate::app::Katalyst;
use crate::prelude::*;
use futures::Future;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(PartialEq, Debug)]
pub enum ModuleType {
    Authenticator,
    Authorizer,
    RequestHandler,
    Plugin,
    Cache,
}

pub type ModuleResultSync = Result<Context, ModuleError>;
pub type ModuleResult = Box<Future<Item = Context, Error = ModuleError> + Send>;

pub trait Module: Send + Sync + Debug {
    fn name(&self) -> &'static str;

    fn supported_hooks(&self) -> Vec<ModuleType>;

    fn build_hook(
        &self,
        _: ModuleType,
        _: Arc<Katalyst>,
        _: &unstructured::Document,
    ) -> Result<Arc<ModuleDispatch>, ConfigurationFailure> {
        Err(ConfigurationFailure::InvalidResource)
    }

    fn build_cache(
        &self,
        _: ModuleType,
        _: Arc<Katalyst>,
        _: &unstructured::Document,
    ) -> Result<Arc<CacheProvider>, ConfigurationFailure> {
        Err(ConfigurationFailure::InvalidResource)
    }
}
