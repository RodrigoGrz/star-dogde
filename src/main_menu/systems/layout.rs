use bevy::prelude::*;

use crate::main_menu::components::{MainMenu, PlayButton};
use crate::main_menu::styles::*;

pub fn spawn_main_menu(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    let main_menu_entity = build_main_main(&mut commands, &asset_server);
    
}

pub fn despawn_main_menu(
    mut commands: Commands, 
    main_menu_query: Query<Entity, With<MainMenu>>,
) {
    if let Ok(main_menu_entity) = main_menu_query.single() {
        commands.entity(main_menu_entity).despawn();
    }
}

pub fn build_main_main(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>
) -> Entity {
    let main_menu_entity: Entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(8.0),
                row_gap: Val::Px(8.0),
                ..default()
            },
            MainMenu {}
        ))
    .with_children(|parent| {
        // Title
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Px(300.0),                     
                height: Val::Px(120.0),
                ..default()
            }
        )).with_children(|parent| {
            // Image 1
            parent.spawn((
                ImageNode::new(asset_server.load("sprites/ball_blue_large.png")),
                Node {
                    width: Val::Px(64.0),
                    height: Val::Px(64.0),
                    ..default()
                },
            ));
            // Text
            parent.spawn((
                Text::new("Bevy Ball Game"),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::new(JustifyText::Center, LineBreak::NoWrap),
            ));
            // Image 2
            parent.spawn((
                ImageNode::new(asset_server.load("sprites/ball_red_large.png")),
                Node {
                    width: Val::Px(64.0),
                    height: Val::Px(64.0),
                    ..default()
                },
            ));
        });
        // Play Button
        parent
            .spawn((
                Node {
                    width: Val::Px(200.0),                     
                    height: Val::Px(80.0),
                    // padding: Val::Px(200.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },

                BackgroundColor(NORMAL_BUTTON_COLOR),

                Interaction::default(),

                PlayButton {},
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("Play"),

                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 32.0,
                        ..default()
                    },

                    TextColor(Color::WHITE),
                ));
            });

        // Quit Button
        parent
            .spawn((
                Node {
                    width: Val::Px(200.0),                     
                    height: Val::Px(80.0),
                    // padding: Val::Px(200.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },

                BackgroundColor(NORMAL_BUTTON_COLOR),

                Interaction::default(),

                PlayButton {},
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("Quit"),

                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 32.0,
                        ..default()
                    },

                    TextColor(Color::WHITE),
                ));
            });
    })
    .id();

    main_menu_entity
}