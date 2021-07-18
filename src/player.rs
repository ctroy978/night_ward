use bevy::prelude::*;

use crate::{SCALE_UP, Materials, PlayerDirection, Direction, 
    Player, PlayerAction};


pub struct PlayerPlugin;
impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut AppBuilder){
        app
            .add_startup_stage(
                "player", 
                SystemStage::single(player_spawn.system(),)
                  )
            .add_system(move_player.system())
            .add_system(animate_player.system());
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
                translation: Vec3::new(0.0, 0.0, 10.0),
                scale: Vec3::new(SCALE_UP, SCALE_UP, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
    .insert(Player{
        action: PlayerAction::Stand,
    })
        .insert(Timer::from_seconds(0.1, true));
}

fn animate_player(
    time: Res<Time>,
    player_direction: Res<PlayerDirection>,
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
            if player_direction.direction == Direction::Left{
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
                _ => {
                    sprite.index = 18
                }
            }
        }
    }
}

fn move_player(
    mut player_direction: ResMut<PlayerDirection>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player)>,
    ){

    if let Ok((mut player)) = query.single_mut(){

        if keyboard_input.pressed(KeyCode::Left){
            player_direction.direction = Direction::Left;
            player.action = PlayerAction::Walk;
        }
        if keyboard_input.just_released(KeyCode::Left){
            player_direction.direction = Direction::NotMoving;
            player.action = PlayerAction::Stand;
        }
        
        if keyboard_input.pressed(KeyCode::Right){
            player_direction.direction = Direction::Right;
            player.action = PlayerAction::Walk;
        } 

        if keyboard_input.just_released(KeyCode::Right){
            player_direction.direction = Direction::NotMoving;
            player.action = PlayerAction::Stand;
        }

    }
}

