use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_peoples)
            .add_system(greet_peoples);
    }

}

// systems run in parallel so greet_peoples and hello_world can happen in random orders
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

fn add_peoples(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Elaina Proctor".to_string()));
    commands.spawn().insert(Person).insert(Name("Renzo Hume".to_string()));
    commands.spawn().insert(Person).insert(Name("Zayna Nieves".to_string()));
}

struct GreetTimer(Timer);

// we query everything with a name and we filter with the component Person
fn greet_peoples(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}", name.0);
        }
    }
}
