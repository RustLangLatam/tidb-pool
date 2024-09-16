# `tidb-pool`

`tidb-pool` is a Rust crate that simplifies the creation of a MySQL connection pool for TiDB using a configuration file in TOML format. It provides a flexible, efficient, and configurable way to connect to TiDB, leveraging connection pooling for optimal performance in both lazy and immediate connection modes. This crate is built on top of the popular [`sqlx`](https://docs.rs/sqlx/) library for managing database connections in an asynchronous environment.

## Features

- **TOML-based Configuration**: Easily configure the connection options via a `.toml` file.
- **MySQL Connection Pool**: Creates a connection pool for TiDB using the MySQL protocol.
- **SSL Support**: Optional SSL support for secure database connections.
- **Lazy vs Immediate Connections**: Choose between lazy connection initialization or establishing connections immediately.
- **Customizable Pooling Options**: Fine-tune connection pooling with options like max/min connections, timeouts, and more.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
tidb-pool = "0.1.0"
```

Then, include the crate in your project:

```rust
use tidb_pool::build_pool_from_config;
```

## Usage

This crate is designed to work with a TOML configuration file that specifies the connection details and pool options. Here's how you can use it to create a connection pool for TiDB:

### 1. Create a TOML Configuration File

Create a file `config.toml` with the necessary configuration settings for connecting to your TiDB database:

```toml
[tidb]
host = "127.0.0.1"
port = 4000
username = "root"
password = "secret"
databaseName = "test_db"

[tidb.pool_options]
maxConnections = 10
minConnections = 5
acquireTimeout = 30
idleTimeout = 300
maxLifetime = 3600
isLazy = true

# Optional: Uncomment to use SSL
# ssl_ca = "/path/to/ca-cert.pem"
```

### 2. Load the Configuration and Create the Pool

Now, you can load the configuration and build the connection pool using `tidb-pool`:

```rust
use tidb_pool::build_pool_from_config;
use sqlx::MySqlPool;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Load the configuration from the TOML file
    let config_content = fs::read_to_string("config.toml").expect("Failed to read config file");
    let config: TiDBConfig = toml::from_str(&config_content).expect("Invalid TOML configuration");

    // Build the connection pool
    let pool: MySqlPool = build_pool_from_config(config).await?;
    
    // Use the pool (e.g., execute queries)

    Ok(())
}
```

### 3. Configuration Fields

Here are the available fields in the TOML configuration:

- **TiDB Section**:
    - `host`: Hostname or IP address of the TiDB server.
    - `port`: Port number for the TiDB server (defaults to 4000).
    - `username`: Username for authentication.
    - `password`: Password for authentication.
    - `databaseName`: Name of the TiDB database to connect to.
    - `ssl_ca`: (Optional) Path to the CA certificate for SSL verification.

- **Pool Options Section**:
    - `maxConnections`: Maximum number of connections in the pool.
    - `minConnections`: Minimum number of connections maintained in the pool.
    - `acquireTimeout`: Timeout (in seconds) for acquiring a connection from the pool.
    - `idleTimeout`: Timeout (in seconds) for closing idle connections.
    - `maxLifetime`: Maximum lifetime (in seconds) for connections in the pool.
    - `isLazy`: Whether to lazily initialize connections (`true`) or establish them immediately (`false`).

## Lazy vs Immediate Connections

The `isLazy` field in the configuration controls whether connections are established lazily or immediately:

- **Lazy Connections (`isLazy = true`)**: Connections are only created when they are actually requested.
- **Immediate Connections (`isLazy = false`)**: Connections are established as soon as the pool is created.

### Example TOML Configuration

```toml
[tidb]
host = "127.0.0.1"
port = 4000
username = "admin"
password = "mypassword"
databaseName = "example_db"

[tidb.pool_options]
maxConnections = 15
minConnections = 5
acquireTimeout = 30
idleTimeout = 300
maxLifetime = 1800
isLazy = false

# Optional: Uncomment if you use SSL
# ssl_ca = "/path/to/ca-cert.pem"
```

## Example

```rust
use tidb_pool::{build_pool_from_config, TiDBConfig};
use std::fs;
use sqlx::MySqlPool;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Load the TiDB configuration from a TOML file
    let config_content = fs::read_to_string("config.toml").expect("Failed to read config file");
    let config: TiDBConfig = toml::from_str(&config_content).expect("Invalid TOML format");

    // Build the connection pool using the configuration
    let pool: MySqlPool = build_pool_from_config(config).await?;

    // Now you can use `pool` to execute queries, transactions, etc.
    
    Ok(())
}
```

## Error Handling

The `build_pool_from_config` function returns a `Result<MySqlPool, sqlx::Error>`. If there is an error in creating the pool, it logs the issue and returns the error, allowing the caller to handle it gracefully.

## License

This project is licensed under the [MIT License](LICENSE).