

use bevy::prelude::*;

use crate::snake_parts::SnakeSegments;

pub struct ScorePlugin;
struct ScoreText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    right:Val::Percent(50.0),
                    top: Val::Percent(0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "0".to_string(),
                    style: TextStyle {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 60.0,
                        color: Color::WHITE,
                        ..Default::default()
                    },
                }],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ScoreText);
}

fn update_score(mut score_text: Query<&mut Text, With<ScoreText>>, segments: Res<SnakeSegments>){
    for mut txt in score_text.iter_mut(){
        txt.sections[0].value = format!("{}", segments.0.len());
    }
}

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
        app.add_system(update_score.system());
    }
}
