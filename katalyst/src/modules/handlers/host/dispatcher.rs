use super::*;
use crate::context::*;
use futures::future::*;
use futures::Future;

impl HostDispatcher {
    pub fn prepare(&self, mut ctx: Context) -> ModuleResultSync {
        let config = try_req!(
            ctx,
            ctx.engine
                .get_instance()
                .map_err(|_| RequestFailure::Internal)
        );

        let balancer_lease = match config.hosts.get(&self.host) {
            Some(s) => try_req!(ctx, s.servers.lease().map_err(|_| RequestFailure::Internal)),
            None => {
                return Err(ctx.fail(RequestFailure::NotFound));
            }
        };

        let transformer = try_req!(ctx, self.transformer(&ctx, balancer_lease.to_string()));
        ctx.detail.balancer_lease = Some(balancer_lease);

        let request = ctx.request.take();
        ctx.request = RequestContainer::Empty;

        let mut client_req = try_req!(ctx, transformer.transform(request));
        ctx.request = RequestContainer::Empty;
        add_forwarding_headers(&mut client_req.headers_mut(), &ctx.detail.remote_ip);
        strip_hop_headers(&mut client_req.headers_mut());
        ctx.request = RequestContainer::new(client_req);
        Ok(ctx)
    }

    pub fn send(mut ctx: Context) -> ModuleResult {
        let dsr = ctx.request.take();
        ctx.request = RequestContainer::Empty;
        let client = ctx.engine.get_client();
        let res = client.request(dsr);
        Box::new(res.then(|response| match response {
            Ok(r) => {
                ctx.response = ResponseContainer::new(r);
                ok(ctx)
            }
            Err(e) => {
                warn!("Could not send upstream request! Caused by: {:?}", e);
                err(ctx.fail(RequestFailure::GatewayTimeout))
            }
        }))
    }

    pub fn clean_response(mut ctx: Context) -> Context {
        if let ResponseContainer::Raw { data } = &mut ctx.response {
            strip_hop_headers(data.headers_mut());
        }
        ctx
    }
}
