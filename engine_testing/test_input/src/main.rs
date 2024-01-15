use oxigen::prelude::*;

fn print_when_press(input: Res<Keyboard>) {
    if input.pressed(KeyCode::Space) {
        println!("Space was pressed");
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugin(oxigen::DefaultPlugins);
    app.add_systems(Update, print_when_press);
    app.run();
}
