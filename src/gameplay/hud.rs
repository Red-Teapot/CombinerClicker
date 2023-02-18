use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{
    lens::{TransformPositionLens, UiPositionLens},
    *,
};

use crate::assets::Images;

use super::{
    components::Balance,
    input::{MouseState, WorldMouse, MouseButtonState, WorldMouseEvent},
    machines::Machine,
    tile_tracked_entities::TilePosition,
    TILE_SIZE,
};

pub fn update_balance_display(
    wallet: Res<Balance>,
    ui_images: Res<Images>,
    game_images: Res<Images>,
    mut money_display: Query<&mut Text, With<MoneyDisplay>>,
    mut machine_names: Query<(&mut Text, &MachineName), Without<MoneyDisplay>>,
    mut machine_icons: Query<(&mut UiImage, &MachineIcon)>,
    mut machine_buy_buttons: Query<&mut MachineBuyButton>,
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

    for mut button in machine_buy_buttons.iter_mut() {
        button.enabled = wallet.coins >= button.machine.cost();
    }
}

pub fn select_machine_button(
    buttons: Query<(Entity, &Interaction, &MachineBuyButton), Changed<Interaction>>,
    wallet: Res<Balance>,
    mut button_selected_events: EventWriter<MachineButtonSelectedEvent>,
) {
    for (entity, interaction, button) in buttons.iter() {
        if !button.enabled {
            continue;
        }

        match interaction {
            Interaction::Clicked if !button.selected && button.machine.cost() <= wallet.coins => {
                button_selected_events.send(MachineButtonSelectedEvent(Some(entity)));
            }

            Interaction::Clicked if button.selected => {
                button_selected_events.send(MachineButtonSelectedEvent(None));
            }

            _ => (),
        }
    }
}

pub fn update_selected_machine_button(
    mut commands: Commands,
    mut buttons: Query<(Entity, &mut MachineBuyButton, &Style)>,
    mut button_selected_events: EventReader<MachineButtonSelectedEvent>,
) {
    for &MachineButtonSelectedEvent(selected_option) in button_selected_events.iter() {
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
    mut button_selected_events: EventReader<MachineButtonSelectedEvent>,
    mut world_mouse_events: EventReader<WorldMouseEvent>,
    buttons: Query<&MachineBuyButton>,
    images: Res<Images>,
    world_mouse: Res<WorldMouse>,
    building_ghosts: Query<Entity, With<BuildingGhost>>,
) {
    for &MachineButtonSelectedEvent(selected_option) in button_selected_events.iter() {
        for ghost_entity in building_ghosts.iter() {
            commands.entity(ghost_entity).despawn_recursive();
        }

        if let Some(selected) = selected_option {
            let button = buttons.get(selected).unwrap();

            let ghost_entity = button.machine.spawn_graphics(&mut commands, &images);

            let tile_position = TilePosition::from_world(world_mouse.position_world);

            commands
                .entity(ghost_entity)
                .insert(BuildingGhost {
                    machine: button.machine,
                    start_tile: tile_position,
                    end_tile: tile_position,
                })
                .insert(Transform::from_translation(
                    tile_position.to_world().extend(0.0),
                ));
        }
    }
}

pub fn hide_building_ghost_on_right_click(
    building_ghosts: Query<Entity, With<BuildingGhost>>,
    mut button_selected_events: EventWriter<MachineButtonSelectedEvent>,
    mut world_mouse_events: EventReader<WorldMouseEvent>,
) {
    if !building_ghosts.is_empty() {
        for event in world_mouse_events.iter() {
            if let WorldMouseEvent::Click { button: MouseButton::Right, .. } = event {
                button_selected_events.send(MachineButtonSelectedEvent(None));
                break;
            }
        }
    }

    world_mouse_events.clear();
}

pub fn drag_building_ghost(
    mut commands: Commands,
    world_mouse: Res<WorldMouse>,
    mut building_ghosts: Query<(Entity, &mut BuildingGhost, &Transform)>,
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

pub struct MachineButtonSelectedEvent(Option<Entity>);

#[derive(Component)]
pub struct MoneyDisplay;

#[derive(Component)]
pub struct MachineIcon(pub Machine);

#[derive(Component)]
pub struct MachineName(pub Machine);

#[derive(Component)]
pub struct MachineBuyButton {
    pub enabled: bool,
    pub selected: bool,
    pub machine: Machine,
}

impl MachineBuyButton {
    pub fn new(machine: Machine) -> MachineBuyButton {
        MachineBuyButton {
            enabled: false,
            selected: false,
            machine,
        }
    }
}

#[derive(Component)]
pub struct BuildingGhost {
    machine: Machine,
    start_tile: TilePosition,
    end_tile: TilePosition,
}
