use derive_more::{Display, Error, From};

#[derive(Debug, Display, Error, From)]
pub enum DBError {
    NotFound,
    /*
    PGError(PGError),
    PGMError(PGMError),
    PoolError(PoolError),*/
}
