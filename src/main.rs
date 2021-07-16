use bevy::prelude::*;


const PLAYER_SPRITE: &str = "anim/player1.png";
const BG_NIGHT: &str = "setting/bg_night.png";


//resources
pub struct Materials{
    background: Handle<ColorMaterial>,
}
struct WinSize{
    h: f32,
    w: f32,
}


//components
struct Background;


struct Velocity{
    velocity: Vec3,
}



fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor{
            title: "Night Ward".to_string(),
            width: 640.0,
            height: 500.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage(
            "background setup",
            SystemStage::single(background_spawn.system())
          )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
    ){

    let mut window = windows.get_primary_mut().unwrap();
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    //build resources
    commands.insert_resource(Materials{
        background: materials.add(asset_server.load(BG_NIGHT).into()),
    });

    commands.insert_resource(WinSize{
        h: window.height(),
        w: window.width(),
    })
    
}

fn background_spawn(
    mut commands: Commands,
    materials: Res<Materials>,
    win_size: Res<WinSize>,
    ){

    let top = win_size.h / 2.0;
    let left = -win_size.w / 2.0;
    let right = win_size.w / 2.0;

    for i in 1..=3{
        commands
            .spawn_bundle(SpriteBundle{
                material: materials.background.clone(),
                transform: Transform{
                    translation: Vec3::new(0.0, top - 127.0, 0.0),
                    scale: Vec3::new(2.0, 2.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            })
        .insert(Velocity{
            velocity: 100.0 * Vec3::new(2.0, 0.0, 0.0),
        });
    }
}


