use bevy::prelude::*;


mod backgrounds;
mod player;
use backgrounds::BackgroundsPlugin;
use player::PlayerPlugin;


const BG_NIGHT: &str = "Background/Layer_0010_1.png";
const BG_02: &str = "Background/Layer_0009_2.png";
const BG_03: &str = "Background/Layer_0008_3.png";
const BG_04: &str = "Background/Layer_0006_4.png";
const BG_05: &str = "Background/Layer_0005_5.png";
const BG_06: &str = "Background/Layer_0003_6.png";
const BG_07: &str = "Background/Layer_0002_7.png";
const BG_08: &str = "Background/Layer_0001_8.png";
const BG_09: &str = "Background/Layer_0000_9.png";
const BG_10: &str = "Background/Layer_0007_Lights.png";

//game assets
const SCALE_UP: f32 = 3.5;
const PLAYER_SPRITE: &str = "anim/player1.png";

//game values
const SPEEDFAST: f32 = 300.0;
const SPEEDSLOW: f32 = 100.0;
const SPEEDSTOP: f32 = 0.0;

//resources
pub struct Materials{
    background: Handle<ColorMaterial>,
    bg_02: Handle<ColorMaterial>,
    bg_03: Handle<ColorMaterial>,
    bg_04: Handle<ColorMaterial>,
    bg_05: Handle<ColorMaterial>,
    bg_06: Handle<ColorMaterial>,
    bg_07: Handle<ColorMaterial>,
    bg_08: Handle<ColorMaterial>,
    bg_09: Handle<ColorMaterial>,
    bg_10: Handle<ColorMaterial>,
    //game assets
    player_sprite: Handle<TextureAtlas>,
}
pub struct WinSize{
    h: f32,
    w: f32,
}

struct Player{
    action: PlayerAction,
    direction: Direction, 
    vel_mod: f32, //RUN, WALK, STOP, etc
}


//components
struct Background;

struct Velocity{
    velocity: Vec3,
}

//struct PlayerDirection{
//    direction: Direction,
//}

enum PlayerAction{
    Charge,
    Chop,
    Jump,
    Stand,
    Swipe,
    Walk,
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
            height: 700.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(BackgroundsPlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: ResMut<Windows>,
    ){

    let mut window = windows.get_primary_mut().unwrap();
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    //create texture atless for player
    let texture_handle_player = asset_server.load(PLAYER_SPRITE);
    let texture_atlas_player =
        TextureAtlas::from_grid(
            texture_handle_player, Vec2::new(64.0, 48.0), 9,4 
                               );
    //build resources
    commands.insert_resource(Materials{
        background: materials.add(asset_server.load(BG_NIGHT).into()),
        bg_02: materials.add(asset_server.load(BG_02).into()),
        bg_03: materials.add(asset_server.load(BG_03).into()),
        bg_04: materials.add(asset_server.load(BG_04).into()),
        bg_05: materials.add(asset_server.load(BG_05).into()),
        bg_06: materials.add(asset_server.load(BG_06).into()),
        bg_07: materials.add(asset_server.load(BG_07).into()),
        bg_08: materials.add(asset_server.load(BG_08).into()),
        bg_09: materials.add(asset_server.load(BG_09).into()),
        bg_10: materials.add(asset_server.load(BG_10).into()),
        player_sprite: texture_atlases.add(texture_atlas_player),
    });


    commands.insert_resource(WinSize{
        h: window.height(),
        w: window.width(),
    });

    //commands.insert_resource(PlayerDirection{
    //    direction: Direction::Left,
    //});
}
