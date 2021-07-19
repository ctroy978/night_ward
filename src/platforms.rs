
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::{SCALE_UP, Materials,  Direction, WinSize, Platform, Gravity, 
    Player, StrikeBox, PlayerAction, SPEEDSTOP, SPEEDFAST, SPEEDSLOW};


pub struct PlatformsPlugin;
impl Plugin for PlatformsPlugin{
    fn build(&self, app: &mut AppBuilder){
        app
            .add_startup_stage(
                "platforms", 
                SystemStage::single(platform_spawn.system(),)
                  )
            .add_system(touching_platform.system());
    }
}

fn platform_spawn(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<WinSize>,
    ){
    let screen_bottom = -(windows.h/2.0);
    let screen_width = windows.w;
    commands
        .spawn_bundle(SpriteBundle{
            sprite: Sprite::new(Vec2::new(screen_width, 30.0)),
            transform: Transform{
                //z translation is 0.0 to keep hidden. Not working right.   
                translation: Vec3::new(0.0, screen_bottom + 35.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Platform);
}

fn touching_platform(
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<(
        Entity, &Transform,  &TextureAtlasSprite, &StrikeBox, &mut Gravity), With<Player>>,
    mut platform_query: Query<(Entity,  &Transform, &Sprite), With<Platform>>,
    ){

    for(platform_entity, platform_tf, platform_sprite) in platform_query.iter(){
        for(player_entity, player_tf, platform_sprite, strike_box, mut gravity) in player_query.iter_mut(){
            let platform_size = Vec2::new(800.0, 30.0);
            let player_size= Vec2::new(strike_box.h, strike_box.w);

            let collision = collide(
                platform_tf.translation,
                platform_size,
                player_tf.translation,
                player_size,
               );
            

            if let Some(_) = collision{
                gravity.falling = false;
            }
        };
    }
}
