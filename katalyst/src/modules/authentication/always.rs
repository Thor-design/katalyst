use crate::{app::Katalyst, context::*, modules::*};
use futures::future::*;

#[derive(Default, Debug)]
pub struct AlwaysAuthenticator;

impl ModuleProvider for AlwaysAuthenticator {
    fn name(&self) -> &'static str {
        "always"
    }

    fn build(&self, _: ModuleType, _: Arc<Katalyst>, _: &unstructured::Document) -> Result<Module> {
        Ok(AlwaysAuthenticator.into_module())
    }
}

impl AuthenticatorModule for AlwaysAuthenticator {
    fn authenticate(&self, mut ctx: Context) -> ModuleResult {
        let mut result = KatalystAuthenticationInfo::default();
        result.add_claim("KatalystAuthenticator".to_string(), "always".to_string());
        ctx = match ctx.set_authenticated(result) {
            Ok(c) => c,
            Err(e) => return Box::new(err(e)),
        };
        Box::new(ok::<Context, ModuleError>(ctx))
    }
}
