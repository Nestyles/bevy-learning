use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_peoples)
            .add_system(greet_peoples)
            .add_system(hello_world);
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

// we query everything with a name and we filter with the component Person
fn greet_peoples(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}", name.0);
    }
}

fn hello_world() {
    println!("hello world !");
}
