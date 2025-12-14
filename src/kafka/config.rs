use serde::{Deserialize, Serialize};

use crate::app::state::{AuthConfig, ConnectionProfile, SaslMechanism};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaConfig {
    pub brokers: String,

    #[serde(default)]
    pub consumer_group: Option<String>,

    #[serde(default)]
    pub security: SecurityConfig,

    #[serde(default = "default_connection_timeout")]
    pub connection_timeout_ms: u32,

    #[serde(default = "default_request_timeout")]
    pub request_timeout_ms: u32,
}

fn default_connection_timeout() -> u32 {
    10000 // 10 seconds
}
fn default_request_timeout() -> u32 {
    15000 // 15 seconds
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SecurityConfig {
    #[default]
    None,

    SaslPlain {
        username: String,
        password: String,
    },

    SaslScram256 {
        username: String,
        password: String,
    },

    SaslScram512 {
        username: String,
        password: String,
    },

    Ssl {
        ca_location: Option<String>,
        cert_location: Option<String>,
        key_location: Option<String>,
        key_password: Option<String>,
    },

    SaslSsl {
        mechanism: KafkaSaslMechanism,
        username: String,
        password: String,
        ca_location: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum KafkaSaslMechanism {
    #[default]
    Plain,
    ScramSha256,
    ScramSha512,
}

impl From<ConnectionProfile> for KafkaConfig {
    fn from(profile: ConnectionProfile) -> Self {
        let security = match profile.auth {
            AuthConfig::None => SecurityConfig::None,
            AuthConfig::SaslPlain { username, password } => {
                SecurityConfig::SaslPlain { username, password }
            }
            AuthConfig::SaslScram256 { username, password } => {
                SecurityConfig::SaslScram256 { username, password }
            }
            AuthConfig::SaslScram512 { username, password } => {
                SecurityConfig::SaslScram512 { username, password }
            }
            AuthConfig::Ssl {
                ca_location,
                cert_location,
                key_location,
                key_password,
            } => SecurityConfig::Ssl {
                ca_location,
                cert_location,
                key_location,
                key_password,
            },
            AuthConfig::SaslSsl {
                mechanism,
                username,
                password,
                ca_location,
            } => {
                let mech = match mechanism {
                    SaslMechanism::Plain => KafkaSaslMechanism::Plain,
                    SaslMechanism::ScramSha256 => KafkaSaslMechanism::ScramSha256,
                    SaslMechanism::ScramSha512 => KafkaSaslMechanism::ScramSha512,
                };
                SecurityConfig::SaslSsl {
                    mechanism: mech,
                    username,
                    password,
                    ca_location,
                }
            }
        };

        KafkaConfig {
            brokers: profile.brokers,
            consumer_group: profile.consumer_group,
            security,
            connection_timeout_ms: 30000,
            request_timeout_ms: 60000,
        }
    }
}
