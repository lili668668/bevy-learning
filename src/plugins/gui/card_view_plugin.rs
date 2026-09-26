use bevy::prelude::*;
use game_core::action::Action;
use game_core::event::GameEvent;
use crate::resources::*;
use crate::events::*;
use crate::components::*;

pub struct CardViewPlugin;

impl Plugin for CardViewPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CardViews>()
            .add_observer(render_cards);
    }
}

fn render_cards(
    event: On<CoreEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut views: ResMut<CardViews>,
    mut sprite_query: Query<&mut Sprite>,
) {
    match &event.0 {
        GameEvent::Dealt(ids) => {
            for (_, entity) in views.0.drain() {
                commands.entity(entity).despawn();
            }
            for (index, id) in ids.iter().enumerate() {
                let entity = commands.spawn((
                    Sprite {
                        image: asset_server.load("images/card.png"),
                        custom_size: Some(Vec2::new(100.0, 150.0)),
                        ..default()
                    },
                    Transform::from_xyz(-180.0 + (index as f32 * 120.0), 0.0, 0.0),
                    Pickable::default(),
                    CardView(*id),
                )).observe(on_card_click).id();
                views.0.insert(*id, entity);
            }
        }
        GameEvent::Placed(id) => {
            let Some(entity) = views.0.get(id) else { return; };
            let Ok(mut sprite) = sprite_query.get_mut(*entity) else { return; };
            sprite.color = bevy::color::palettes::css::GRAY.into();
        }
        GameEvent::Captured { hand_card, table_card } => {
            for id in [hand_card, table_card] {
                let Some(entity) = views.0.remove(id) else { continue; };
                commands.entity(entity).despawn();
            }
        }
        _ => {}
    }
}

fn on_card_click (
    click: On<Pointer<Click>>,
    mut commands: Commands,
    view_query: Query<&CardView>,
) {
    if click.event.button != PointerButton::Primary { return; }

    let Ok(view) = view_query.get(click.event_target()) else { return; };
    commands.trigger(PlayerAction(Action::Play(view.0)));
}
