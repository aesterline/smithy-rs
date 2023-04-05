/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

mod adaptive;
mod never;
mod standard;

pub use adaptive::AdaptiveRetryStrategy;
use aws_smithy_types::retry::ErrorKind;
pub use never::NeverRetryStrategy;
pub use standard::StandardRetryStrategy;
use std::time::Duration;

const TRANSIENT_ERROR_STATUS_CODES: &[u16] = &[500, 502, 503, 504];

#[non_exhaustive]
#[derive(Eq, PartialEq, Debug)]
enum RetryReason {
    Error(ErrorKind),
    Explicit(Duration),
}

trait ErrorIsRetryable {
    fn is_retryable(&self) -> bool;
}
