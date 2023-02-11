use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{*, lens::UiPositionLens};

use crate::assets::Images;

use super::{machines::Machine, components::{Money, BuildingGhost}};

pub fn update_money(
    money_res: Res<Money>,
    ui_images: Res<Images>,
    game_images: Res<Images>,
    mut money_display: Query<&mut Text, With<MoneyDisplay>>,
    mut machine_names: Query<(&mut Text, &MachineName), Without<MoneyDisplay>>,
    mut machine_icons: Query<(&mut UiImage, &MachineIcon)>,
    mut machine_buy_buttons: Query<&mut MachineBuyButton>,
) {
    if !money_res.is_changed() {
        return;
    }

    let mut text = money_display.single_mut();
    text.sections[0].value = money_res.0.to_string();

    for (mut text, MachineName(machine)) in machine_names.iter_mut() {
        if money_res.0 >= machine.cost() {
            text.sections[0].value = machine.name().to_string();
        }
    }

    for (mut image, MachineIcon(machine)) in machine_icons.iter_mut() {
        if money_res.0 >= machine.cost() {
            image.0 = machine.image(&game_images);
        } else {
            image.0 = ui_images.locked.clone();
        }
    }

    for mut button in machine_buy_buttons.iter_mut() {
        button.enabled = money_res.0 >= button.machine.cost();
    }
}

pub fn handle_machine_buy_buttons(
    mut commands: Commands,
    ghosts: Query<&BuildingGhost>,
    images: Res<Images>,
    buttons: Query<(Entity, &Interaction, &MachineBuyButton), Changed<Interaction>>,
    mut money: ResMut<Money>,
) {
    for (entity, interaction, button) in buttons.iter() {
        if !button.enabled {
            continue;
        }

        match interaction {
            Interaction::Clicked => {
                if !ghosts.is_empty() {
                    continue;
                }

                money.0 -= button.machine.cost();

                match button.machine {
                    Machine::Miner => {
                        commands
                            .spawn(SpriteBundle {
                                texture: images.miner.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Machine(Machine::Miner));

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: 0,
                                offset_y: -1,
                            });
                    }

                    Machine::Collector => {
                        commands
                            .spawn(SpriteBundle {
                                texture: images.collector.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Machine(Machine::Collector));

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: 0,
                                offset_y: 1,
                            });
                    }

                    Machine::ConveyorUp => {
                        commands
                            .spawn(SpriteBundle {
                                texture: images.conveyor_up.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Machine(Machine::ConveyorUp));

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: 0,
                                offset_y: -1,
                            });

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: 0,
                                offset_y: 1,
                            });
                    }

                    Machine::ConveyorDown => {
                        commands
                            .spawn(SpriteBundle {
                                texture: images.conveyor_down.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Machine(Machine::ConveyorDown));

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: 0,
                                offset_y: -1,
                            });

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: 0,
                                offset_y: 1,
                            });
                    }

                    Machine::ConveyorLeft => {
                        commands
                            .spawn(SpriteBundle {
                                texture: images.conveyor_left.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Machine(Machine::ConveyorLeft));

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: -1,
                                offset_y: 0,
                            });

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: 1,
                                offset_y: 0,
                            });
                    }

                    Machine::ConveyorRight => {
                        commands
                            .spawn(SpriteBundle {
                                texture: images.conveyor_right.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Machine(Machine::ConveyorRight));

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: -1,
                                offset_y: 0,
                            });

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: 1,
                                offset_y: 0,
                            });
                    }

                    Machine::Adder => {
                        commands
                            .spawn(SpriteBundle {
                                texture: images.adder.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Machine(Machine::Adder));

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: -1,
                                offset_y: 0,
                            });

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: 1,
                                offset_y: 0,
                            });

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: 0,
                                offset_y: -1,
                            });
                    }

                    Machine::Multiplier => {
                        commands
                            .spawn(SpriteBundle {
                                texture: images.multiplier.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Machine(Machine::Multiplier));

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: -1,
                                offset_y: 0,
                            });

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: 1,
                                offset_y: 0,
                            });

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: 0,
                                offset_y: -1,
                            });
                    }
                }

                commands.entity(entity).insert(Animator::new(Tween::new(
                    EaseFunction::CubicOut,
                    Duration::from_secs_f32(0.2),
                    UiPositionLens {
                        start: UiRect::bottom(Val::Px(8.0)),
                        end: UiRect::bottom(Val::Px(0.0)),
                    },
                )));
            }

            Interaction::Hovered => {
                commands.entity(entity).insert(Animator::new(Tween::new(
                    EaseFunction::CubicOut,
                    Duration::from_secs_f32(0.2),
                    UiPositionLens {
                        start: UiRect::bottom(Val::Px(0.0)),
                        end: UiRect::bottom(Val::Px(8.0)),
                    },
                )));
            }

            Interaction::None => {
                commands.entity(entity).insert(Animator::new(Tween::new(
                    EaseFunction::CubicOut,
                    Duration::from_secs_f32(0.2),
                    UiPositionLens {
                        start: UiRect::bottom(Val::Px(8.0)),
                        end: UiRect::bottom(Val::Px(0.0)),
                    },
                )));
            }
        }
    }
}

#[derive(Component)]
pub struct MoneyDisplay;

#[derive(Component)]
pub struct MachineIcon(pub Machine);

#[derive(Component)]
pub struct MachineName(pub Machine);

#[derive(Component)]
pub struct MachineBuyButton {
    pub enabled: bool,
    pub machine: Machine,
}