use bevy::{input::mouse::MouseButtonInput, prelude::*, window::WindowResolution};

const WINDOWS_WIDTH: f32 = 800.;
const WINDOWS_HEIGHT: f32 = 800.;

struct BoardPlugin;

#[derive(Component)]
struct Cell;

#[derive(Resource)]
struct Board {
    width: i32,
    height: i32,
    cells: Vec<bool>,
}

impl Default for Board {
    fn default() -> Self {
        let width = 10;
        let height = 10;
        let cells = vec![false; (width * height) as usize];
        Board {
            width,
            height,
            cells,
        }
    }
}

impl Board {
    pub fn new(width: i32, height: i32) -> Self {
        let cells = vec![false; (width * height) as usize];
        Board {
            width,
            height,
            cells,
        }
    }

    pub fn get_index(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn get_cell(&self, x: i32, y: i32) -> bool {
        self.cells[self.get_index(x, y)]
    }

    pub fn set_cell(&mut self, x: i32, y: i32, value: bool) {
        let index = self.get_index(x, y);
        self.cells[index] = value;
    }

    pub fn toggle_cell(&mut self, x: i32, y: i32) {
        let index = self.get_index(x, y);
        self.cells[index] = !self.cells[index];
    }

    pub fn clear(&mut self) {
        for cell in self.cells.iter_mut() {
            *cell = false;
        }
    }

    pub fn next_generation(&self) -> Board {
        let mut next = Board::new(self.width, self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                let neighbors = self.count_neighbors(x, y);
                let index = next.get_index(x, y);
                let cell = self.get_cell(x, y);
                next.cells[index] = match (cell, neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }
        next
    }

    fn count_neighbors(&self, x: i32, y: i32) -> i32 {
        let mut count = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x + dx;
                let ny = y + dy;
                if nx >= 0 && nx < self.width && ny >= 0 && ny < self.height {
                    if self.get_cell(nx, ny) {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // Configure the main camera
    commands.spawn(Camera2dBundle::default());

    // Get window dimensions
    let window_width = WINDOWS_WIDTH;
    let window_height = WINDOWS_HEIGHT;

    // Define square size
    let square_size = 50.0;

    // Calculate the number of squares that fit in the window
    let cols = (window_width / square_size).ceil() as i32;
    let rows = (window_height / square_size).ceil() as i32;

    // Create a white material
    let sprite = Sprite {
        color: Color::WHITE,
        rect: Some(Rect::new(-30., -30., 30., 30.)),
        ..Default::default()
    };

    // Spawn squares to fill the screen
    for col in 0..cols {
        for row in 0..rows {
            commands.spawn((SpriteBundle {
                sprite: sprite.clone(),
                transform: Transform::from_translation(Vec3::new(
                    col as f32 * square_size - window_width / 2.0 + square_size / 2.0,
                    row as f32 * square_size - window_height / 2.0 + square_size / 2.0,
                    0.0,
                )),
                ..Default::default()
            }, Cell));
        }
    }
}

fn on_click(mut board: ResMut<Board>, buttons: Res<ButtonInput<MouseButton>>, windows: Query<&Window>,camera_q: Query<(&Camera, &GlobalTransform)>,  mut query: Query<(&mut Transform, &mut Sprite), With<Cell>>) {
    if buttons.just_pressed(MouseButton::Left) {
        let window = windows.single();
        let (camera, camera_transform) = camera_q.single();
        if let Some(windows_pos) = window.cursor_position().and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) {
            let mut n = 0;
            for (u, (transform, mut sprite)) in query.iter_mut().enumerate() {
                let sprite_pos = Vec2::new(transform.translation.x, transform.translation.y);
                let distance = (sprite_pos - windows_pos).length();
                if distance < 30. {
                    if sprite.color == Color::RED {
                        sprite.color = Color::WHITE;
                        board.cells[u] = false;
                    } else {
                        sprite.color = Color::RED;
                        board.cells[u] = true;
                    }
                }
                n += 1;
            }

        }
    }
}

fn next_generation(mut board: ResMut<Board>, keyboard: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Sprite, With<Cell>>) {
    if keyboard.just_pressed(KeyCode::KeyA) {
        let next = board.next_generation();
        for (u, mut sprite) in query.iter_mut().enumerate() {
            if next.cells[u] {
                sprite.color = Color::RED;
            } else {
                sprite.color = Color::WHITE;
            }
        }

        *board = next;
    }
}


impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {

        app
            .insert_resource(
                Board::new(16, 16)
            )
            .add_systems(Startup, setup)
            .add_systems(Update, (on_click, next_generation))
        ;


    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
                        primary_window: Some(
                            Window {
                                title: "Game Of Life".to_string(),
                                resolution: WindowResolution::new(WINDOWS_WIDTH, WINDOWS_HEIGHT),
                                ..default()
                            }
                        ),
                        ..Default::default()
                    }))
        .add_plugins(BoardPlugin)
        .run();
}
