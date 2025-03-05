use crate::{
    components::figure::{square::Square, Figure},
    events::figure::{FigureTriggerDragging, FigureTriggerUp},
    resource::figure_spawner::FigureSpawner,
    states::gameplay::StateGame,
};
use bevy::prelude::*;

impl Figure {
    pub(crate) fn call_dragging_events(
        mut event_reader: EventReader<FigureTriggerDragging>,
        mut figures: Query<(&mut Transform, &mut Figure)>,
        mut figure_spawner: ResMut<FigureSpawner>,
        time: Res<Time>,
    ) {
        for entity in event_reader.read() {
            let entity = entity.0;
            let (mut transform, mut figure) = figures.get_mut(entity).unwrap();

            Figure::upscaling_when_drag(
                &mut transform,
                time.delta_secs(),
                &mut figure.state_animation,
            );

            figure_spawner.remove_lerp_figure(entity);
        }
    }

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
}
