use crate::{prelude::LdtkLevel, resources::LdtkLevels};
pub(crate) mod handle_new_level;

use bevy::{prelude::*, sprite::Anchor};
#[derive(Component)]
pub struct LdtkLevelLoaded;

pub fn asset_events(
	mut levels: ResMut<LdtkLevels>,
	mut ev_asset: EventReader<AssetEvent<LdtkLevel>>,
	level_query: Query<(Entity, &Handle<LdtkLevel>), Without<LdtkLevelLoaded>>,
) {
	for ev in ev_asset.read() {
		debug!("ev: {ev:?}");
		if let AssetEvent::<LdtkLevel>::LoadedWithDependencies { id } = ev {
			if let Some((entity, handle)) = level_query
				.iter()
				.find(|(_entity, handle)| handle.id() == *id)
			{
				debug!("Found a matching label and entity! so exciting!");
				levels.to_load.insert((entity, handle.clone()));
			}
		}
	}
}

pub fn levels_changed(
	mut commands: Commands,
	mut levels: ResMut<LdtkLevels>,
	mut asset_server: ResMut<AssetServer>,
	level_assets: Res<Assets<LdtkLevel>>,
	mut query: Query<&mut Transform>,
) {
	debug!("resource changed: {levels:?}");

	if !levels.to_load.is_empty() {
		let to_load: Vec<_> = levels.to_load.drain().collect();
		to_load.iter().for_each(|x| {
			levels.loaded.insert(x.clone());
			finish_level(
				x.clone(),
				&mut commands,
				&mut asset_server,
				&level_assets,
				&mut query,
			);
		});
	}
}

pub(crate) fn finish_level(
	entity_handle: (Entity, Handle<LdtkLevel>),
	// mut transform: &mut Transform,
	commands: &mut Commands,
	asset_server: &mut AssetServer,
	level_assets: &Assets<LdtkLevel>,
	query: &mut Query<&mut Transform>,
) {
	let (entity, handle) = entity_handle;

	debug!("finish_level: {entity:?}");

	let Some(level) = level_assets.get(handle.id()) else {
		return;
	};

	commands
		.entity(entity)
		.insert(Name::from(level.value.identifier.to_owned()))
		.with_children(|parent| {
			parent.spawn(SpriteBundle {
				sprite: Sprite {
					// color: (),
					// flip_x: (),
					// flip_y: (),
					// custom_size: (),
					// rect: (),
					anchor: Anchor::TopLeft,
					..default()
				},
				// transform: todo!(),
				// global_transform: todo!(),
				texture: asset_server.load(
					level
						.dir
						.join(format!("png/{}_bg.png", level.value.identifier)),
				),
				// visibility: todo!(),
				// inherited_visibility: todo!(),
				// view_visibility: todo!(),
				..default()
			});
		});

	let mut transform = query.get_mut(entity).unwrap();

	transform.translation = Vec3::new(level.value.world_x as f32, -level.value.world_y as f32, 0.0);
}
