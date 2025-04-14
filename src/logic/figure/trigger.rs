use crate::{
    components::figure::{square::SquareComponent, FigureComponent},
    events::figure::{FigureTriggerDraggingEvent, FigureTriggerUpEvent},
    resource::figure_spawner::FigureSpawnerResource,
    states::gameplay::GameState,
};
use bevy::prelude::*;

impl FigureComponent {
    /// Processes figure dragging events by upscaling the dragged figure and removing its lerp effect.
    ///
    /// This function iterates through all received [`FigureTriggerDraggingEvent`] events. For each event,
    /// it performs the following steps:
    ///
    /// 1. Retrieves the figure entity from the event.
    /// 2. Obtains mutable references to the figure's [`Transform`] and [`FigureComponent`] from the query.
    /// 3. Calls [`FigureComponent::upscaling_when_drag`] to update the transform based on the elapsed time
    ///    (using `time.delta_secs()`) and the current dragging animation state.
    /// 4. Removes the figure from the lerp tracking in the [`FigureSpawnerResource`] by calling
    ///    [`FigureSpawnerResource::remove_lerp_figure`].
    /// 5. Logs a trace message indicating the removal of the lerp figure.
    ///
    /// # Parameters
    ///
    /// - `event_reader`: An [`EventReader`] for [`FigureTriggerDraggingEvent`] events.
    /// - `figures`: A query that provides mutable access to the [`Transform`] and [`FigureComponent`] of figures.
    /// - `figure_spawner`: A mutable reference to the [`FigureSpawnerResource`] managing figure lerping.
    /// - `time`: A resource providing the elapsed time since the last update.
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

    /// Initiates dragging by checking pointer down events and updating the game state.
    ///
    /// This function is triggered by a pointer down event (e.g., mouse click or touch) and checks whether
    /// the event's target is associated with a square or a figure. The steps include:
    ///
    /// 1. Verifying that the current game state is [`GameState::Idle`]. If not, the function returns immediately.
    /// 2. Attempting to retrieve a [`SquareComponent`] from the event's target.
    ///    - If successful, it checks if the square has a parent entity. If the parent is a valid figure entity,
    ///      the state is updated to [`GameState::Dragging`] with that figure.
    /// 3. If the target does not correspond to a square, it checks whether the target itself is a figure.
    ///    - If so, the state is updated to [`GameState::Dragging`] with that entity.
    ///
    /// The function uses the [`NextState<GameState>`] resource to set the upcoming game state.
    ///
    /// # Parameters
    ///
    /// - `trigger`: A [`Trigger<Pointer<Down>>`] event that contains the target entity of the pointer down event.
    /// - `figures`: A query returning entities that have a [`FigureComponent`].
    /// - `square_query`: A query providing access to [`SquareComponent`] data.
    /// - `current_state`: A resource holding the current [`GameState`]. Dragging is only initiated if the state is [`GameState::Idle`].
    /// - `state_figure`: A mutable resource to set the next game state.
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

    /// Stops dragging based on input release events.
    ///
    /// This function listens for input events that indicate the end of a dragging interaction, specifically:
    /// - A release of the left mouse button.
    /// - Any touch input being just released.
    ///
    /// When one of these events is detected, if the current game state is [`GameState::Dragging`], the function:
    ///
    /// 1. Transitions the game state to [`GameState::Placing`] for the dragged figure.
    /// 2. Emits a [`FigureTriggerUpEvent`] for the figure, signaling that dragging has ended.
    /// 3. Logs the state transition and event emission via trace messages.
    ///
    /// # Parameters
    ///
    /// - `mouse_input`: A resource containing the state of mouse buttons.
    /// - `touch_input`: A resource containing the current touch input state.
    /// - `current_state`: A resource representing the current [`GameState`].
    /// - `next_state`: A mutable resource for setting the next [`GameState`].
    /// - `event_writer`: An event writer used to emit [`FigureTriggerUpEvent`] events.
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
