use crate::{
    components::figure::{square::SquareComponent, FigureComponent},
    events::figure::{FigureTriggerDraggingEvent, FigureTriggerUpEvent},
    resource::figure_spawner::FigureSpawnerResource,
    states::gameplay::GameState,
};
use bevy::prelude::*;

impl FigureComponent {
    pub(crate) fn call_dragging_events(
        mut event_reader: EventReader<FigureTriggerDraggingEvent>,
        mut figures: Query<(&mut Transform, &mut FigureComponent)>,
        mut figure_spawner: ResMut<FigureSpawnerResource>,
        time: Res<Time>,
    ) {
        for entity in event_reader.read() {
            let entity = entity.0;
            let (mut transform, mut figure) = figures.get_mut(entity).unwrap();

            FigureComponent::upscaling_when_drag(
                &mut transform,
                time.delta_secs(),
                &mut figure.state_animation,
            );

            figure_spawner.remove_lerp_figure(entity);

            trace!("Removed lerp figure {:?}", entity);
        }
    }

    pub(crate) fn start_dragging(
        trigger: Trigger<Pointer<Down>>,
        figures: Query<Entity, With<FigureComponent>>,
        square_query: Query<&SquareComponent>,
        current_state: Res<State<GameState>>,
        mut state_figure: ResMut<NextState<GameState>>,
    ) {
        if *current_state.get() != GameState::Idle {
            return;
        }

        if let Ok(square) = square_query.get(trigger.target) {
            if let Some(parent) = square.parent {
                if let Ok(entity) = figures.get(parent) {
                    state_figure.set(GameState::Dragging(entity));
                    trace!("Next state: {:?}", state_figure);
                }
            }
        } else if let Ok(entity) = figures.get(trigger.target) {
            state_figure.set(GameState::Dragging(entity));
            trace!("Next state: {:?}", state_figure);
        }
    }

    pub(crate) fn stop_dragging(
        mouse_input: Res<ButtonInput<MouseButton>>,
        touch_input: Res<Touches>,
        current_state: Res<State<GameState>>,
        mut next_state: ResMut<NextState<GameState>>,
        mut event_writer: EventWriter<FigureTriggerUpEvent>,
    ) {
        if mouse_input.just_released(MouseButton::Left) || touch_input.any_just_released() {
            if let GameState::Dragging(figure) = current_state.get() {
                next_state.set(GameState::Placing(*figure));
                trace!("Next state: {:?}", next_state);

                event_writer.send(FigureTriggerUpEvent(*figure));
                trace!("FigureTriggerUp sent");
            }
        }
    }
}
