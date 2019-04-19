use std::any::Any;
use std::sync::Arc;

pub use crate::authentication::*;
pub use crate::context::Context;
pub use crate::error::*;
pub use crate::expression::*;
pub use crate::locator::Locatable;
#[allow(unused_imports)]
pub(crate) use failure::ResultExt;

pub(crate) trait KatalystCommonUtilities {
    fn arc() -> Arc<Self>
    where
        Self: Sized + Default,
    {
        Arc::new(Self::default())
    }

    fn boxed() -> Box<Self>
    where
        Self: Sized + Default,
    {
        Box::new(Self::default())
    }
}

impl<T> KatalystCommonUtilities for T where T: Any {}

pub(crate) trait OptionUtilities<T> {
    fn with(&self, message: &'static str) -> Result<&T, RequestFailure>;
    fn with_owned(self, message: &'static str) -> Result<T, RequestFailure>;
}

impl<T> OptionUtilities<T> for Option<T>
where
    T: Any,
{
    fn with(&self, message: &'static str) -> Result<&T, RequestFailure> {
        match self {
            Some(t) => Ok(t),
            None => Err(RequestFailure::Other(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                message,
            )),
        }
    }

    fn with_owned(self, message: &'static str) -> Result<T, RequestFailure> {
        match self {
            Some(t) => Ok(t),
            None => Err(RequestFailure::Other(
                http::StatusCode::INTERNAL_SERVER_ERROR,
                message,
            )),
        }
    }
}