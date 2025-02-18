use crate::{
    components::figures::{square::Square, Figure},
    events::figure::FigureTriggerUp,
    states::gameplay::StateGame,
};
use bevy::prelude::*;

pub(crate) fn start_dragging(
    trigger: Trigger<Pointer<Down>>,
    figures: Query<Entity, With<Figure>>,
    square_query: Query<&Square>,
    current_state: Res<State<StateGame>>,
    mut state_figure: ResMut<NextState<StateGame>>,
) {
    if *current_state.get() != StateGame::Idle {
        return;
    }

    if let Ok(square) = square_query.get(trigger.target) {
        if let Some(parent) = square.parent {
            if let Ok(entity) = figures.get(parent) {
                state_figure.set(StateGame::Dragging(entity));
            }
        }
    } else if let Ok(entity) = figures.get(trigger.target) {
        state_figure.set(StateGame::Dragging(entity));
    }
}

pub(crate) fn stop_dragging(
    mouse_input: Res<ButtonInput<MouseButton>>,
    touch_input: Res<Touches>,
    current_state: Res<State<StateGame>>,
    mut next_state: ResMut<NextState<StateGame>>,
    mut event_writer: EventWriter<FigureTriggerUp>,
) {
    if mouse_input.just_released(MouseButton::Left) || touch_input.any_just_released() {
        if let StateGame::Dragging(figure) = current_state.get() {
            next_state.set(StateGame::Placing(*figure));
            event_writer.send(FigureTriggerUp(*figure));
        }
    }
}
