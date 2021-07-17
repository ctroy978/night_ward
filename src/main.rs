use bevy::prelude::*;


const PLAYER_SPRITE: &str = "anim/player1.png";
const BG_NIGHT: &str = "Background/Layer_0010_1.png";
const BG_02: &str = "Background/Layer_0009_2.png";
const BG_03: &str = "Background/Layer_0008_3.png";
const BG_04: &str = "Background/Layer_0006_4.png";


//resources
pub struct Materials{
    background: Handle<ColorMaterial>,
    bg_02: Handle<ColorMaterial>,
    bg_03: Handle<ColorMaterial>,
    bg_04: Handle<ColorMaterial>,
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

struct PlayerDirection{
    direction: Direction,
}

#[derive(PartialEq, Eq)]
enum Direction{
    Left,
    Right,
    Up,
    Down,
    NotMoving,
}


fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor{
            title: "Night Ward".to_string(),
            width: 900.0,
            height: 800.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage(
            "background setup",
            SystemStage::single(background_spawn.system())
          )
        .add_system(scroll_backgrounds.system())
        .add_system(flip_backgrounds.system())
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
        bg_02: materials.add(asset_server.load(BG_02).into()),
        bg_03: materials.add(asset_server.load(BG_03).into()),
        bg_04: materials.add(asset_server.load(BG_04).into()),
    });

    commands.insert_resource(WinSize{
        h: window.height(),
        w: window.width(),
    });

    commands.insert_resource(PlayerDirection{
        direction: Direction::Left,
    });
}

fn background_spawn(
    mut commands: Commands,
    materials: Res<Materials>,
    win_size: Res<WinSize>,
    ){

    let top = win_size.h / 2.0;
    let bottom = -win_size.h / 2.0;
    let left = win_size.w / 2.0;
    let bg_width = 928.0;
    let left_x = 0.0;
    let right_x = bg_width;

    commands
        //first layer
        .spawn_bundle(SpriteBundle{
            material: materials.background.clone(),
            transform: Transform{
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Background);
    commands
        .spawn_bundle(SpriteBundle{
            material: materials.bg_02.clone(),
            transform: Transform{
                translation: Vec3::new(left_x, 0.0, 0.5),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Background)
        .insert(Velocity{
            velocity: Vec3::new(1.0, 0.0, 0.0),
        });
    commands
        .spawn_bundle(SpriteBundle{
            material: materials.bg_02.clone(),
            transform: Transform{
                translation: Vec3::new(right_x, 0.0, 0.5),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Background)
        .insert(Velocity{
            velocity: Vec3::new(1.0, 0.0, 0.0),
        });
    commands
        .spawn_bundle(SpriteBundle{
            material: materials.bg_03.clone(),
            transform: Transform{
                translation: Vec3::new(left_x, 0.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Background)
            .insert(Velocity{
                velocity: Vec3::new(1.0, 0.0, 0.0),
            });
    commands
        .spawn_bundle(SpriteBundle{
            material: materials.bg_03.clone(),
            transform: Transform{
                translation: Vec3::new(right_x, 0.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Background)
            .insert(Velocity{
                velocity: Vec3::new(1.0, 0.0, 0.0),
            });
    commands
        .spawn_bundle(SpriteBundle{
            material: materials.bg_04.clone(),
            transform: Transform{
                translation: Vec3::new(left_x, 0.0, 1.5),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Background)
            .insert(Velocity{
                velocity: Vec3::new(1.0, 0.0, 0.0),
        });
    commands
        .spawn_bundle(SpriteBundle{
            material: materials.bg_04.clone(),
            transform: Transform{
                translation: Vec3::new(right_x, 0.0, 1.5),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Background)
            .insert(Velocity{
                velocity: Vec3::new(1.0, 0.0, 0.0),
        });
}


fn get_size(
    mut query: Query<(&mut Sprite ), With<Background>>,
    ){
    for (trans) in query.iter_mut(){
        println!("{:?}", trans);
    }
}

fn scroll_backgrounds(
    time: Res<Time>,
    player_direction: Res<PlayerDirection>,
    mut query: Query<(&mut Transform, &Velocity),With<Background>>,
    ){
    for (mut transform, velocity) in query.iter_mut(){
        //layers will be divided by value in transform.z.  
        //the further back z (lower) the slower the velocity. 
        let vel  = match player_direction.direction{
            Direction::Right => -100.0,
            Direction::Left => 100.0,
            Direction::NotMoving => 0.0,
            _ => 100.0,
        };
        let multiplier = transform.translation.z;
        transform.translation += 
            (vel * multiplier) * velocity.velocity * time.delta_seconds();
    }
}

fn flip_backgrounds(
    player_direction: Res<PlayerDirection>,
    mut query: Query<(&Sprite, &mut Transform), With<Background>>,
    ){
    for(mut sprite, mut transform) in query.iter_mut(){
        if player_direction.direction == Direction::Right && transform.translation.x < -sprite.size.x{
            transform.translation.x = transform.translation.x + (sprite.size.x * 2.0);
        }
        if player_direction.direction == Direction::Left && transform.translation.x > sprite.size.x{
            transform.translation.x = transform.translation.x - (sprite.size.x * 2.0);
        }
    }
}

