use std::sync::LazyLock;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;
#[cfg(target_arch = "wasm32")]
use web_time::Instant;

pub mod config;
pub mod io;
pub mod opt;
pub mod samplers;
pub mod util;

pub static EPOCH: LazyLock<Instant> = LazyLock::new(Instant::now);

//limits the number of items to be placed, for debugging purposes
pub const ITEM_LIMIT: usize = usize::MAX;
