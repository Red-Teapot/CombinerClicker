use super::components::*;
use super::*;
use crate::assets::*;
use crate::{palette, BackgroundInteraction, GameState};
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy_tweening::lens::UiPositionLens;
use bevy_tweening::{Animator, EaseFunction, RepeatCount, RepeatStrategy, Tween};
use iyes_loopless::prelude::*;
use std::time::Duration;

pub fn startup_title(mut commands: Commands, ui_fonts: Res<Fonts>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                ..default()
            },
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .with_children(|container| {
            container.spawn(TextBundle {
                text: Text::from_section(
                    "One Clicker",
                    TextStyle {
                        font: ui_fonts.varela.clone(),
                        font_size: 64.0,
                        color: palette::DARK_BLUE,
                    },
                )
                .with_alignment(TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    vertical: VerticalAlign::Center,
                }),
                transform: Transform::from_xyz(0.0, 200.0, 0.0),
                ..default()
            });

            container
                .spawn(TextBundle {
                    text: Text::from_section(
                        "Click to start",
                        TextStyle {
                            font: ui_fonts.varela.clone(),
                            font_size: 48.0,
                            color: palette::BLUE,
                        },
                    )
                    .with_alignment(TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        vertical: VerticalAlign::Center,
                    }),
                    ..default()
                })
                .insert(Animator::new(
                    Tween::new(
                        EaseFunction::QuadraticInOut,
                        Duration::from_secs(1),
                        UiPositionLens {
                            start: UiRect::top(Val::Px(-20.0)),
                            end: UiRect::top(Val::Px(20.0)),
                        },
                    )
                    .with_repeat_strategy(RepeatStrategy::MirroredRepeat)
                    .with_repeat_count(RepeatCount::Infinite),
                ));

            container
                .spawn(TextBundle {
                    text: Text::from_section(
                        "Game by RedTeapot\nMade for Bevy Jam #2",
                        TextStyle {
                            font: ui_fonts.varela.clone(),
                            font_size: 32.0,
                            color: palette::LIGHT_BROWN,
                        },
                    )
                    .with_alignment(TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        vertical: VerticalAlign::Center,
                    }),
                    transform: Transform::from_xyz(0.0, -180.0, 0.0),
                    ..default()
                })
                .insert(TitleHint);

            let version_str = format!("Version {} (post-jam)", env!("CARGO_PKG_VERSION"));
            container.spawn(TextBundle {
                text: Text::from_section(
                    version_str.as_str(),
                    TextStyle {
                        font: ui_fonts.varela.clone(),
                        font_size: 32.0,
                        color: palette::LIGHT_BLUE,
                    },
                ),
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect::new(
                        Val::Px(8.0),
                        Val::Undefined,
                        Val::Undefined,
                        Val::Px(8.0),
                    ),
                    ..default()
                },
                focus_policy: FocusPolicy::Pass,
                ..default()
            });
        })
        .insert(TitleHint);
}

pub fn handle_title_click(
    mut commands: Commands,
    interactions: Query<&Interaction, With<BackgroundInteraction>>,
    hints: Query<Entity, With<TitleHint>>,
    fade_out: Option<ResMut<TitleFadeOut>>,
    time: Res<Time>,
) {
    for interaction in interactions.iter() {
        match interaction {
            Interaction::Clicked if fade_out.is_none() => {
                let fade_out_time = Duration::from_secs_f32(FADE_OUT_TIME);

                commands.insert_resource(TitleFadeOut {
                    timer: Timer::new(fade_out_time, TimerMode::Once),
                });

                for hint in hints.iter() {
                    let mut hint_commands = commands.entity(hint);

                    hint_commands.insert(Animator::new(Tween::new(
                        EaseFunction::CubicOut,
                        fade_out_time,
                        UiPositionLens {
                            start: UiRect::left(Val::Percent(0.0)),
                            end: UiRect::left(Val::Percent(-100.0)),
                        },
                    )));
                }
            }

            _ => (),
        }
    }

    match fade_out {
        Some(mut fade_out) => {
            let timer = &mut fade_out.timer;
            timer.tick(time.delta());

            if timer.finished() {
                commands.remove_resource::<TitleFadeOut>();

                for hint in hints.iter() {
                    commands.entity(hint).despawn_recursive();
                }

                commands.insert_resource(NextState(GameState::Gameplay));
            }
        }
        None => (),
    }
}
