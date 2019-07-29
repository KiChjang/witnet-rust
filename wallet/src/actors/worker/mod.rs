use std::cell::RefCell;
use std::result;
use std::sync::{Arc, Mutex};

use actix::prelude::*;
use rand::Rng as _;

use witnet_crypto::key::SignEngine;

use crate::types;

pub mod db;
pub mod error;
pub mod handlers;
pub mod methods;
pub mod params;

pub use db::*;
pub use error::*;
pub use handlers::*;
pub use params::*;

pub type Result<T> = result::Result<T, Error>;

pub struct Worker {
    params: Params,
    engine: SignEngine,
    rng: RefCell<rand::rngs::ThreadRng>,
    wallets_mutex: Arc<Mutex<()>>,
    addresses_mutex: Arc<Mutex<()>>,
}

impl Actor for Worker {
    type Context = SyncContext<Self>;
}