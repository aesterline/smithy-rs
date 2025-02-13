/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::client::orchestrator::{
    AuthOptionResolver, AuthOptionResolverParams, BoxError, HttpAuthOption,
};

/// New-type around a `Vec<HttpAuthOption>` that implements `AuthOptionResolver`.
///
/// This is useful for clients that don't require `AuthOptionResolverParams` to resolve auth options.
#[derive(Debug)]
pub struct AuthOptionListResolver {
    auth_options: Vec<HttpAuthOption>,
}

impl AuthOptionListResolver {
    /// Creates a new instance of `AuthOptionListResolver`.
    pub fn new(auth_options: Vec<HttpAuthOption>) -> Self {
        Self { auth_options }
    }
}

impl AuthOptionResolver for AuthOptionListResolver {
    fn resolve_auth_options(
        &self,
        _params: &AuthOptionResolverParams,
    ) -> Result<Vec<HttpAuthOption>, BoxError> {
        Ok(self.auth_options.clone())
    }
}

/// Empty params to be used with [`AuthOptionListResolver`].
#[derive(Debug)]
pub struct AuthOptionListResolverParams;

impl AuthOptionListResolverParams {
    /// Creates new `AuthOptionListResolverParams`.
    pub fn new() -> Self {
        Self
    }
}
