use std::time::Duration;

use bevy::{prelude::*, ui::FocusPolicy};
use bevy_ninepatch::{NinePatchBundle, NinePatchData};
use bevy_tweening::{
    lens::{TransformPositionLens, UiPositionLens},
    *,
};

use crate::{
    assets::{Fonts, Images, NinePatches},
    palette,
};

use super::{
    components::Balance,
    input::{MouseButtonState, WorldMouse, WorldMouseEvent},
    machines::{Machine, MachinePlaceRequest, MachineDeleteRequest},
    tile_tracked_entities::TilePosition,
    TILE_SIZE,
};

pub fn setup_hud(
    mut commands: Commands,
    images: Res<Images>,
    fonts: Res<Fonts>,
    ninepatches: Res<NinePatches>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Stretch,
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .with_children(|window| {
            window
                .spawn(NodeBundle {
                    style: Style {
                        align_items: AlignItems::FlexEnd,
                        ..default()
                    },
                    focus_policy: FocusPolicy::Pass,
                    ..default()
                })
                .with_children(|top_panel| {
                    top_panel
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                padding: UiRect::all(Val::Px(8.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|money_display| {
                            money_display
                                .spawn(ImageBundle {
                                    image: images.coin.clone().into(),
                                    style: Style {
                                        size: Size::new(Val::Px(48.0), Val::Px(48.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|coin| {
                                    coin.spawn(TextBundle {
                                        text: Text::from_section(
                                            "1",
                                            TextStyle {
                                                font: fonts.varela.clone(),
                                                color: palette::DARK_BLUE,
                                                font_size: 40.0,
                                            },
                                        )
                                        .with_alignment(TextAlignment::CENTER),
                                        ..default()
                                    });
                                })
                                .insert(Name::new("Icon"));

                            money_display
                                .spawn(TextBundle {
                                    text: Text::from_section(
                                        "0",
                                        TextStyle {
                                            font: fonts.varela.clone(),
                                            color: palette::DARK_BLUE,
                                            font_size: 48.0,
                                        },
                                    ),
                                    style: Style {
                                        margin: UiRect {
                                            left: Val::Px(16.0),
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    ..default()
                                })
                                .insert(Name::new("Value"))
                                .insert(MoneyDisplay);
                        })
                        .insert(Name::new("Money Display"));
                })
                .insert(Name::new("Top Panel"))
                .insert(Animator::new(Tween::new(
                    EaseFunction::QuadraticOut,
                    Duration::from_secs_f32(0.2),
                    UiPositionLens {
                        start: UiRect::top(Val::Px(-64.0)),
                        end: UiRect::top(Val::Px(0.0)),
                    },
                )));

            let bottom_panel_content = window
                .spawn(NodeBundle {
                    style: Style {
                        align_items: AlignItems::FlexEnd,
                        justify_content: JustifyContent::Center,
                        min_size: Size {
                            height: Val::Px(208.0),
                            ..default()
                        },
                        padding: UiRect::new(
                            Val::Px(-8.0),
                            Val::Px(-8.0),
                            Val::Px(-8.0),
                            Val::Px(64.0),
                        ),
                        ..default()
                    },
                    focus_policy: FocusPolicy::Pass,
                    ..default()
                })
                .with_children(|bottom_panel| {
                    for machine in Machine::list() {
                        bottom_panel
                            .spawn(ButtonBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Column,
                                    padding: UiRect::all(Val::Px(8.0)),
                                    align_items: AlignItems::Center,
                                    size: Size {
                                        width: Val::Px(90.0),
                                        height: Val::Undefined,
                                    },
                                    ..default()
                                },
                                background_color: Color::NONE.into(),
                                ..default()
                            })
                            .with_children(|container| {
                                container
                                    .spawn(TextBundle {
                                        text: Text::from_section(
                                            "???",
                                            TextStyle {
                                                font: fonts.varela.clone(),
                                                color: palette::LIGHT_BROWN,
                                                font_size: 20.0,
                                            },
                                        )
                                        .with_alignment(TextAlignment::BOTTOM_CENTER),
                                        style: Style {
                                            margin: UiRect {
                                                bottom: Val::Px(4.0),
                                                ..default()
                                            },
                                            max_size: Size {
                                                width: Val::Px(90.0),
                                                height: default(),
                                            },
                                            ..default()
                                        },
                                        focus_policy: FocusPolicy::Pass,
                                        ..default()
                                    })
                                    .insert(MachineName(*machine));

                                container
                                    .spawn(ImageBundle {
                                        image: images.locked.clone().into(),
                                        style: Style {
                                            size: Size::new(Val::Px(64.0), Val::Px(64.0)),
                                            ..default()
                                        },
                                        focus_policy: FocusPolicy::Pass,
                                        ..default()
                                    })
                                    .insert(MachineIcon(*machine));

                                container.spawn(TextBundle {
                                    text: Text::from_section(
                                        machine.cost().to_string(),
                                        TextStyle {
                                            font: fonts.varela.clone(),
                                            color: palette::DARK_BLUE,
                                            font_size: 28.0,
                                        },
                                    ),
                                    focus_policy: FocusPolicy::Pass,
                                    ..default()
                                });
                            })
                            .insert(ToolbarButton::default())
                            .insert(*machine);
                    }

                    bottom_panel
                        .spawn(ButtonBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(8.0)),
                                align_items: AlignItems::Center,
                                size: Size {
                                    width: Val::Px(90.0),
                                    height: Val::Undefined,
                                },
                                ..default()
                            },
                            background_color: Color::NONE.into(),
                            ..default()
                        })
                        .with_children(|container| {
                            container.spawn(TextBundle {
                                text: Text::from_section(
                                    "Delete",
                                    TextStyle {
                                        font: fonts.varela.clone(),
                                        color: palette::LIGHT_BROWN,
                                        font_size: 20.0,
                                    },
                                )
                                .with_alignment(TextAlignment::BOTTOM_CENTER),
                                style: Style {
                                    margin: UiRect {
                                        bottom: Val::Px(4.0),
                                        ..default()
                                    },
                                    max_size: Size {
                                        width: Val::Px(90.0),
                                        height: default(),
                                    },
                                    ..default()
                                },
                                focus_policy: FocusPolicy::Pass,
                                ..default()
                            });

                            container.spawn(ImageBundle {
                                image: images.delete.clone().into(),
                                style: Style {
                                    size: Size::new(Val::Px(64.0), Val::Px(64.0)),
                                    ..default()
                                },
                                focus_policy: FocusPolicy::Pass,
                                ..default()
                            });

                            container.spawn(TextBundle {
                                text: Text::from_section(
                                    " ".to_string(),
                                    TextStyle {
                                        font: fonts.varela.clone(),
                                        color: palette::DARK_BLUE,
                                        font_size: 28.0,
                                    },
                                ),
                                focus_policy: FocusPolicy::Pass,
                                ..default()
                            });
                        })
                        .insert(ToolbarButton {
                            enabled: true,
                            ..default()
                        })
                        .insert(ToolbarButtonDelete);
                })
                .insert(Name::new("Bottom Panel Content"))
                .id();

            window
                .spawn(NinePatchBundle {
                    nine_patch_data: NinePatchData::with_single_content(
                        images.panel.clone(),
                        ninepatches.panel.clone(),
                        bottom_panel_content,
                    ),
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        align_self: AlignSelf::Center,
                        position: UiRect {
                            bottom: Val::Px(-84.0 - 144.0),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::new("Bottom Panel"))
                .insert(Animator::new(Tween::new(
                    EaseFunction::QuadraticOut,
                    Duration::from_secs_f32(0.2),
                    UiPositionLens {
                        start: UiRect::bottom(Val::Px(-84.0 - 144.0)),
                        end: UiRect::bottom(Val::Px(-84.0)),
                    },
                )));
        });
}

pub fn update_balance_display(
    wallet: Res<Balance>,
    ui_images: Res<Images>,
    game_images: Res<Images>,
    mut money_display: Query<&mut Text, With<MoneyDisplay>>,
    mut machine_names: Query<(&mut Text, &MachineName), Without<MoneyDisplay>>,
    mut machine_icons: Query<(&mut UiImage, &MachineIcon)>,
    mut machine_buy_buttons: Query<(&mut ToolbarButton, &Machine)>,
) {
    if !wallet.is_changed() {
        return;
    }

    let mut text = money_display.single_mut();
    text.sections[0].value = wallet.coins.to_string();

    for (mut text, MachineName(machine)) in machine_names.iter_mut() {
        if wallet.coins >= machine.cost() {
            text.sections[0].value = machine.name().to_string();
        }
    }

    for (mut image, MachineIcon(machine)) in machine_icons.iter_mut() {
        if wallet.coins >= machine.cost() {
            image.0 = machine.image(&game_images);
        } else {
            image.0 = ui_images.locked.clone();
        }
    }

    for (mut button, machine) in machine_buy_buttons.iter_mut() {
        button.enabled = wallet.coins >= machine.cost();
    }
}

pub fn select_toolbar_button(
    buttons: Query<(Entity, &Interaction, &ToolbarButton), Changed<Interaction>>,
    mut button_selected_events: EventWriter<ToolbarButtonSelectedEvent>,
) {
    for (entity, interaction, button) in buttons.iter() {
        if !button.enabled {
            continue;
        }

        match interaction {
            Interaction::Clicked if !button.selected => {
                button_selected_events.send(ToolbarButtonSelectedEvent(Some(entity)));
            }

            Interaction::Clicked if button.selected => {
                button_selected_events.send(ToolbarButtonSelectedEvent(None));
            }

            _ => (),
        }
    }
}

pub fn update_selected_machine_button(
    mut commands: Commands,
    mut buttons: Query<(Entity, &mut ToolbarButton, &Style)>,
    mut button_selected_events: EventReader<ToolbarButtonSelectedEvent>,
) {
    for &ToolbarButtonSelectedEvent(selected_option) in button_selected_events.iter() {
        for (entity, mut button, style) in buttons.iter_mut() {
            let start_bottom = match style.position.bottom {
                Val::Px(val) => Val::Px(val),
                _ => Val::Px(0.0),
            };

            match selected_option {
                Some(selected) if selected == entity => {
                    button.selected = true;

                    commands.entity(entity).insert(Animator::new(Tween::new(
                        EaseFunction::CubicOut,
                        Duration::from_secs_f32(0.2),
                        UiPositionLens {
                            start: UiRect::bottom(start_bottom),
                            end: UiRect::bottom(Val::Px(12.0)),
                        },
                    )));
                }

                _ => {
                    button.selected = false;

                    commands.entity(entity).insert(Animator::new(Tween::new(
                        EaseFunction::CubicOut,
                        Duration::from_secs_f32(0.2),
                        UiPositionLens {
                            start: UiRect::bottom(start_bottom),
                            end: UiRect::bottom(Val::Px(0.0)),
                        },
                    )));
                }
            }
        }
    }
}

pub fn show_hide_building_ghost(
    mut commands: Commands,
    mut button_selected_events: EventReader<ToolbarButtonSelectedEvent>,
    machine_buttons: Query<&Machine, (With<ToolbarButton>, Without<ToolbarButtonDelete>)>,
    delete_buttons: Query<&ToolbarButtonDelete, With<ToolbarButton>>,
    images: Res<Images>,
    world_mouse: Res<WorldMouse>,
    building_ghosts: Query<Entity, With<ToolGhost>>,
) {
    for &ToolbarButtonSelectedEvent(selected_option) in button_selected_events.iter() {
        for ghost_entity in building_ghosts.iter() {
            commands.entity(ghost_entity).despawn_recursive();
        }

        if let Some(selected) = selected_option {
            let tile_position = TilePosition::from_world(world_mouse.position_world);

            let ghost_entity = if let Ok(machine) = machine_buttons.get(selected) {
                let entity = machine.spawn_graphics(&mut commands, &images);

                commands.entity(entity).insert(*machine);

                Some(entity)
            } else if let Ok(_) = delete_buttons.get(selected) {
                let entity = commands
                    .spawn(SpriteBundle {
                        texture: images.delete.clone(),
                        ..default()
                    })
                    .insert(ToolbarButtonDelete)
                    .id();

                Some(entity)
            } else {
                None
            };

            if let Some(entity) = ghost_entity {
                commands
                    .entity(entity)
                    .insert(ToolGhost {
                        start_tile: tile_position,
                        end_tile: tile_position,
                    })
                    .insert(Transform::from_translation(
                        tile_position.to_world().extend(0.1),
                    ));
            }
        }
    }
}

pub fn hide_building_ghost_on_right_click(
    building_ghosts: Query<Entity, With<ToolGhost>>,
    mut button_selected_events: EventWriter<ToolbarButtonSelectedEvent>,
    mut world_mouse_events: EventReader<WorldMouseEvent>,
) {
    if !building_ghosts.is_empty() {
        for event in world_mouse_events.iter() {
            if let WorldMouseEvent::Click {
                button: MouseButton::Right,
                ..
            } = event
            {
                button_selected_events.send(ToolbarButtonSelectedEvent(None));
                break;
            }
        }
    }

    world_mouse_events.clear();
}

pub fn drag_building_ghost(
    mut commands: Commands,
    world_mouse: Res<WorldMouse>,
    mut building_ghosts: Query<(Entity, &mut ToolGhost, &Transform)>,
) {
    if let MouseButtonState::Dragging { .. } = world_mouse.button_state_middle {
        return;
    }

    let half_tile = Vec2::splat(TILE_SIZE / 2.0).extend(0.0);

    let mouse_tile_pos = TilePosition::from_world(world_mouse.position_world);

    for (entity, mut ghost, transform) in building_ghosts.iter_mut() {
        if ghost.end_tile != mouse_tile_pos {
            let start_translation = transform.translation;

            commands.entity(entity).insert(Animator::new(Tween::new(
                EaseFunction::CubicOut,
                Duration::from_secs_f32(0.1),
                TransformPositionLens {
                    start: start_translation,
                    end: half_tile + mouse_tile_pos.to_world().extend(0.0),
                },
            )));
        } else {
            ghost.start_tile = mouse_tile_pos;
        }
    }
}

pub fn ghost_place_machine(
    mut machine_place_requests: EventWriter<MachinePlaceRequest>,
    mut world_mouse_events: EventReader<WorldMouseEvent>,
    building_ghosts: Query<&Machine, With<ToolGhost>>,
) {
    if let Ok(machine) = building_ghosts.get_single() {
        for event in world_mouse_events.iter() {
            match event {
                WorldMouseEvent::Click {
                    button: MouseButton::Left,
                    position,
                } => {
                    let tile_position = TilePosition::from_world(*position);

                    machine_place_requests.send(MachinePlaceRequest {
                        machine: *machine,
                        position: tile_position,
                    });
                }

                WorldMouseEvent::Drag {
                    button: MouseButton::Left,
                    start_world,
                    end_world,
                } => {
                    let start_tile = TilePosition::from_world(*start_world);
                    let end_tile = TilePosition::from_world(*end_world);

                    let num_steps = i32::max(
                        (end_tile.x - start_tile.x).abs(),
                        (end_tile.y - start_tile.y).abs(),
                    )
                    .max(1);

                    let start_tile_vec = start_tile.to_vec();
                    let step_vec = (end_tile.to_vec() - start_tile_vec) / (num_steps as f32);

                    for i in 0..num_steps {
                        let pos_vec = start_tile_vec + step_vec * (i as f32);
                        let tile_position = TilePosition::from_vec(pos_vec);

                        machine_place_requests.send(MachinePlaceRequest {
                            machine: *machine,
                            position: tile_position,
                        });
                    }
                }

                _ => {}
            }
        }
    }
}

pub fn ghost_delete_machine(
    mut machine_delete_requests: EventWriter<MachineDeleteRequest>,
    mut world_mouse_events: EventReader<WorldMouseEvent>,
    building_ghosts: Query<&ToolbarButtonDelete, With<ToolGhost>>,
) {
    if let Ok(_) = building_ghosts.get_single() {
        for event in world_mouse_events.iter() {
            match event {
                WorldMouseEvent::Click {
                    button: MouseButton::Left,
                    position,
                } => {
                    let tile_position = TilePosition::from_world(*position);

                    machine_delete_requests.send(MachineDeleteRequest {
                        position: tile_position,
                    });
                }

                WorldMouseEvent::Drag {
                    button: MouseButton::Left,
                    start_world,
                    end_world,
                } => {
                    let start_tile = TilePosition::from_world(*start_world);
                    let end_tile = TilePosition::from_world(*end_world);

                    let num_steps = i32::max(
                        (end_tile.x - start_tile.x).abs(),
                        (end_tile.y - start_tile.y).abs(),
                    )
                    .max(1);

                    let start_tile_vec = start_tile.to_vec();
                    let step_vec = (end_tile.to_vec() - start_tile_vec) / (num_steps as f32);

                    for i in 0..num_steps {
                        let pos_vec = start_tile_vec + step_vec * (i as f32);
                        let tile_position = TilePosition::from_vec(pos_vec);

                        machine_delete_requests.send(MachineDeleteRequest {
                            position: tile_position,
                        });
                    }
                }

                _ => {}
            }
        }
    }
}

pub struct ToolbarButtonSelectedEvent(Option<Entity>);

#[derive(Component)]
pub struct MoneyDisplay;

#[derive(Component)]
pub struct MachineIcon(pub Machine);

#[derive(Component)]
pub struct MachineName(pub Machine);

#[derive(Component)]
pub struct ToolbarButton {
    pub enabled: bool,
    pub selected: bool,
}

impl Default for ToolbarButton {
    fn default() -> Self {
        ToolbarButton {
            enabled: false,
            selected: false,
        }
    }
}

#[derive(Component)]
pub struct ToolbarButtonDelete;

#[derive(Component)]
pub struct ToolGhost {
    start_tile: TilePosition,
    end_tile: TilePosition,
}
