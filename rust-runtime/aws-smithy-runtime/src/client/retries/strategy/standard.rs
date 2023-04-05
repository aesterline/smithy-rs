/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::client::retries::strategy::RetryReason;
use crate::client::retries::{RetryStrategy, ShouldAttempt};
use aws_smithy_http::result::SdkError;
use aws_smithy_runtime_api::client::interceptors::InterceptorContext;
use aws_smithy_runtime_api::client::orchestrator::{BoxError, HttpRequest, HttpResponse};
use aws_smithy_runtime_api::config_bag::ConfigBag;
use aws_smithy_types::retry::ErrorKind;
use std::time::Duration;

const DEFAULT_MAX_ATTEMPTS: u32 = 3;

#[derive(Debug)]
pub struct StandardRetryStrategy {
    max_attempts: u32,
    initial_backoff: Duration,
    max_backoff: Duration,
    base: fn() -> f64,
}

impl StandardRetryStrategy {
    pub fn with_base(mut self, base: fn() -> f64) -> Self {
        self.base = base;
        self
    }

    pub fn with_max_attempts(mut self, max_attempts: u32) -> Self {
        self.max_attempts = max_attempts;
        self
    }

    pub fn with_initial_backoff(mut self, initial_backoff: Duration) -> Self {
        self.initial_backoff = initial_backoff;
        self
    }
}

impl Default for StandardRetryStrategy {
    fn default() -> Self {
        Self {
            max_attempts: DEFAULT_MAX_ATTEMPTS,
            max_backoff: Duration::from_secs(20),
            // by default, use a random base for exponential backoff
            base: fastrand::f64,
            initial_backoff: Duration::from_secs(1),
        }
    }
}

impl RetryStrategy for StandardRetryStrategy {
    // TODO(token-bucket) add support for optional cross-request token bucket
    fn should_attempt_initial_request(
        &self,
        _cfg: &mut ConfigBag,
    ) -> Result<ShouldAttempt, BoxError> {
        Ok(ShouldAttempt::Yes)
    }

    fn should_attempt_retry(
        &self,
        ctx: &InterceptorContext<HttpRequest, HttpResponse>,
        cfg: &mut ConfigBag,
    ) -> Result<ShouldAttempt, BoxError> {
        // Look a the result. If it's OK then we're done; No retry required. Otherwise, we need to inspect it
        let output_or_error = ctx
            .output_or_error()
            .expect("this method is only called after a request attempt");
        let error = match output_or_error {
            Ok(_) => {
                tracing::trace!("request succeeded, no retry necessary");
                return Ok(ShouldAttempt::No);
            }
            Err(err) => err,
        };

        let request_attempts: &RequestAttempts = cfg
            .get()
            .expect("at least one request attempt is made before any retry is attempted");
        if request_attempts.attempts() == self.max_attempts {
            tracing::trace!(
                attempts = request_attempts.attempts(),
                max_attempts = self.max_attempts,
                "not retrying because we are out of attempts"
            );
            return Ok(ShouldAttempt::No);
        }

        let error = error
            .downcast_ref::<SdkError<HttpResponse>>()
            .expect("error will always be an SdkError");
        let retry_reason = match error {
            SdkError::TimeoutError(_err) => Some(RetryReason::Error(ErrorKind::TransientError)),
            SdkError::DispatchFailure(err) => {
                if err.is_timeout() || err.is_io() {
                    Some(RetryReason::Error(ErrorKind::TransientError))
                } else if let Some(ek) = err.is_other() {
                    Some(RetryReason::Error(ek))
                } else {
                    None
                }
            }
            SdkError::ResponseError { .. } => Some(RetryReason::Error(ErrorKind::TransientError)),
            SdkError::ConstructionFailure(_) => None,
            SdkError::ServiceError(context) => {
                // use crate::client::retries::strategy::TRANSIENT_ERROR_STATUS_CODES;

                // let err = context.err();
                // let response = context.raw();

                todo!("implement `ProvideErrorKind` for `HttpResponse`")
                // if let Some(kind) = err.retryable_error_kind() {
                //     Some(RetryReason::Error(kind))
                // } else if TRANSIENT_ERROR_STATUS_CODES.contains(&response.http().status().as_u16()) {
                //     Some(RetryReason::Error(ErrorKind::TransientError))
                // } else {
                //     None
                // }
            }
            _ => unreachable!("all error variants covered"),
        };

        let backoff = match retry_reason {
            Some(RetryReason::Explicit(dur)) => dur,
            Some(RetryReason::Error(_)) => {
                let backoff = calculate_exponential_backoff(
                    // Generate a random base multiplier to create jitter
                    (self.base)(),
                    // Get the backoff time multiplier in seconds (with fractional seconds)
                    self.initial_backoff.as_secs_f64(),
                    // `self.local.attempts` tracks number of requests made including the initial request
                    // The initial attempt shouldn't count towards backoff calculations so we subtract it
                    request_attempts.attempts() - 1,
                );
                Duration::from_secs_f64(backoff).min(self.max_backoff)
            }
            None => {
                tracing::trace!(
                    attempts = request_attempts.attempts(),
                    max_attempts = self.max_attempts,
                    "encountered unretryable error"
                );
                return Ok(ShouldAttempt::No);
            }
        };

        tracing::debug!(
            "attempt {} failed with {:?}; retrying after {:?}",
            request_attempts.attempts(),
            retry_reason,
            backoff
        );

        Ok(ShouldAttempt::YesAfterDelay(backoff))
    }
}

fn calculate_exponential_backoff(base: f64, initial_backoff: f64, retry_attempts: u32) -> f64 {
    base * initial_backoff * 2_u32.pow(retry_attempts) as f64
}
