use std::time::Duration;

use sqlx::{
    mysql::{MySqlConnectOptions, MySqlPoolOptions, MySqlSslMode},
    ConnectOptions, Error, MySqlPool,
};

use crate::config::TiDBConfig;

/// Creates a connection pool to TiDB using the provided configuration.
///
/// This function builds a connection pool based on the settings in the `TiDBConfig`.
/// It allows for customization of various connection pool options, such as the
/// maximum and minimum number of connections, timeouts, and SSL settings.
///
/// The pool can be set to "lazy" mode, meaning it will not establish connections
/// until they are needed, or it can connect immediately depending on the
/// `is_lazy` setting in the configuration.
///
/// ## Parameters:
/// - `config`: A `TiDBConfig` instance containing the connection and pool settings.
///
/// ## Returns:
/// - `Result<MySqlPool, Error>`: Returns the constructed `MySqlPool` or an error if
///   the pool cannot be created.
///
/// ## Example:
/// ```rust,ignore
/// let config = TiDBConfig { ... }; // Your configuration here
/// let pool = build_pool_from_config(config).await?;
/// ```
#[tracing::instrument(name = "tidb_svc", err, skip(config))]
pub async fn build_pool_from_config(config: TiDBConfig) -> Result<MySqlPool, Error> {
    info!("Initializing connection pool to TiDB...");

    // Log the database host for debugging purposes
    info!("Database host: {}", config.get_host());

    // Define the port, defaulting to 4000 if not provided
    let port = config.port.unwrap_or(4000_u16);

    // Build the connection options
    let mut conn_options = MySqlConnectOptions::new()
        .host(config.host.as_str())
        .port(port)
        .database(config.database_name.as_str())
        .username(config.username.as_str())
        .password(config.password.as_str())
        .statement_cache_capacity(1000); // Optimize by caching SQL statements

    // If SSL is enabled (ssl_ca is set), configure SSL options
    if let Some(file_name) = config.ssl_ca {
        conn_options = conn_options
            .ssl_mode(MySqlSslMode::VerifyCa)
            .ssl_ca(file_name);
    }

    // Configure logging options for SQL statements (for debugging)
    let conn_options = conn_options
        .log_statements(tracing::log::LevelFilter::Debug) // Log SQL statements at debug level
        .log_slow_statements(tracing::log::LevelFilter::Off, Duration::default()); // No slow query logging

    // Build the pool options from the configuration, setting various timeouts and connection limits
    let pool_options: MySqlPoolOptions = MySqlPoolOptions::new()
        .max_connections(config.pool_options.max_connections) // Maximum number of connections
        .min_connections(config.pool_options.min_connections) // Minimum number of connections
        .idle_timeout(Duration::from_secs(config.pool_options.idle_timeout)) // Time to wait before closing idle connections
        .max_lifetime(Duration::from_secs(config.pool_options.max_lifetime)) // Maximum lifetime of a connection
        .acquire_timeout(Duration::from_secs(config.pool_options.acquire_timeout)); // Timeout for acquiring a new connection

    // Log the pool settings for debugging
    log_pool_settings(&pool_options);

    // Conditionally initialize the connection pool (lazy or immediate)
    let pool_db: MySqlPool = if config.pool_options.is_lazy {
        // Lazy connection pool: Connections are created only when needed
        Ok(pool_options.connect_lazy_with(conn_options.clone()))
    } else {
        // Immediate connection pool: Establish connections right away
        pool_options.connect_with(conn_options.clone()).await
    }
        .map_err(|err| {
            // Handle connection errors and log the failure
            error!(
            "Failed to connect to TiDB server at {}:{}",
            config.host, port
        );
            err
        })?;

    // Successfully initialized the pool
    info!("TiDB connection pool initialized successfully. Lazy mode: {}", config.pool_options.is_lazy);
    Ok(pool_db)
}

/// Logs the settings of the connection pool for debugging purposes.
///
/// This function logs the important settings of the `MySqlPoolOptions` such as
/// the maximum and minimum number of connections, timeouts, etc.
///
/// ## Parameters:
/// - `pool_options`: The `MySqlPoolOptions` instance containing the pool settings.
fn log_pool_settings(pool_options: &MySqlPoolOptions) {
    info!("Connection pool settings:");
    info!("  Max connections: {}", pool_options.get_max_connections());
    info!("  Min connections: {}", pool_options.get_min_connections());
    info!("  Acquire timeout: {:?}", pool_options.get_acquire_timeout());
    info!("  Idle timeout: {:?}", pool_options.get_idle_timeout());
    info!("  Max lifetime: {:?}", pool_options.get_max_lifetime());
}