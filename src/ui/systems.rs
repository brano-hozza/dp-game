use bevy::prelude::*;

use crate::score::components::Score;

pub fn setup(mut commands: Commands) {
    let mut root_node = commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        ..default()
    });

    root_node.with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "Score: 0",
                TextStyle {
                    font_size: 30.0,
                    ..default()
                },
            ),
            Label,
            Score(0),
        ));
    });
}
