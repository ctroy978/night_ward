
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::{SCALE_UP, Materials,  Direction, WinSize, Platform, Gravity, 
    Player, StrikeBox, PlayerAction, SPEEDSTOP, SPEEDFAST, SPEEDSLOW, Velocity};

const MAX_HEIGHT: f32 = 60.0;//added from bottom of screen

pub struct PlatformsPlugin;
impl Plugin for PlatformsPlugin{
    fn build(&self, app: &mut AppBuilder){
        app
            .add_startup_stage(
                "platforms", 
                SystemStage::single(platform_spawn.system(),)
                  )
            .add_system(touching_platform.system())
            .add_system(scroll_platform.system());
    }
}

fn platform_spawn(
    mut commands: Commands,
    //mut materials: ResMut<Assets<ColorMaterial>>,
    materials: Res<Materials>,
    windows: Res<WinSize>,
    ){
    let screen_bottom = -(windows.h/2.0);
    let screen_width = windows.w;
    commands
        .spawn_bundle(SpriteBundle{
            material: materials.pl_01.clone(),
            sprite: Sprite::new(Vec2::new(screen_width, 30.0)),
            transform: Transform{
                //z translation is 0.0 to keep hidden. Not working right.   
                translation: Vec3::new(0.0, screen_bottom - 5.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Platform);

    commands
        .spawn_bundle(SpriteBundle{
            sprite: Sprite::new(Vec2::new(90.0, 30.0)), //Sprite::new(Vec2::new(60.0, 30.0)),
            transform: Transform{
                translation: Vec3::new(300.0, screen_bottom + MAX_HEIGHT, 1.9),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Platform)
        .insert(Velocity{
            velocity: Vec3::new(1.0, 0.0, 0.0),    
        });
}

fn touching_platform(
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<(
        &mut Player, &Transform,  &TextureAtlasSprite, &StrikeBox, &mut Gravity)>,
    mut platform_query: Query<(Entity,  &Transform, &Sprite), With<Platform>>,
    ){

    let mut on_something = false;
    for(mut player, player_tf, player_sprite, strike_box, mut gravity) in player_query.iter_mut(){
        for(platform_entity, platform_tf, platform_sprite) in platform_query.iter(){

            //let player_size= Vec2::new(strike_box.h, strike_box.w);
            let player_size= Vec2::new(40.0, 115.0);

            let collision = collide(
                platform_tf.translation,
                platform_sprite.size,
                player_tf.translation,
                player_size,
               );
            

            if let Some(_) = collision{
                if player_tf.translation.y - player_size.y/2.0 + 5.0 > platform_tf.translation.y{
                    on_something = true;
                }
                else{
                    player.action = PlayerAction::Bumped;
                }
            }
        };
        if on_something{
            gravity.falling = false;
        }else{
            gravity.falling = true;
        }
    }
}

fn scroll_platform(
    time: Res<Time>,
    mut bg_query: Query<(&mut Transform, &Velocity),With<Platform>>,
    mut player_query: Query<(&Player)>,
    ){
    if let Ok((player)) = player_query.single_mut(){
        //layers will be divided by value in transform.z.  
        //the further back z (lower) the slower the velocity. 
        let dir = match player.direction{
            Direction::Right => -1.0,
            Direction::Left => 1.0,
            //Direction::NotMoving => 0.0,
            _ => 0.0,
        };
        let vel = dir * player.vel_mod;
        for (mut transform, velocity) in bg_query.iter_mut(){
            //multiplyer is the z value of the background. The further back
            //into the background, the slower the multiplier.
            let multiplier = transform.translation.z;
            transform.translation += 
                (vel * multiplier) * velocity.velocity * time.delta_seconds();
        }
    }
}
