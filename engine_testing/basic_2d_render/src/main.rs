use std::sync::Arc;

pub use oxigen::prelude::*;

#[derive(Resource, Default)]
struct Sprites(HashMap<&'static str, Arc<Sprite>>);

fn main() {
    let mut app = App::new();

    app.add_plugin(DefaultPlugins)
        .init_resource::<Sprites>()
        .add_systems(Startup, setup_sprites)
        .add_systems(Update, spawn_stars);

    app.run();
}

fn setup_sprites(mut sprites: ResMut<Sprites>) {
    let sprite = Sprite::load("assets/star.png").expect("Couldn't load sprite");
    let sprite = Arc::new(sprite);
    sprites.0.insert("Star", sprite);
    println!("Loaded sprite!");
}

fn spawn_stars(world: &mut World, sprites: Res<Sprites>, input: Res<Keyboard>) {
    if input.pressed(KeyCode::Space) {
        println!("Spawing Star!");
        let sprite_handle = Arc::clone(sprites.0.get("Star").unwrap());
        let x = rand::random::<f32>() * 500.0;
        let y = rand::random::<f32>() * 500.0;
        world.spawn(
            SpriteBundle::from_sprite(sprite_handle).with_transform(Transform {
                position: Vec3::new(x, y, 0.0),
            }),
        );
    }
}
