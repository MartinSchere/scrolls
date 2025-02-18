use gasket::messaging::OutputPort;
use serde::Deserialize;

use crate::{bootstrap, crosscut, model, storage};

#[cfg(target_family = "unix")]
pub mod n2c;

pub mod n2n;
pub mod utils;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Config {
    N2N(n2n::Config),

    #[cfg(target_family = "unix")]
    N2C(n2c::Config),
}

impl Config {
    pub fn bootstrapper(
        self,
        chain: &crosscut::ChainWellKnownInfo,
        intersect: &crosscut::IntersectConfig,
    ) -> Bootstrapper {
        match self {
            Config::N2N(c) => Bootstrapper::N2N(c.bootstrapper(chain, intersect)),
            Config::N2C(c) => Bootstrapper::N2C(c.bootstrapper(chain, intersect)),
        }
    }
}

pub enum Bootstrapper {
    N2N(n2n::Bootstrapper),
    N2C(n2c::Bootstrapper),
}

impl Bootstrapper {
    pub fn borrow_output_port(&mut self) -> &'_ mut OutputPort<model::ChainSyncCommandEx> {
        match self {
            Bootstrapper::N2N(p) => p.borrow_output_port(),
            Bootstrapper::N2C(p) => p.borrow_output_port(),
        }
    }

    pub fn spawn_stages(self, pipeline: &mut bootstrap::Pipeline, state: storage::ReadPlugin) {
        match self {
            Bootstrapper::N2N(p) => p.spawn_stages(pipeline, state),
            Bootstrapper::N2C(p) => p.spawn_stages(pipeline, state),
        }
    }
}
