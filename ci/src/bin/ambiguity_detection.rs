//! A test to confirm that `bevy_rapier` doesn't regress its system ambiguities count when using [`DefaultPlugins`].
//! This is run in CI.
//!
//! Note that because we want to test this in a more complete setup than examples, we're using a different project here.
//! You can run this test using `cargo run --bin ambiguity_detection`.

use bevy::{
    app::Plugins,
    ecs::schedule::{InternedScheduleLabel, LogLevel, ScheduleBuildSettings},
    log::LogPlugin,
    prelude::*,
    utils::HashMap,
};

fn main() {
    check_ambiguities(
        (
            bevy_rapier3d::plugin::RapierPhysicsPlugin::<()>::default(),
            bevy_rapier3d::prelude::RapierPickingPlugin::default(),
        ),
        true,
    );
    check_ambiguities(
        (
            bevy_rapier3d::plugin::RapierPhysicsPlugin::<()>::default().in_fixed_schedule(),
            bevy_rapier3d::prelude::RapierPickingPlugin::default(),
        ),
        false,
    );
    check_ambiguities(
        (
            bevy_rapier2d::plugin::RapierPhysicsPlugin::<()>::default(),
            bevy_rapier3d::prelude::RapierPickingPlugin::default(),
        ),
        false,
    );
    check_ambiguities(
        (
            bevy_rapier2d::plugin::RapierPhysicsPlugin::<()>::default().in_fixed_schedule(),
            bevy_rapier3d::prelude::RapierPickingPlugin::default(),
        ),
        false,
    );
}

fn check_ambiguities<M>(plugin: impl Plugins<M>, set_logger: bool) {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TransformPlugin, AssetPlugin::default()));
    if set_logger {
        app.add_plugins(LogPlugin::default());
    }
    app.add_plugins(plugin);

    let main_app = app.main_mut();
    configure_ambiguity_detection(main_app);

    app.finish();
    app.cleanup();
    app.update();

    let main_app_ambiguities = count_ambiguities(app.main());
    assert_eq!(
        main_app_ambiguities.total(),
        0,
        "Main app has unexpected ambiguities among the following schedules: \n{main_app_ambiguities:#?}.",
    );
}

/// Contains the number of conflicting systems per schedule.
#[derive(Debug, Deref, DerefMut)]
struct AmbiguitiesCount(pub HashMap<InternedScheduleLabel, usize>);

impl AmbiguitiesCount {
    fn total(&self) -> usize {
        self.values().sum()
    }
}

fn configure_ambiguity_detection(sub_app: &mut SubApp) {
    let mut schedules = sub_app.world_mut().resource_mut::<Schedules>();
    for (_, schedule) in schedules.iter_mut() {
        schedule.set_build_settings(ScheduleBuildSettings {
            ambiguity_detection: LogLevel::Ignore,
            use_shortnames: false,
            ..default()
        });
    }
}

/// Returns the number of conflicting systems per schedule.
fn count_ambiguities(sub_app: &SubApp) -> AmbiguitiesCount {
    let schedules = sub_app.world().resource::<Schedules>();
    let mut ambiguities = HashMap::new();
    for (_, schedule) in schedules.iter() {
        let ambiguities_in_schedule = schedule.graph().conflicting_systems().len();
        ambiguities.insert(schedule.label(), ambiguities_in_schedule);
    }
    AmbiguitiesCount(ambiguities)
}
