/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

package software.amazon.smithy.rustsdk

import software.amazon.smithy.model.shapes.OperationShape
import software.amazon.smithy.rust.codegen.client.smithy.ClientCodegenContext
import software.amazon.smithy.rust.codegen.client.smithy.customize.ClientCodegenDecorator
import software.amazon.smithy.rust.codegen.client.smithy.generators.config.ConfigCustomization
import software.amazon.smithy.rust.codegen.client.smithy.generators.config.ServiceConfig
import software.amazon.smithy.rust.codegen.core.rustlang.Writable
import software.amazon.smithy.rust.codegen.core.rustlang.rust
import software.amazon.smithy.rust.codegen.core.rustlang.rustTemplate
import software.amazon.smithy.rust.codegen.core.rustlang.writable
import software.amazon.smithy.rust.codegen.core.smithy.RuntimeConfig
import software.amazon.smithy.rust.codegen.core.smithy.customize.AdHocCustomization
import software.amazon.smithy.rust.codegen.core.smithy.customize.OperationCustomization
import software.amazon.smithy.rust.codegen.core.smithy.customize.OperationSection
import software.amazon.smithy.rust.codegen.core.smithy.customize.adhocCustomization

class CredentialsCacheDecorator : ClientCodegenDecorator {
    override val name: String = "CredentialsCache"
    override val order: Byte = 0
    override fun configCustomizations(
        codegenContext: ClientCodegenContext,
        baseCustomizations: List<ConfigCustomization>,
    ): List<ConfigCustomization> {
        return baseCustomizations + CredentialCacheConfig(codegenContext.runtimeConfig)
    }

    override fun operationCustomizations(
        codegenContext: ClientCodegenContext,
        operation: OperationShape,
        baseCustomizations: List<OperationCustomization>,
    ): List<OperationCustomization> {
        return baseCustomizations + CredentialsCacheFeature(codegenContext.runtimeConfig)
    }

    override fun extraSections(codegenContext: ClientCodegenContext): List<AdHocCustomization> =
        listOf(
            adhocCustomization<SdkConfigSection.CopySdkConfigToClientConfig> { section ->
                rust("${section.serviceConfigBuilder}.set_credentials_cache(${section.sdkConfig}.credentials_cache().cloned());")
            },
        )
}

/**
 * Add a `.credentials_cache` field and builder to the `Config` for a given service
 */
class CredentialCacheConfig(runtimeConfig: RuntimeConfig) : ConfigCustomization() {
    private val codegenScope = arrayOf(
        "cache" to AwsRuntimeType.awsCredentialTypes(runtimeConfig).resolve("cache"),
        "provider" to AwsRuntimeType.awsCredentialTypes(runtimeConfig).resolve("provider"),
        "DefaultProvider" to defaultProvider(),
    )

    override fun section(section: ServiceConfig) = writable {
        when (section) {
            ServiceConfig.ConfigImpl -> rustTemplate(
                """
                /// Returns the credentials cache.
                pub fn credentials_cache(&self) -> #{cache}::SharedCredentialsCache {
                    self.config_bag.load::<#{cache}::SharedCredentialsCache>().clone()
                }
                """,
                *codegenScope,
            )

            ServiceConfig.BuilderImpl -> {
                rustTemplate(
                    """
                    /// Sets the credentials cache for this service
                    pub fn credentials_cache(mut self, credentials_cache: #{cache}::CredentialsCache) -> Self {
                        self.set_credentials_cache(Some(credentials_cache));
                        self
                    }

                    /// Sets the credentials cache for this service
                    pub fn set_credentials_cache(&mut self, credentials_cache: Option<#{cache}::CredentialsCache>) -> &mut Self {
                        self.config_bag.store_or_unset(credentials_cache);
                        self
                    }
                    """,
                    *codegenScope,
                )
            }

            else -> emptySection
        }
    }
}

class CredentialsCacheFeature(private val runtimeConfig: RuntimeConfig) : OperationCustomization() {
    override fun section(section: OperationSection): Writable {
        return when (section) {
            is OperationSection.MutateRequest -> writable {
                rust(
                    """
                    #T(&mut ${section.request}.properties_mut(), ${section.config}.credentials_cache.clone());
                    """,
                    setCredentialsCache(runtimeConfig),
                )
            }

            else -> emptySection
        }
    }
}

fun setCredentialsCache(runtimeConfig: RuntimeConfig) =
    AwsRuntimeType.awsHttp(runtimeConfig).resolve("auth::set_credentials_cache")
