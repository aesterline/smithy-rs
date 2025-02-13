/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use http::header::ToStrError;
use http::{HeaderMap, HeaderValue};

const X_AMZN_QUERY_ERROR: &str = "x-amzn-query-error";
const QUERY_COMPATIBLE_ERRORCODE_DELIMITER: char = ';';

fn aws_query_compatible_error_from_header(
    headers: &HeaderMap<HeaderValue>,
) -> Result<Option<&str>, ToStrError> {
    headers
        .get(X_AMZN_QUERY_ERROR)
        .map(|v| v.to_str())
        .transpose()
}

/// Obtains custom error code and error type from the given `headers`.
///
/// Looks up a value for the `X_AMZN_QUERY_ERROR` header and if found, the value should be in the
/// form of `<error code>;<error type>`. The function then splits it into two parts and returns
/// a (error code, error type) as a tuple.
///
/// Any execution path besides the above happy path will yield a `None`.
pub fn parse_aws_query_compatible_error(headers: &HeaderMap<HeaderValue>) -> Option<(&str, &str)> {
    let header_value = match aws_query_compatible_error_from_header(headers) {
        Ok(error) => error?,
        _ => return None,
    };

    header_value
        .find(QUERY_COMPATIBLE_ERRORCODE_DELIMITER)
        .map(|idx| (&header_value[..idx], &header_value[idx + 1..]))
}

#[cfg(test)]
mod test {
    use crate::aws_query_compatible_errors::{
        aws_query_compatible_error_from_header, parse_aws_query_compatible_error,
        X_AMZN_QUERY_ERROR,
    };

    #[test]
    fn aws_query_compatible_error_from_header_should_provide_value_for_custom_header() {
        let mut response: http::Response<()> = http::Response::default();
        response.headers_mut().insert(
            X_AMZN_QUERY_ERROR,
            http::HeaderValue::from_static("AWS.SimpleQueueService.NonExistentQueue;Sender"),
        );

        let actual = aws_query_compatible_error_from_header(response.headers()).unwrap();

        assert_eq!(
            Some("AWS.SimpleQueueService.NonExistentQueue;Sender"),
            actual,
        );
    }

    #[test]
    fn parse_aws_query_compatible_error_should_parse_code_and_type_fields() {
        let mut response: http::Response<()> = http::Response::default();
        response.headers_mut().insert(
            X_AMZN_QUERY_ERROR,
            http::HeaderValue::from_static("AWS.SimpleQueueService.NonExistentQueue;Sender"),
        );

        let actual = parse_aws_query_compatible_error(response.headers());

        assert_eq!(
            Some(("AWS.SimpleQueueService.NonExistentQueue", "Sender")),
            actual,
        );
    }

    #[test]
    fn parse_aws_query_compatible_error_should_return_none_when_header_value_has_no_delimiter() {
        let mut response: http::Response<()> = http::Response::default();
        response.headers_mut().insert(
            X_AMZN_QUERY_ERROR,
            http::HeaderValue::from_static("AWS.SimpleQueueService.NonExistentQueue"),
        );

        let actual = parse_aws_query_compatible_error(response.headers());

        assert_eq!(None, actual);
    }

    #[test]
    fn parse_aws_query_compatible_error_should_return_none_when_there_is_no_target_header() {
        let mut response: http::Response<()> = http::Response::default();
        response.headers_mut().insert(
            "x-amzn-requestid",
            http::HeaderValue::from_static("a918fbf2-457a-4fe1-99ba-5685ce220fc1"),
        );

        let actual = parse_aws_query_compatible_error(response.headers());

        assert_eq!(None, actual);
    }
}
