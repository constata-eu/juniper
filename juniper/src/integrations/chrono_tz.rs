//! [`Tz`] (timezone) scalar implementation, represented by its [IANA database][1] name.
//!
//! [`Tz`]: chrono_tz::Tz
//! [1]: http://www.iana.org/time-zones

use crate::{graphql_scalar, InputValue, ScalarValue, Value};

/// See [List of tz database time zones][1] `TZ database name` column for
/// available names.
///
/// [1]: https://en.wikipedia.org/wiki/List_of_tz_database_time_zones
#[graphql_scalar(with = tz, parse_token(String))]
pub type TimeZone = chrono_tz::Tz;

mod tz {
    use super::*;

    pub(super) fn to_output<S: ScalarValue>(v: &TimeZone) -> Value<S> {
        Value::scalar(v.name().to_owned())
    }

    pub(super) fn from_input<S: ScalarValue>(v: &InputValue<S>) -> Result<TimeZone, String> {
        v.as_string_value()
            .ok_or_else(|| format!("Expected `String`, found: {}", v))
            .and_then(|s| {
                s.parse::<TimeZone>()
                    .map_err(|e| format!("Failed to parse `Tz`: {}", e))
            })
    }
}

#[cfg(test)]
mod test {
    use super::TimeZone;

    mod from_input_value {
        use super::TimeZone;

        use crate::{graphql_input_value, FromInputValue, InputValue, IntoFieldError};

        fn tz_input_test(raw: &'static str, expected: Result<TimeZone, &str>) {
            let input: InputValue = graphql_input_value!((raw));
            let parsed = FromInputValue::from_input_value(&input);

            assert_eq!(
                parsed.as_ref(),
                expected.map_err(IntoFieldError::into_field_error).as_ref(),
            );
        }

        #[test]
        fn europe_zone() {
            tz_input_test("Europe/London", Ok(chrono_tz::Europe::London));
        }

        #[test]
        fn etc_minus() {
            tz_input_test("Etc/GMT-3", Ok(chrono_tz::Etc::GMTMinus3));
        }

        mod invalid {
            use super::tz_input_test;

            #[test]
            fn forward_slash() {
                tz_input_test(
                    "Abc/Xyz",
                    Err("Failed to parse `Tz`: received invalid timezone"),
                );
            }

            #[test]
            fn number() {
                tz_input_test(
                    "8086",
                    Err("Failed to parse `Tz`: received invalid timezone"),
                );
            }

            #[test]
            fn no_forward_slash() {
                tz_input_test(
                    "AbcXyz",
                    Err("Failed to parse `Tz`: received invalid timezone"),
                );
            }
        }
    }
}
