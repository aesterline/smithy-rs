/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

/// Smithy identity used by auth and signing.
pub mod identity;

/// Smithy interceptors for smithy clients.
///
/// Interceptors are lifecycle hooks that can read/modify requests and responses.
pub mod interceptors;

pub mod orchestrator;

/// Runtime plugin type definitions.
pub mod runtime_plugin;

/// Smithy endpoint resolution runtime plugins
pub mod endpoints;

/// Smithy auth runtime plugins
pub mod auth;

/// Smithy retry traits and common behavior
mod retries;
