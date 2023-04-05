/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::client::retries::{RetryStrategy, ShouldAttempt};
use aws_smithy_runtime_api::client::interceptors::InterceptorContext;
use aws_smithy_runtime_api::client::orchestrator::{BoxError, HttpRequest, HttpResponse};
use aws_smithy_runtime_api::config_bag::ConfigBag;

#[derive(Debug, Clone)]
pub struct NeverRetryStrategy {}

impl NeverRetryStrategy {
    pub fn new() -> Self {
        Self {}
    }
}

impl RetryStrategy for NeverRetryStrategy {
    fn should_attempt_initial_request(
        &self,
        _cfg: &mut ConfigBag,
    ) -> Result<ShouldAttempt, BoxError> {
        Ok(ShouldAttempt::Yes)
    }

    fn should_attempt_retry(
        &self,
        _context: &InterceptorContext<HttpRequest, HttpResponse>,
        _cfg: &mut ConfigBag,
    ) -> Result<ShouldAttempt, BoxError> {
        Ok(ShouldAttempt::No)
    }
}
