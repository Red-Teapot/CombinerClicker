use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{*, lens::UiPositionLens};

use crate::assets::Images;

use super::{machines::Machine, components::{Balance}};

pub fn update_wallet(
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
    text.sections[0].value = wallet.money.to_string();

    for (mut text, MachineName(machine)) in machine_names.iter_mut() {
        if wallet.money >= machine.cost() {
            text.sections[0].value = machine.name().to_string();
        }
    }

    for (mut image, MachineIcon(machine)) in machine_icons.iter_mut() {
        if wallet.money >= machine.cost() {
            image.0 = machine.image(&game_images);
        } else {
            image.0 = ui_images.locked.clone();
        }
    }

    for mut button in machine_buy_buttons.iter_mut() {
        button.enabled = wallet.money >= button.machine.cost();
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
            Interaction::Clicked if !button.selected && button.machine.cost() <= wallet.money => {
                // TODO: Add the building ghost
                button_selected_events.send(MachineButtonSelectedEvent(Some(entity)));
            }

            Interaction::Clicked if button.selected => {
                // TODO: Remove the building ghost
                button_selected_events.send(MachineButtonSelectedEvent(None));
            }

            _ => ()
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
                },

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
                },
            }
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