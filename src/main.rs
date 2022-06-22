use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::core::Name;

pub const CLEAR: Color = Color::rgb(0.0, 0.0, 0.0);
pub const RESOLUTION: f32 = 1.0 / 1.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor{
        width: 800.0,
        height: 800.0,
        title: "Rustlike".to_string(),
        resizable: false,
        ..Default::default()
        })
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii)
        .add_plugins(DefaultPlugins)
        .run();
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>){
    let mut sprite = TextureAtlasSprite::new(1);
    sprite.color = Color::rgb(0.3, 0.3, 0.9);
    sprite.custom_size = Some(Vec2::splat(0.1));

    commands.spawn_bundle(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: ascii.0.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 900.0),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Name::new("Player"));
}

fn spawn_camera(mut commands: Commands){
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}

struct AsciiSheet(Handle<TextureAtlas>);

fn load_ascii(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>){
        let image = assets.load("Ascii.png");
        let atlas = TextureAtlas::from_grid_with_padding(
            image,
            Vec2::splat(9.0),
            16,
            16,
            Vec2::splat(2.0)
        );

        let atlas_handle = texture_atlases.add(atlas);

        commands.insert_resource(AsciiSheet(atlas_handle));
}
