//! This module defines the configuration settings required for the application,
//! including connection pooling options and TiDB-specific configurations.
//!
//! The `Config` struct serves as the main configuration holder for the application,
//! containing nested configuration for TiDB connections via the `TiDBConfig` struct.
//!
//! This configuration can be serialized and deserialized using the `serde` library,
//! allowing it to be easily loaded from or saved to formats like JSON, TOML, or YAML.
//!
//! The TiDB configuration (`TiDBConfig`) supports features like connection pooling, SSL,
//! and customizable timeouts for optimized performance and resource management.


/// Main configuration for the application.
///
/// The `Config` struct holds the overall configuration needed by the application,
/// including TiDB-specific settings within the `tidb` field.
///
/// # Example (TOML)
/// ```toml
/// [tidb]
/// host = "127.0.0.1"
/// port = 4000
/// username = "admin"
/// password = "secret"
/// databaseName = "mydb"
///
/// [tidb.pool_options]
/// maxConnections = 10
/// minConnections = 5
/// acquireTimeout = 30
/// idleTimeout = 300
/// maxLifetime = 3600
/// isLazy = true
///
/// # Optional: Uncomment to use SSL
/// # ssl_ca = "/path/to/ca-cert.pem"
/// ```
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// TiDB configuration parameters.
    pub tidb: TiDBConfig,
}

/// Configuration settings for connecting to TiDB.
///
/// This struct contains all the necessary fields to establish and manage connections
/// to a TiDB instance. It includes connection information such as host, port, username,
/// and database name, as well as options for managing a connection pool and optional
/// SSL configuration.
///
/// # Example (TOML)
/// ```toml
/// host = "127.0.0.1"
/// port = 4000
/// username = "admin"
/// password = "secret"
/// databaseName = "test_db"
///
/// [pool_options]
/// maxConnections = 10
/// minConnections = 2
/// acquireTimeout = 30
/// idleTimeout = 300
/// maxLifetime = 3600
/// isLazy = true
///
/// # Optional: Uncomment to use SSL
/// # ssl_ca = "/path/to/ca-cert.pem"
/// ```
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TiDBConfig {
    /// Hostname or IP address of the TiDB server.
    pub host: String,

    /// Port number of the TiDB server.
    ///
    /// If not specified, it defaults to 4000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,

    /// Username for authentication to the TiDB server.
    pub username: String,

    /// Password for authentication to the TiDB server.
    pub password: String,

    /// Name of the TiDB database to connect to.
    pub database_name: String,

    /// Connection pooling options for managing connections to TiDB.
    #[serde(rename = "pool_options", default)]
    pub pool_options: PoolOptions,

    /// Path to the SSL CA certificate for encrypted connections.
    ///
    /// Optional: If not specified, SSL will not be used for the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_ca: Option<String>,
}

impl TiDBConfig {
    /// Returns the host and port of the TiDB server as a single string.
    ///
    /// If the port is not specified, the default port (4000) is used.
    ///
    /// # Example
    /// ```
    /// let config = tidb_poll::TiDBConfig {
    ///     host: "127.0.0.1".into(),
    ///     port: None,
    ///     ..Default::default()
    /// };
    /// assert_eq!(config.get_host(), "127.0.0.1:4000");
    /// ```
    pub fn get_host(&self) -> String {
        let port = self.port.unwrap_or(4000);
        format!("{}:{}", self.host, port)
    }
}

/// Connection pooling options for managing TiDB connections.
///
/// These settings control the behavior of the connection pool, including the maximum and minimum
/// number of connections, timeouts for acquiring and idle connections, and whether to use
/// a lazy connection pool.
///
/// A lazy connection pool does not initialize the connections immediately; instead, it waits until
/// a connection is needed.
///
/// # Example (TOML)
/// ```toml
/// maxConnections = 10
/// minConnections = 5
/// acquireTimeout = 30
/// idleTimeout = 300
/// maxLifetime = 3600
/// isLazy = true
/// ```

/// Default value for `max_connections`.
fn default_max_connections() -> u32 {
    10
}

/// Default value for `min_connections`.
fn default_min_connections() -> u32 {
    1
}

/// Default value for `acquire_timeout`.
fn default_acquire_timeout() -> u64 {
    30
}

