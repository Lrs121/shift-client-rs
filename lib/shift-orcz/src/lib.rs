mod client;
pub mod code;
mod game;
pub mod shift_code;

pub use crate::{
    client::Client,
    code::Code,
    game::Game,
    shift_code::ShiftCode,
};

/// Library Result Type
pub type OrczResult<T> = Result<T, OrczError>;

/// Library Error Type
#[derive(Debug, thiserror::Error)]
pub enum OrczError {
    /// Reqwest HTTP Error
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),

    /// Invalid HTTP StatusCode
    #[error("invalid status '{0}'")]
    InvalidStatus(reqwest::StatusCode),

    /// Error Parsing a Table
    ///
    /// This is usually a library error; update this lib.
    #[error("invalid table")]
    TableParse,

    /// a tokio task failed
    #[error("{0}")]
    TokioJoin(#[from] tokio::task::JoinError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works_bl() {
        let client = Client::new();
        let codes = client.get_shift_codes(Game::Borderlands).await.unwrap();
        dbg!(codes);
    }

    #[tokio::test]
    async fn it_works_bl2() {
        let client = Client::new();
        let codes = client.get_shift_codes(Game::Borderlands2).await.unwrap();
        dbg!(codes);
    }

    #[tokio::test]
    async fn it_works_blps() {
        let client = Client::new();
        let codes = client
            .get_shift_codes(Game::BorderlandsPreSequel)
            .await
            .unwrap();
        dbg!(codes);
    }

    #[tokio::test]
    async fn it_works_bl3() {
        let client = Client::new();
        let codes = client.get_shift_codes(Game::Borderlands3).await.unwrap();
        dbg!(codes);
    }
}
