use crate::config::parsers;
use crate::config::Gateway;
use crate::error::*;
use crate::service;
use crate::templates::Providers;
use std::sync::Arc;
use std::sync::RwLock;

/// This is the API Gateway container
#[derive(Default)]
pub struct KatalystEngine {
    state: Arc<RwLock<Option<Gateway>>>,
    providers: Providers,
}

impl KatalystEngine {
    /// Update the running configuration of the API Gateway.
    pub fn update_state(&self, new_state: Gateway) -> Result<(), KatalystError> {
        let mut state = self.state.write()?;
        *state = Option::Some(new_state);
        Ok(())
    }

    /// Get a copy of the currently running API Gateway configuration.
    pub fn get_state(&self) -> Result<Gateway, KatalystError> {
        let state = self.state.read()?;
        match state.clone() {
            Some(s) => Ok(s),
            None => Err(KatalystError::StateUnavailable),
        }
    }
}

#[derive(Default)]
pub struct Katalyst {
    engine: Arc<KatalystEngine>,
}

impl Katalyst {
    #[inline]
    pub fn engine(&self) -> Arc<KatalystEngine> {
        self.engine.clone()
    }

    /// Load a configuration file
    pub fn load(&self, config_file: &str) -> Result<(), KatalystError> {
        let mut config = parsers::parse_file(config_file);
        self.engine
            .update_state(config.build(&self.engine.providers))?;
        Ok(())
    }

    /// Start the API Gateway
    #[inline]
    pub fn run(&self) -> Result<(), KatalystError> {
        service::run_service(self.engine.clone())?;
        Ok(())
    }
}
