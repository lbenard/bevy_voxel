use crate::world::World;
use bevy::prelude::*;
use bevy_spectator::SpectatorSystemSet;

use crate::world::raycast::cast;

#[derive(Clone, Copy)]
pub struct RaycastResult {
    pub position: IVec3,
}

#[derive(Resource, Default)]
pub struct Raycast {
    pub result: Option<RaycastResult>,
}

pub struct RaycastPlugin;

#[derive(Component)]
struct Cursor;

impl Plugin for RaycastPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Raycast>()
            .add_systems(Startup, Self::setup_cursor)
            .add_systems(
                Update,
                (Self::raycast, Self::render_box, Self::render_cursor)
                    .chain()
                    .after(SpectatorSystemSet), // Run after camera update to avoid the raycast to lag behind one frame
            );
    }
}

impl RaycastPlugin {
    fn setup_cursor(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn((
            Cursor,
            NodeBundle {
                style: Style {
                    width: Val::Px(32.0),
                    height: Val::Px(32.0),
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    ..default()
                },
                // a `NodeBundle` is transparent by default, so to see the image we have to its color to `WHITE`
                background_color: Color::WHITE.into(),
                ..default()
            },
            UiImage::new(asset_server.load("ui/crosshair.png")),
        ));
    }

    fn raycast(
        camera: Query<(&Camera, &Transform)>,
        world: Res<World>,
        mut raycast: ResMut<Raycast>,
    ) {
        for (_camera, transform) in &camera {
            let direction = transform.forward();
            let result = cast(&world, transform.translation, 100.0, direction);
            if let Some(result) = result {
                *raycast = Raycast {
                    result: Some(RaycastResult { position: result.0 }),
                }
            } else {
                *raycast = Raycast::default();
            }
        }
    }

    fn render_box(mut gizmos: Gizmos, raycast: Res<Raycast>) {
        let Some(raycast) = &raycast.result else { return };

        gizmos.cuboid(
            Transform::from_translation(raycast.position.as_vec3() + Vec3::new(0.5, 0.5, 0.5))
                .with_scale(Vec3::splat(1.)),
            Color::BLACK,
        );
    }

    fn render_cursor(mut cursor: Query<(&mut Visibility,), With<Cursor>>, raycast: Res<Raycast>) {
        let visibility = if raycast.result.is_some() {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        *cursor.single_mut().0 = visibility;
    }
}
