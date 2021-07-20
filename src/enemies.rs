
use bevy::prelude::*;

use crate::{SCALE_UP, Materials,  Direction, Platform, Velocity, Gravity, 
    Player, StrikeBox, PlayerAction, SPEEDSTOP, SPEEDFAST, SPEEDMED, SPEEDSLOW,
    Enemy, Proximity, WinSize};


pub struct EnemiesPlugin;
impl Plugin for EnemiesPlugin{
    fn build(&self, app: &mut AppBuilder){
        app
            .add_startup_stage(
                "enemy", 
                SystemStage::single(enemy_spawn.system(),)
                )
            .add_system(scroll_enemy.system())
            .add_system(animate_enemy.system())
            .add_system(near_player.system());
    }
}


fn enemy_spawn(
    mut commands: Commands,
    materials: Res<Materials>,
    ){
    commands
        .spawn_bundle(SpriteSheetBundle{
            texture_atlas: materials.skely_one_sprite.clone(),
            transform: Transform{
                translation: Vec3::new(700.0, 0.0, 1.8),
                scale: Vec3::new(SCALE_UP, SCALE_UP, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
    .insert(Enemy{
        action: PlayerAction::Stand,
        direction: Direction::Right,
    })
    .insert(Timer::from_seconds(0.1, true))
    .insert(Gravity{
        falling: false,
    })
    .insert(Velocity{
        velocity: Vec3::new(1.0, 0.0, 0.0),
    })
    .insert(Proximity{
        near_player: false,
    })
    .insert(StrikeBox{
        h: 40.0,
        w: 115.0,
        attack_h: 40.0,
        attack_w: 130.0,
    });

}

fn near_player(
    window: Res<WinSize>,
    mut q: QuerySet<(
        Query<&mut Transform, With<Enemy>>,
        Query<&mut Transform, With<Player>>,
        Query<&mut Proximity, With<Enemy>>,
        )>,
    ){
        let mut enemy_x: f32 = 0.0;
        let mut player_x: f32 = 0.0;
        for mut transform in q.q0_mut().iter_mut(){
            enemy_x = transform.translation.x; 
        }
        for mut transform in q.q1_mut().iter_mut(){
            player_x = transform.translation.x + (window.w / 2.0);
        }
        for mut proximity in q.q2_mut().iter_mut(){
            if enemy_x < player_x && enemy_x > -player_x{
                proximity.near_player = true;
            }else{
                proximity.near_player = false;
            } 
        }
}

fn animate_enemy(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut Timer, &mut TextureAtlasSprite, 
        &Handle<TextureAtlas>, &mut Enemy, &mut Transform,
             )>,
             ){

    for(mut timer, mut sprite, texture_atlas_handle, mut enemy, mut transform) in query.iter_mut(){
        timer.tick(time.delta());
        if timer.finished(){
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();

            match enemy.action{
                PlayerAction::Walk => {
                    match sprite.index{
                        18 => sprite.index = 19,
                        19 => sprite.index = 20,
                        20 => sprite.index = 21,
                        _ => sprite.index = 18,
                    }
                    
                }
                _ => sprite.index = 18,
            }
        }
    }
}

fn scroll_enemy(
    time: Res<Time>,
    mut bg_query: Query<(&mut Enemy, &mut Transform, &Velocity, &Proximity)>,
    mut player_query: Query<(&Player)>,
    ){
    if let Ok((player)) = player_query.single_mut(){
        //layers will be divided by value in transform.z.  
        //the further back z (lower) the slower the velocity. 
        let dir = match player.direction{
            Direction::Right => -1.0,
            Direction::Left => 1.0,
            _ => 0.0,
        };
        let mut vel = dir * player.vel_mod;
        for (mut enemy, mut transform, enemy_velocity, proximity) in bg_query.iter_mut(){

            //once enemy is on screen, the enemy closes in on player
            if proximity.near_player{
                if transform.translation.x > player.current_x{
                    vel += -1.0 * 50.0;
                    //flip sprite 
                    transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
                    //animate walking
                    enemy.action = PlayerAction::Walk;
                }else{
                    vel += 1.0 * 50.0;
                    //flip sprite
                    transform.rotation = Quat::default();
                    //animate walking
                    enemy.action = PlayerAction::Walk;
                }
            }else{
                vel = dir * player.vel_mod;
            }

            //multiplyer is the z value of the background. The further back
            //into the background, the slower the multiplier.
            let multiplier = transform.translation.z;
            transform.translation.x  += 
                (vel * multiplier) * enemy_velocity.velocity.x * time.delta_seconds();

        }
    }
}
