//! 租户 JWT 中间件
//! 校验通过后注入 TenantClaims，并可由下游解析租户库连接

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
};

use crate::auth::{verify_tenant, TenantClaims};
use crate::AppState;

pub struct TenantAuth;

impl<S, B> Transform<S, ServiceRequest> for TenantAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = TenantAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TenantAuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct TenantAuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for TenantAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        Box::pin(async move {
            let state = req
                .app_data::<actix_web::web::Data<AppState>>()
                .ok_or_else(|| ErrorUnauthorized("missing app state"))?;

            let token = extract_bearer(&req)?;
            let claims = verify_tenant(&token, &state.config)
                .map_err(|e| ErrorUnauthorized(e.to_string()))?;

            req.extensions_mut().insert(claims);
            service.call(req).await
        })
    }
}

fn extract_bearer(req: &ServiceRequest) -> Result<String, actix_web::Error> {
    let auth = req
        .headers()
        .get(actix_web::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| ErrorUnauthorized("missing Authorization"))?;
    auth.strip_prefix("Bearer ")
        .map(|s| s.to_string())
        .ok_or_else(|| ErrorUnauthorized("invalid Authorization scheme"))
}

pub fn tenant_claims(req: &actix_web::HttpRequest) -> Option<TenantClaims> {
    req.extensions().get::<TenantClaims>().cloned()
}
