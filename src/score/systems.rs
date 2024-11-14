use bevy::prelude::*;

use super::components::{GainScoreEvent, Score};

pub fn display_score_gain_system(
    mut ev_gain_score: EventReader<GainScoreEvent>,
    mut query: Query<(&mut Score, &mut Text)>,
) {
    for GainScoreEvent(amount) in ev_gain_score.read() {
        for (mut score, mut text) in query.iter_mut() {
            score.0 += amount;
            text.sections[0].value = format!("Score: {} (Last gain {})", score.0, amount);
        }
    }
}
