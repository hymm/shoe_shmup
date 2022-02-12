use bevy::app::AppExit;
use bevy::prelude::*;

use crate::loading::FontAssets;
use crate::menu::{ButtonColors, ButtonInteraction};
use crate::GameState;

#[derive(Component)]
struct PauseMenu;

#[derive(Component)]
struct ExitButton;
#[derive(Component)]
struct CloseButton;

fn setup_pause_menu(
    mut commands: Commands,
    button_colors: Res<ButtonColors>,
    font_assets: Res<FontAssets>,
) {
    let node_entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(PauseMenu)
        .id();

    commands.entity(node_entity).with_children(|parent| {
        let button_bundle = ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: button_colors.normal,
            ..Default::default()
        };

        let text_style = TextStyle {
            font: font_assets.fira_sans.clone(),
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        };
        parent
            .spawn_bundle(button_bundle.clone())
            .insert(ExitButton)
            .insert(PauseMenu)
            .with_children(|parent| {
                parent
                    .spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "Exit".to_string(),
                                style: text_style.clone(),
                            }],
                            alignment: Default::default(),
                        },
                        ..Default::default()
                    })
                    .insert(PauseMenu);
            });

        parent
            .spawn_bundle(button_bundle.clone())
            // .insert(CloseButton)
            .insert(PauseMenu)
            .with_children(|parent| {
                parent
                    .spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "Save".to_string(),
                                style: text_style.clone(),
                            }],
                            alignment: Default::default(),
                        },
                        ..Default::default()
                    })
                    .insert(PauseMenu);
            });

        parent
            .spawn_bundle(button_bundle.clone())
            .insert(CloseButton)
            .insert(PauseMenu)
            .with_children(|parent| {
                parent
                    .spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "Close".to_string(),
                                style: text_style.clone(),
                            }],
                            alignment: Default::default(),
                        },
                        ..Default::default()
                    })
                    .insert(PauseMenu);
            });
    });
}

fn despawn_pause_menu(mut commands: Commands, q: Query<Entity, With<PauseMenu>>) {
    for e in q.iter() {
        commands.entity(e).despawn();
    }
}

fn pause(keyboard_input: Res<Input<KeyCode>>, mut game_state: ResMut<State<GameState>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Paused).unwrap();
    }
}

fn hover_button(
    button_colors: Res<ButtonColors>,
    mut interaction_query: Query<ButtonInteraction, (Changed<Interaction>, With<Button>)>,
) {
    for (_button, interaction, mut color, _children) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                *color = button_colors.hovered;
            }
            Interaction::None => {
                *color = button_colors.normal;
            }
            _ => {}
        }
    }
}

fn click_exit_button(
    mut interaction_query: Query<ButtonInteraction, (Changed<Interaction>, With<ExitButton>)>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (_button, interaction, _color, _children) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                app_exit_events.send(AppExit);
            }
            _ => {}
        }
    }
}

fn click_close_button(
    mut interaction_query: Query<ButtonInteraction, (Changed<Interaction>, With<CloseButton>)>,
    mut game_state: ResMut<State<GameState>>,
) {
    for (_button, interaction, mut _color, _children) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                game_state.set(GameState::Playing).unwrap();
            }
            _ => {}
        }
    }
}

pub struct PauseMenuPlugin;
impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Paused).with_system(setup_pause_menu))
            .add_system_set(
                SystemSet::on_update(GameState::Paused)
                    .with_system(click_exit_button)
                    .with_system(hover_button)
                    .with_system(click_close_button),
            )
            .add_system_set(SystemSet::on_exit(GameState::Paused).with_system(despawn_pause_menu))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(pause));
    }
}
