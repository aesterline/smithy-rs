/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_runtime_api::client::interceptors::InterceptorContext;
use aws_smithy_runtime_api::client::orchestrator::{BoxError, HttpRequest, HttpResponse};
use aws_smithy_runtime_api::config_bag::ConfigBag;
use std::fmt::Debug;
use std::time::Duration;

pub mod rate_limiting;
pub mod strategy;
