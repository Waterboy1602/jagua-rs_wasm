use crate::io::export::export_layout_snapshot;
use crate::probs::bpp::entities::{BPInstance, BPSolution};
use crate::probs::bpp::io::ext_repr::ExtBPSolution;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;
#[cfg(target_arch = "wasm32")]
use web_time::Instant;

/// Exports a solution out of the library
pub fn export(instance: &BPInstance, solution: &BPSolution, epoch: Instant) -> ExtBPSolution {
    ExtBPSolution {
        cost: solution.cost(instance),
        layouts: solution
            .layout_snapshots
            .values()
            .map(|sl| export_layout_snapshot(sl, instance))
            .collect(),
        run_time_sec: solution.time_stamp.duration_since(epoch).as_secs(),
        density: solution.density(instance),
    }
}
