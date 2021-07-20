use bevy::prelude::*;

use crate::{SCALE_UP, Materials,  Direction, Platform, Velocity, Gravity, 
    Player, StrikeBox, PlayerAction, SPEEDSTOP, SPEEDFAST, SPEEDMED, SPEEDSLOW};


pub struct PlayersPlugin;
impl Plugin for PlayersPlugin{
    fn build(&self, app: &mut AppBuilder){
        app
            .add_startup_stage(
                "player", 
                SystemStage::single(player_spawn.system(),)
                  )
            .add_system(input_player.system())
            .add_system(gravity_player.system())
            .add_system(animate_player.system())
            .add_system(control_player.system());
    }
}
fn player_spawn(
    mut commands: Commands,
    materials: Res<Materials>,
    ){
    commands
        .spawn_bundle(SpriteSheetBundle{
            texture_atlas: materials.player_sprite.clone(),
            transform: Transform{
                translation: Vec3::new(0.0, 0.0, 1.8),
                scale: Vec3::new(SCALE_UP, SCALE_UP, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player{
            action: PlayerAction::Stand,
            direction: Direction::Right,
            vel_mod: SPEEDSTOP,
        })
        .insert(Timer::from_seconds(0.1, true))
        .insert(Velocity{
            velocity: Vec3::new(0.0, 0.0, 0.0),
        })
        .insert(Gravity{
            falling: true,
        })
        .insert(StrikeBox{
            //TODO: figure out the real size -- see platofrm touch!!!!!!!!!
            //!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!111
            h: 9.6 * SCALE_UP,  
            w: 7.1 * SCALE_UP,
            attack_h: 9.6 * SCALE_UP,
            attack_w: (7.1 * SCALE_UP) + 10.0,
        });
}


fn animate_player(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut Timer, &mut TextureAtlasSprite, 
        &Handle<TextureAtlas>, &mut Player, &mut Transform,
             )>,
             ){

    for(mut timer, mut sprite, texture_atlas_handle, mut player, mut transform) in query.iter_mut(){
        timer.tick(time.delta());
        if timer.finished(){
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            
            //is player going left or right? 
            if player.direction == Direction::Left{
                //face sprite left
                transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
            }else{
                transform.rotation = Quat::default();
            }

            match player.action{
                PlayerAction::Walk => {
                    match sprite.index{
                        18 => sprite.index = 19,
                        19 => sprite.index = 20,
                        20 => sprite.index = 21,
                        _ => sprite.index = 18,
                    }
                }
                PlayerAction::Charge =>{
                    match sprite.index{
                        14 => sprite.index = 15,
                        15 => sprite.index = 16,
                        16 => sprite.index = 17,
                        _ => sprite.index = 14,
                    } 
                    if sprite.index == 17{
                        player.vel_mod = SPEEDSTOP;
                        player.action = PlayerAction::Stand;
                    }
                }
                PlayerAction::Fly => {
                    match sprite.index{
                        23 => sprite.index = 24,
                        24 => sprite.index = 25,
                        25 => sprite.index = 26,
                        _ => sprite.index = 23,
                    }
                    if sprite.index == 26 {
                        player.vel_mod = SPEEDSTOP;
                        player.action = PlayerAction::Stand;
                    }
                }

                PlayerAction::Jump => {
                    match sprite.index{
                        23 => sprite.index = 24,
                        24 => sprite.index = 25,
                        25 => sprite.index = 26,
                        _ => sprite.index = 23,
                    }
                    if sprite.index == 26 {
                        if player.vel_mod == SPEEDSLOW{
                            player.action = PlayerAction::Walk;
                        }else{
                            if player.vel_mod == SPEEDSTOP{
                                player.action = PlayerAction::Stand;
                            } 
                        }
                    }
                }
                _ => {
                    sprite.index = 18
                }
            }
        }
    }
}

fn control_player(
    time: Res<Time>,
    mut query: Query<(&mut Player, &mut Transform, 
                      &mut Velocity, &mut Gravity)>,
    ){
    if let Ok((mut player, mut transform, 
               mut player_velocity, mut player_gravity)) = query.single_mut(){

        let delta_seconds = f32::min(0.3, time.delta_seconds());

        match player.action{
            PlayerAction::Jump =>{
                    if player_gravity.falling == false{
                    transform.translation.y += 10.0; //get it off plaform
                    player_velocity.velocity.y = 333.0; //initial up velocity 
                    player_gravity.falling = true;
                    //player.action = PlayerAction::Stand;
                    }
            }
            PlayerAction::Bumped => {
                transform.translation.y += 10.0;//get him off platform
                player_velocity.velocity.y = 30.0;
                player_gravity.falling = true;
                if player.direction == Direction::Left{
                    player.direction = Direction::Right;
                    transform.translation.x += 10.0; //get it off platform
                }else{
                    player.direction = Direction::Left;
                    transform.translation.x -= 10.0; //get it off platform
                }
                player.vel_mod = SPEEDMED;
                player.action = PlayerAction::Fly;
            }

            _ =>{
                //nothing
            }
        }
    }
}

fn gravity_player(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity, &Gravity), With<Player>>,
    ){
    if let Ok((mut transform, mut player_velocity, gravity)) = query.single_mut(){
        if gravity.falling{
            let delta_seconds = f32::min(0.3, time.delta_seconds());
            let g = 800.0 * Vec3::new(0.0, -2.0, 0.0).normalize();
            transform.translation += player_velocity.velocity * delta_seconds;
            player_velocity.velocity = player_velocity.velocity + (g * delta_seconds);
        }
    }
}

fn input_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player)>,
    ){


    if let Ok((mut player)) = query.single_mut(){

        if keyboard_input.just_pressed(KeyCode::Left){
            player.direction = Direction::Left;
            player.action = PlayerAction::Walk;
            player.vel_mod = SPEEDSLOW;
        }

        if keyboard_input.just_released(KeyCode::Left){
            player.action = PlayerAction::Stand;
            player.vel_mod = SPEEDSTOP;
        }

        if keyboard_input.just_pressed(KeyCode::Right){
            player.direction = Direction::Right;
            player.action = PlayerAction::Walk;
            player.vel_mod = SPEEDSLOW;
        }

        if keyboard_input.just_released(KeyCode::Right){
            player.action = PlayerAction::Stand;
            player.vel_mod = SPEEDSTOP;
        }

        if keyboard_input.just_pressed(KeyCode::R){
            player.vel_mod = SPEEDFAST;
            player.action = PlayerAction::Charge;
        }

        if keyboard_input.just_pressed(KeyCode::Space){
            if player.action != PlayerAction::Jump{
                player.action = PlayerAction::Jump;
            }
        }
    }
}


