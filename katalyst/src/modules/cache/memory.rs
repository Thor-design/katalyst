use super::*;
use futures::future::*;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Default, Debug)]
pub struct MemoryCacheBuilder;

impl ModuleProvider for MemoryCacheBuilder {
    fn name(&self) -> &'static str {
        "memory_cache"
    }

    fn build(&self, _: ModuleType, _: Arc<Katalyst>, _: &unstructured::Document) -> Result<Module> {
        Ok(MemoryCache::default().into_module())
    }
}

#[derive(Default, Debug)]
pub struct MemoryCache {
    cache: RwLock<HashMap<String, Arc<CachedObject>>>,
}

impl CacheProviderModule for MemoryCache {
    fn get_key(
        &self,
        key: &str,
    ) -> Box<Future<Item = Arc<CachedObject>, Error = GatewayError> + Send> {
        Box::new(match self.cache.read() {
            Ok(read) => match read.get(key) {
                Some(r) => ok(r.clone()),
                None => err(GatewayError::StateUnavailable),
            },
            Err(_) => err(GatewayError::StateUnavailable),
        })
    }

    fn set_key(
        &self,
        key: &str,
        val: CachedObject,
    ) -> Box<Future<Item = (), Error = GatewayError> + Send> {
        let mut cache = match self.cache.write() {
            Ok(s) => s,
            Err(_) => return Box::new(err(GatewayError::StateUnavailable)),
        };
        cache.insert(key.to_owned(), Arc::new(val));
        Box::new(ok(()))
    }
}
