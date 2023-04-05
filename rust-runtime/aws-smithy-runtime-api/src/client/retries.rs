/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::client::interceptors::InterceptorContext;
use crate::client::orchestrator::{BoxError, HttpRequest, HttpResponse};
use crate::config_bag::ConfigBag;
use std::fmt::Debug;
use std::time::Duration;

/// An answer to the question "should I make a request attempt?"
pub enum ShouldAttempt {
    Yes,
    No,
    YesAfterDelay(Duration),
}

pub trait RetryStrategy: Send + Sync + Debug {
    fn should_attempt_initial_request(
        &self,
        cfg: &mut ConfigBag,
    ) -> Result<ShouldAttempt, BoxError>;

    fn should_attempt_retry(
        &self,
        context: &InterceptorContext<HttpRequest, HttpResponse>,
        cfg: &mut ConfigBag,
    ) -> Result<ShouldAttempt, BoxError>;
}
