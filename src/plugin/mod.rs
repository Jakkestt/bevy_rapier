pub use self::configuration::{RapierConfiguration, TimestepMode};
pub use self::plugin::{
    NoUserData, PhysicsSet, RapierContextInitialization, RapierPhysicsPlugin,
    RapierTransformPropagateSet,
};
pub use narrow_phase::{ContactManifoldView, ContactPairView, ContactView, SolverContactView};

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
pub mod systems;

pub mod configuration;
pub mod context;
mod narrow_phase;
#[allow(clippy::module_inception)]
mod plugin;
