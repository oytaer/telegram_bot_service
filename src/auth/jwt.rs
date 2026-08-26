//! JWT 编解码（三套密钥）

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use crate::auth::claims::{AdminClaims, AgentClaims, TenantClaims};
use crate::config::AppConfig;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("invalid token: {0}")]
    InvalidToken(String),
    #[error("token expired")]
    Expired,
    #[error("wrong role")]
    WrongRole,
    #[error("missing authorization header")]
    MissingHeader,
}

pub fn sign_admin(claims: &AdminClaims, cfg: &AppConfig) -> Result<String, AuthError> {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(cfg.jwt_admin_secret.as_bytes()),
    )
    .map_err(|e| AuthError::InvalidToken(e.to_string()))
}

pub fn verify_admin(token: &str, cfg: &AppConfig) -> Result<AdminClaims, AuthError> {
    let data = decode::<AdminClaims>(
        token,
        &DecodingKey::from_secret(cfg.jwt_admin_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::Expired,
        _ => AuthError::InvalidToken(e.to_string()),
    })?;
    if data.claims.role != "admin" {
        return Err(AuthError::WrongRole);
    }
    Ok(data.claims)
}

pub fn sign_agent(claims: &AgentClaims, cfg: &AppConfig) -> Result<String, AuthError> {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(cfg.jwt_agent_secret.as_bytes()),
    )
    .map_err(|e| AuthError::InvalidToken(e.to_string()))
}

pub fn verify_agent(token: &str, cfg: &AppConfig) -> Result<AgentClaims, AuthError> {
    let data = decode::<AgentClaims>(
        token,
        &DecodingKey::from_secret(cfg.jwt_agent_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::Expired,
        _ => AuthError::InvalidToken(e.to_string()),
    })?;
    if data.claims.role != "agent" {
        return Err(AuthError::WrongRole);
    }
    Ok(data.claims)
}

pub fn sign_tenant(claims: &TenantClaims, cfg: &AppConfig) -> Result<String, AuthError> {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(cfg.jwt_tenant_secret.as_bytes()),
    )
    .map_err(|e| AuthError::InvalidToken(e.to_string()))
}

pub fn verify_tenant(token: &str, cfg: &AppConfig) -> Result<TenantClaims, AuthError> {
    let data = decode::<TenantClaims>(
        token,
        &DecodingKey::from_secret(cfg.jwt_tenant_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::Expired,
        _ => AuthError::InvalidToken(e.to_string()),
    })?;
    if data.claims.role != "tenant" {
        return Err(AuthError::WrongRole);
    }
    Ok(data.claims)
}
