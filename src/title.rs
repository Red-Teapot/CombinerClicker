use std::time::Duration;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::ui::Val::Percent;
use bevy_tweening::{Animator, EaseFunction, Tween, TweeningType};
use bevy_tweening::lens::{TextColorLens, TransformPositionLens};
use iyes_loopless::prelude::*;
use crate::assets::*;
use crate::{BackgroundInteraction, GameState, palette};

const FADE_OUT_TIME: f32 = 0.2;

pub fn startup_title(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("Combiner Clicker Something", TextStyle {
            font: game_assets.font.clone(),
            font_size: 64.0,
            color: palette::DARK_BLUE,
        }).with_alignment(TextAlignment {
            horizontal: HorizontalAlign::Center,
            vertical: VerticalAlign::Center,
        }),
        transform: Transform::from_xyz(0.0, 200.0, 0.0),
        ..default()
    }).insert(TitleHint);

    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("Click to start", TextStyle {
            font: game_assets.font.clone(),
            font_size: 48.0,
            color: palette::BLUE,
        }).with_alignment(TextAlignment {
            horizontal: HorizontalAlign::Center,
            vertical: VerticalAlign::Center,
        }),
        ..default()
    }).insert(Animator::new(Tween::new(
        EaseFunction::QuadraticInOut,
        TweeningType::PingPong,
        Duration::from_secs(1),
        TransformPositionLens {
            start: vec3(0.0, -10.0, 0.0),
            end: vec3(0.0, 10.0, 0.0),
        }
    ))).insert(TitleHint);

    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("Game by RedTeapot\nMade for BevyJam #2", TextStyle {
            font: game_assets.font.clone(),
            font_size: 32.0,
            color: palette::LIGHT_BROWN,
        }).with_alignment(TextAlignment {
            horizontal: HorizontalAlign::Center,
            vertical: VerticalAlign::Center,
        }),
        transform: Transform::from_xyz(0.0, -180.0, 0.0),
        ..default()
    }).insert(TitleHint);
}

pub fn handle_title_click(mut commands: Commands,
                          interactions: Query<&Interaction, With<BackgroundInteraction>>,
                          hints: Query<(Entity, &Transform, &Text), With<TitleHint>>,
                          mut fade_out: Option<ResMut<TitleFadeOut>>,
                          time: Res<Time>) {
    for interaction in interactions.iter() {
        match interaction {
            Interaction::Clicked if fade_out.is_none() => {
                let fade_out_time = Duration::from_secs_f32(FADE_OUT_TIME);

                commands.insert_resource(TitleFadeOut {
                    timer: Timer::new(fade_out_time, false),
                });
                for (hint, transform, text) in hints.iter() {
                    let mut hint_commands = commands.entity(hint);

                    let start_position: Vec3 = transform.translation;
                    let end_position = start_position + vec3(0.0, 100.0, 0.0);
                    let start_color: Color = text.sections[0].style.color;

                    hint_commands
                        .remove::<Animator<Transform>>()
                        .remove::<Animator<Text>>()
                        .insert(Animator::new(Tween::new(
                            EaseFunction::CubicOut,
                            TweeningType::Once,
                            fade_out_time,
                            TransformPositionLens {
                                start: start_position,
                                end: end_position,
                            }
                        )))
                        .insert(Animator::new(Tween::new(
                            EaseFunction::CubicOut,
                            TweeningType::Once,
                            fade_out_time,
                            TextColorLens {
                                start: start_color,
                                end: Color::NONE,
                                section: 0,
                            }
                        )));
                }
                info!("Starting title fade out");
            }

            _ => ()
        }
    }

    match fade_out {
        Some(mut fade_out) => {
            let timer = &mut fade_out.timer;
            timer.tick(time.delta());

            if timer.finished() {
                commands.remove_resource::<TitleFadeOut>();

                for (hint, _, _) in hints.iter() {
                    commands.entity(hint).despawn_recursive();
                }

                info!("Starting gameplay");
                commands.insert_resource(NextState(GameState::Gameplay));
            }
        }
        None => ()
    }
}

#[derive(Component)]
pub struct TitleHint;

pub struct TitleFadeOut {
    timer: Timer,
}
