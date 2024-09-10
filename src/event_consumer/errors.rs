use deadpool_postgres::{BuildError, ConfigError, CreatePoolError, HookError, PoolError};
use derive_more::{Display, Error, From};

#[derive(Debug, Display, Error, From)]
pub enum DBError {
    NotFound,
    CreatePoolError(CreatePoolError),
    BuildError(BuildError),
    PoolError(PoolError),
    HookError(HookError),
    ConfigError(ConfigError),
}
