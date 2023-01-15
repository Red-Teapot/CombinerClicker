use super::components::*;
use super::*;
use crate::assets::*;
use crate::utils::kayak::KOffsetLens;
use crate::{palette, BackgroundInteraction, GameState};
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy_tweening::lens::UiPositionLens;
use bevy_tweening::{Animator, EaseFunction, RepeatCount, RepeatStrategy, Tween};
use iyes_loopless::prelude::*;
use kayak_ui::{prelude::*, widgets::*};
use std::time::Duration;

pub fn startup_title(
    mut commands: Commands,
    ui_fonts: Res<Fonts>,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    font_mapping.set_default(asset_server.load("varela-round/VarelaRound-Regular.kayak_font"));

    let mut widget_context = KayakRootContext::new();
    widget_context.add_plugin(KayakWidgetsContextPlugin);
    let parent_id = None;

    rsx! {
        <KayakAppBundle>
            <ElementBundle
                styles={KStyle {
                    top: StyleProp::Value(Units::Pixels(0.0)),
                    bottom: StyleProp::Value(Units::Pixels(0.0)),
                    left: StyleProp::Value(Units::Pixels(0.0)),
                    right: StyleProp::Value(Units::Pixels(0.0)),
                    position_type: StyleProp::Value(KPositionType::SelfDirected),
                    ..default()
                }}
            >
                <TextWidgetBundle
                    text={TextProps {
                        content: "One Clicker".to_string(),
                        size: 64.0,
                        alignment: Alignment::Middle,
                        ..default()
                    }}

                    styles={KStyle {
                        color: StyleProp::Value(palette::DARK_BLUE),
                        top: StyleProp::Value(Units::Pixels(16.0)),
                        bottom: StyleProp::Value(Units::Stretch(1.0)),
                        ..default()
                    }}
                />

                {
                    let text = rsx!{
                        <TextWidgetBundle
                            text={TextProps {
                                content: "Click to start".to_string(),
                                size: 48.0,
                                alignment: Alignment::Middle,
                                ..default()
                            }}

                            styles={KStyle {
                                color: StyleProp::Value(palette::BLUE),
                                ..default()
                            }}
                        />
                    };

                    commands.entity(text)
                        .insert(Animator::new(Tween::new(
                            EaseFunction::QuadraticInOut,
                            Duration::from_secs_f32(1.0),
                            KOffsetLens {
                                start: Edge::new(Units::Pixels(-32.0), Units::Auto, Units::Auto, Units::Auto),
                                end: Edge::new(Units::Pixels(32.0), Units::Auto, Units::Auto, Units::Auto),
                            }
                        ).with_repeat_strategy(RepeatStrategy::MirroredRepeat).with_repeat_count(RepeatCount::Infinite)));

                    children.add(text);
                }

                <TextWidgetBundle
                    text={TextProps {
                        content: "Game by RedTeapot\nMade for Bevy Jam #2".to_string(),
                        size: 32.0,
                        alignment: Alignment::Middle,
                        ..default()
                    }}

                    styles={KStyle {
                        color: StyleProp::Value(palette::LIGHT_BROWN),
                        top: StyleProp::Value(Units::Stretch(1.0)),
                        bottom: StyleProp::Value(Units::Pixels(16.0)),
                        ..default()
                    }}
                />
            </ElementBundle>

            <ElementBundle
                styles={KStyle {
                    top: StyleProp::Value(Units::Pixels(0.0)),
                    bottom: StyleProp::Value(Units::Pixels(0.0)),
                    left: StyleProp::Value(Units::Pixels(0.0)),
                    right: StyleProp::Value(Units::Pixels(0.0)),
                    position_type: StyleProp::Value(KPositionType::SelfDirected),
                    ..default()
                }}
            >
                <TextWidgetBundle
                    text={TextProps {
                        content: format!("Version {} (post-jam)", env!("CARGO_PKG_VERSION")),
                        size: 32.0,
                        alignment: Alignment::Start,
                        ..default()
                    }}

                    styles={KStyle {
                        color: StyleProp::Value(palette::LIGHT_BLUE),
                        top: StyleProp::Value(Units::Stretch(1.0)),
                        left: StyleProp::Value(Units::Pixels(8.0)),
                        bottom: StyleProp::Value(Units::Pixels(8.0)),
                        ..default()
                    }}
                />
            </ElementBundle>

            <BackgroundBundle
                styles={KStyle {
                    background_color: StyleProp::Value(*palette::OFF_WHITE.clone().set_a(0.0)),
                    top: StyleProp::Value(Units::Pixels(0.0)),
                    bottom: StyleProp::Value(Units::Pixels(0.0)),
                    left: StyleProp::Value(Units::Pixels(0.0)),
                    right: StyleProp::Value(Units::Pixels(0.0)),
                    z_index: StyleProp::Value(1),
                    ..default()
                }}
            />
        </KayakAppBundle>
    };

    commands
        .spawn(UICameraBundle::new(widget_context))
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
                let fade_out_time = Duration::from_secs_f32(TRANSITION_TIME);

                commands.insert_resource(TitleFadeOut {
                    timer: Timer::new(fade_out_time, TimerMode::Once),
                });

                for hint in hints.iter() {
                    let mut hint_commands = commands.entity(hint);

                    hint_commands.insert(Animator::new(Tween::new(
                        EaseFunction::CubicOut,
                        fade_out_time,
                        KOffsetLens {
                            start: Edge::new(
                                Units::Percentage(0.0),
                                Units::Auto,
                                Units::Auto,
                                Units::Auto,
                            ),
                            end: Edge::new(
                                Units::Percentage(-100.0),
                                Units::Auto,
                                Units::Auto,
                                Units::Auto,
                            ),
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
