use crate::utils::{AppError, Result};
use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(password: &str) -> Result<String> {
    hash(password, DEFAULT_COST).map_err(AppError::Bcrypt)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    verify(password, hash).map_err(AppError::Bcrypt)
}