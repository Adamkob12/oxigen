use oxigen::prelude::*;
use std::time::Instant;

#[derive(Resource)]
struct Counter(usize);

#[derive(Resource)]
struct Timer(Instant);

/// Must have the [`Counter`] resource in the world.
fn stop_when_counter_is<const N: usize>(world: &World) -> bool {
    let counter = world.get_resource::<Counter>().unwrap();
    counter.0 >= N
}

fn increment_counter_every_n_secs(mut counter: ResMut<Counter>, mut timer: ResMut<Timer>) {
    if timer.0.elapsed().as_secs() >= 1 as u64 {
        counter.0 += 1;
        timer.0 = std::time::Instant::now();
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//                                 TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[test]
fn counter_test() {
    const N: usize = 1;
    let mut app = App::new().with_stop_condition(stop_when_counter_is::<N>);
    app.add_systems(Update, (increment_counter_every_n_secs,));
    app.insert_resource(Counter(0))
        .insert_resource(Timer(Instant::now()));

    let world_after_run = app.run();

    let counter = world_after_run.get_resource::<Counter>().unwrap();
    assert_eq!(counter.0, N);
}
