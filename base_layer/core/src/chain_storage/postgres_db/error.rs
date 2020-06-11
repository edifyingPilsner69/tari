// Copyright 2020 The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use crate::chain_storage::ChainStorageError;
use diesel::ConnectionError;
use tari_crypto::tari_utilities::{byte_array::ByteArrayError, hex::HexError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PostgresError {
    #[error("The requested value was not found in the database")]
    NotFound(String),
    #[error("The requested value could not be added to the database")]
    CouldNotAdd(String),
    #[error("The requested value could not be deleted from database")]
    CouldDelete(String),
    #[error("An error occurred in the database")]
    Other(String),
    #[error("Could not deserialize data")]
    SerializationError {
        #[from]
        source: HexError,
    },
    #[error("Could not deserialize data")]
    ByteSerializationError {
        #[from]
        source: ByteArrayError,
    },
    #[error("Could not deserialize json data")]
    JsonSerializationError {
        #[from]
        source: serde_json::error::Error,
    },
    #[error("Could not connect to db")]
    AccessError {
        #[from]
        source: ConnectionError,
    },
    #[error("Chain storage error")]
    ChainStorageError {
        #[from]
        source: ChainStorageError,
    },
    #[error("Diesel rs error")]
    DieselStorageError {
        #[from]
        source: diesel::result::Error,
    },
}

impl From<PostgresError> for ChainStorageError {
    fn from(e: PostgresError) -> Self {
        ChainStorageError::AccessError(format!("Postgres error:{}", e))
    }
}

// impl From<diesel::result::Error> for PostgresError {
//     fn from(e: diesel::result::Error) -> Self {
//         PostgresError::Other("meh".to_string())
//     }
// }