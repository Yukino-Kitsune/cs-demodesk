//! Smoke snapshots require parser observer and raw voxel APIs not exposed by
//! the currently selected upstream demoparser version.
use super::projection::Snapshot;
use crate::{parser::DemoParser, replay::ReplayEvent};
use anyhow::{bail, Result};

pub fn replay(
    _parser: &DemoParser,
    _bytes: &[u8],
    _first_tick: i32,
    _last_tick: i32,
    _rate: f64,
    _events: &[ReplayEvent],
) -> Result<Vec<Snapshot>> {
    bail!("Smoke replay is disabled: required raw demoparser APIs are unavailable")
}
