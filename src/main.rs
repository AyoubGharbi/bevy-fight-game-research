use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, animate_player)
        .add_systems(Update, move_player)
        .run();
}

#[derive(Component)]
struct Player {
    animation_indices: AnimationIndices,
    animation_timer: AnimationTimer,
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("player-idle/player-idle-48x48.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(48.0, 48.0), 10, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let anim_indices = AnimationIndices { first: 1, last: 9 };

    commands.spawn(SpriteSheetBundle {
        texture,
        atlas: TextureAtlas {
            layout: texture_atlas_layout,
            index: anim_indices.first,
        },
        transform: Transform::from_scale(Vec3::splat(6.0)),
        ..default()
    })
        .insert(
            Player {
                animation_indices: anim_indices,
                animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            }
        );
}


fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut Player, &mut TextureAtlas)>,
) {
    for (mut player, mut texture_atlas) in &mut query {
        player.animation_timer.tick(time.delta());
        if player.animation_timer.just_finished() {
            texture_atlas.index = if texture_atlas.index == player.animation_indices.last {
                player.animation_indices.first
            } else {
                texture_atlas.index + 1
            };
        }
    }
}

fn move_player(mut query: Query<&mut Transform, With<Player>>) {
    for mut transform in &mut query {
        transform.translation.x += 0.1;
    }
}

