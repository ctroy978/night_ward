use bevy::prelude::*;

use crate::{WinSize, Materials, Background, Velocity, PlayerDirection, Direction};


pub struct BackgroundsPlugin;

impl Plugin for BackgroundsPlugin{
    fn build(&self, app: &mut AppBuilder){
        app
        .add_startup_stage(
            "background setup",
            SystemStage::single(background_spawn.system()),
          )
        .add_system(scroll_backgrounds.system())
        .add_system(flip_backgrounds.system());
    }
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
                translation: Vec3::new(left_x, 0.0, 0.7),
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
                translation: Vec3::new(right_x, 0.0, 0.7),
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
            material: materials.bg_10.clone(),
            transform: Transform{
                translation: Vec3::new(left_x, 0.0, 0.8),
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
            material: materials.bg_10.clone(),
            transform: Transform{
                translation: Vec3::new(right_x, 0.0, 0.8),
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
            material: materials.bg_04.clone(),
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
            material: materials.bg_05.clone(),
            transform: Transform{
                translation: Vec3::new(left_x, 0.0, 1.3),
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
            material: materials.bg_05.clone(),
            transform: Transform{
                translation: Vec3::new(right_x, 0.0, 1.3),
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
            material: materials.bg_06.clone(),
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
            material: materials.bg_06.clone(),
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
        //bg_07 must match bg_06 in z for speed
    commands
        .spawn_bundle(SpriteBundle{
            material: materials.bg_07.clone(),
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
            material: materials.bg_07.clone(),
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
    commands
        .spawn_bundle(SpriteBundle{
            material: materials.bg_08.clone(),
            transform: Transform{
                translation: Vec3::new(left_x, 0.0, 1.6),
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
            material: materials.bg_08.clone(),
            transform: Transform{
                translation: Vec3::new(right_x, 0.0, 1.6),
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
            material: materials.bg_09.clone(),
            transform: Transform{
                translation: Vec3::new(left_x, 0.0, 1.9),
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
            material: materials.bg_09.clone(),
            transform: Transform{
                translation: Vec3::new(right_x, 0.0, 1.9),
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
        //multiplyer is the z value of the background. The further back
        //into the background, the slower the multiplier.
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