/// Default value for `idle_timeout`.
fn default_idle_timeout() -> u64 {
    300
}

/// Default value for `max_lifetime`.
fn default_max_lifetime() -> u64 {
    1800
}

/// Default value for `is_lazy`.
fn default_is_lazy() -> bool {
    true
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PoolOptions {
    /// Maximum number of connections that the pool will maintain.
    ///
    /// Defaults to 10.
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,

    /// Minimum number of connections that the pool will maintain.
    ///
    /// Defaults to 1.
    #[serde(default = "default_min_connections")]
    pub min_connections: u32,

    /// Timeout (in seconds) for acquiring a connection from the pool.
    ///
    /// Defaults to 30 seconds.
    #[serde(default = "default_acquire_timeout")]
    pub acquire_timeout: u64,

    /// Timeout (in seconds) after which idle connections will be closed.
    ///
    /// Defaults to 300 seconds (5 minutes).
    #[serde(default = "default_idle_timeout")]
    pub idle_timeout: u64,

    /// Maximum lifetime (in seconds) of a connection in the pool.
    ///
    /// Defaults to 1800 seconds (30 minutes).
    #[serde(default = "default_max_lifetime")]
    pub max_lifetime: u64,

    /// Whether the pool should lazily initialize connections.
    ///
    /// Defaults to `true`.
    #[serde(default = "default_is_lazy")]
    pub is_lazy: bool,
}

impl Default for PoolOptions {
    fn default() -> Self {
        PoolOptions {
            max_connections: default_max_connections(),
            min_connections: default_min_connections(),
            acquire_timeout: default_acquire_timeout(),
            idle_timeout: default_idle_timeout(),
            max_lifetime: default_max_lifetime(),
            is_lazy: default_is_lazy(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml;

    /// Test to verify the default values for `PoolOptions`.
    #[test]
    fn test_default_pool_options() {
        let default_options = PoolOptions::default();

        assert_eq!(default_options.max_connections, 10);
        assert_eq!(default_options.min_connections, 1);
        assert_eq!(default_options.acquire_timeout, 30);
        assert_eq!(default_options.idle_timeout, 300);
        assert_eq!(default_options.max_lifetime, 1800);
        assert!(default_options.is_lazy);
    }

    /// Test to verify deserialization from TOML into `PoolOptions`.
    #[test]
    fn test_deserialize_pool_options_from_toml() {
        let toml_data = r#"
        maxConnections = 10
        minConnections = 3
        acquireTimeout = 15
        idleTimeout = 600
        maxLifetime = 3600
        isLazy = false
        "#;

        let pool_options: PoolOptions =
            toml::from_str(toml_data).expect("Failed to deserialize TOML");

        assert_eq!(pool_options.max_connections, 10);
        assert_eq!(pool_options.min_connections, 3);
        assert_eq!(pool_options.acquire_timeout, 15);
        assert_eq!(pool_options.idle_timeout, 600);
        assert_eq!(pool_options.max_lifetime, 3600);
        assert!(!pool_options.is_lazy);
    }

    /// Test to verify deserialization with missing optional fields.
    #[test]
    fn test_deserialize_pool_options_with_missing_fields() {
        let toml_data = r#"
        maxConnections = 10
        isLazy = true
        "#;

        let pool_options: PoolOptions =
            toml::from_str(toml_data).expect("Failed to deserialize TOML");

        assert_eq!(pool_options.max_connections, 10);
        assert_eq!(pool_options.min_connections, 1);  // Default value
        assert_eq!(pool_options.acquire_timeout, 30); // Default value
        assert_eq!(pool_options.idle_timeout, 300);   // Default value
        assert_eq!(pool_options.max_lifetime, 1800);  // Default value
        assert!(pool_options.is_lazy);
    }

    /// Test to verify serialization into TOML.
    #[test]
    fn test_serialize_pool_options_to_toml() {
        let pool_options = PoolOptions {
            max_connections: 20,
            min_connections: 5,
            acquire_timeout: 60,
            idle_timeout: 1200,
            max_lifetime: 7200,
            is_lazy: false,
        };

        let toml_data = toml::to_string(&pool_options).expect("Failed to serialize to TOML");

        let expected_toml = r#"
maxConnections = 20
minConnections = 5
acquireTimeout = 60
idleTimeout = 1200
maxLifetime = 7200
isLazy = false
"#
            .trim();

        assert_eq!(toml_data.trim(), expected_toml);
    }

    /// Test for the `get_host` method in `TiDBConfig` with a specified port.
    #[test]
    fn test_get_host_with_port() {
        let config = TiDBConfig {
            host: "127.0.0.1".into(),
            port: Some(5000),
            username: "admin".into(),
            password: "secret".into(),
            database_name: "mydb".into(),
            pool_options: PoolOptions::default(),
            ssl_ca: None,
        };

        assert_eq!(config.get_host(), "127.0.0.1:5000");
    }

    /// Test for the `get_host` method when the port is missing (should default to 4000).
    #[test]
    fn test_get_host_without_port() {
        let config = TiDBConfig {
            host: "127.0.0.1".into(),
            port: None,
            username: "admin".into(),
            password: "secret".into(),
            database_name: "mydb".into(),
            pool_options: PoolOptions::default(),
            ssl_ca: None,
        };

        assert_eq!(config.get_host(), "127.0.0.1:4000");
    }

    /// Test to verify deserialization of `TiDBConfig` from TOML.
    #[test]
    fn test_deserialize_tidb_config_from_toml() {
        let toml_data = r#"
        host = "127.0.0.1"
        port = 4000
        username = "admin"
        password = "secret"
        databaseName = "mydb"

        [pool_options]
        maxConnections = 10
        minConnections = 5
        acquireTimeout = 30
        idleTimeout = 300
        maxLifetime = 3600
        isLazy = true
        "#;

        let config: TiDBConfig = toml::from_str(toml_data).expect("Failed to deserialize TOML");

        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, Some(4000));
        assert_eq!(config.username, "admin");
        assert_eq!(config.password, "secret");
        assert_eq!(config.database_name, "mydb");

        assert_eq!(config.pool_options.max_connections, 10);
        assert_eq!(config.pool_options.min_connections, 5);
        assert_eq!(config.pool_options.acquire_timeout, 30);
        assert_eq!(config.pool_options.idle_timeout, 300);
        assert_eq!(config.pool_options.max_lifetime, 3600);
        assert!(config.pool_options.is_lazy);
    }

    /// Test to verify serialization of `TiDBConfig` into TOML.
    #[test]
    fn test_serialize_tidb_config_to_toml() {
        let config = TiDBConfig {
            host: "127.0.0.1".into(),
            port: Some(4000),
            username: "admin".into(),
            password: "secret".into(),
            database_name: "mydb".into(),
            pool_options: PoolOptions {
                max_connections: 10,
                min_connections: 5,
                acquire_timeout: 30,
                idle_timeout: 300,
                max_lifetime: 3600,
                is_lazy: true,
            },
            ssl_ca: None,
        };

        let toml_data = toml::to_string(&config).expect("Failed to serialize to TOML");

        let expected_toml = r#"
host = "127.0.0.1"
port = 4000
username = "admin"
password = "secret"
databaseName = "mydb"

[pool_options]
maxConnections = 10
minConnections = 5
acquireTimeout = 30
idleTimeout = 300
maxLifetime = 3600
isLazy = true
"#
            .trim();

        assert_eq!(toml_data.trim(), expected_toml);
    }

    /// Test to verify deserialization of `TiDBConfig` with missing optional fields.
    #[test]
    fn test_deserialize_tidb_config_with_missing_optional_fields() {
        let toml_data = r#"
        host = "127.0.0.1"
        username = "admin"
        password = "secret"
        databaseName = "mydb"

        [pool_options]
        isLazy = true
        "#;

        let config: TiDBConfig = toml::from_str(toml_data).expect("Failed to deserialize TOML");

        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, None); // No port provided in TOML
        assert_eq!(config.username, "admin");
        assert_eq!(config.password, "secret");
        assert_eq!(config.database_name, "mydb");

        assert_eq!(config.pool_options.max_connections, 10); // Default value
        assert_eq!(config.pool_options.min_connections, 1);  // Default value
        assert_eq!(config.pool_options.acquire_timeout, 30); // Default value
        assert_eq!(config.pool_options.idle_timeout, 300);   // Default value
        assert_eq!(config.pool_options.max_lifetime, 1800);  // Default value
        assert!(config.pool_options.is_lazy);
    }
}
