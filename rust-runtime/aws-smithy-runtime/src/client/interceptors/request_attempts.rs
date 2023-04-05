/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_runtime_api::client::interceptors::{
    Interceptor, InterceptorContext, InterceptorError,
};
use aws_smithy_runtime_api::client::orchestrator::{HttpRequest, HttpResponse};
use aws_smithy_runtime_api::config_bag::ConfigBag;

#[derive(Debug, Clone, Copy)]
pub struct RequestAttempts {
    attempts: u32,
}

impl RequestAttempts {
    pub fn attempts(&self) -> u32 {
        self.attempts
    }

    fn increment(&mut self) {
        self.attempts += 1;
    }
}

#[derive(Debug, Default)]
pub struct RequestAttemptsInterceptor {}

impl RequestAttemptsInterceptor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Interceptor<HttpRequest, HttpResponse> for RequestAttemptsInterceptor {
    fn read_before_attempt(
        &self,
        _ctx: &InterceptorContext<HttpRequest, HttpResponse>,
        cfg: &mut ConfigBag,
    ) -> Result<(), InterceptorError> {
        let mut request_attempts: RequestAttempts = cfg
            .get()
            .cloned()
            .unwrap_or_else(|| RequestAttempts { attempts: 0 });
        request_attempts.increment();
        cfg.put(request_attempts);

        Ok(())
    }
}
