use std::f32::consts::PI;

const SPEED: f32 = 1.0;
const SENSITIVITY: f32 = 1.0 / 500.0;
const PITCH_MAX: f32 = 0.01; // between 0 and 1

pub(crate) struct Camera {
    // x, y, z. X right, Y points up, Z points towards back.
    position: [f32; 3],
    // Pitch, yaw (in radians)
    rotation: [f32; 2],
}

pub(crate) enum MovementDirection {
    Up,
    Down,
    Left,
    Right,
    Front,
    Back,
}

impl Camera {
    pub(crate) fn new() -> Self {
        Camera {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0],
        }
    }

    /// Turning on a plane passing through Y
    pub fn _pitch(&self) -> f32 {
        self.rotation[0]
    }
    /// Turning around the Y axis (XZ plane)
    pub fn yaw(&self) -> f32 {
        self.rotation[1]
    }

    pub fn get_position(&self) -> [f32; 3] {
        self.position
    }
    pub fn _get_rotation(&self) -> [f32; 2] {
        self.rotation
    }

    /// Get the rotation matrix for pitch and yaw, but Column-major because that's the way glium likes it
    pub fn get_rotation_matrix(&self) -> [[f32; 3]; 3] {
        let pitch = self.rotation[0];
        let yaw = self.rotation[1];

        let cp = pitch.cos();
        let sp = pitch.sin();
        let cy = yaw.cos();
        let sy = yaw.sin();

        [
            [cy, 0.0, -sy],          // Column 0 (X basis)
            [sp * sy, cp, sp * cy],  // Column 1 (Y basis)
            [cp * sy, -sp, cp * cy], // Column 2 (Z basis)
        ]
    }
    pub fn move_in_direction(&mut self, direction: MovementDirection, delta_t: f32) {
        let movement = SPEED * delta_t;
        let mut position = self.position;
        match direction {
            MovementDirection::Up => position[1] += movement,
            MovementDirection::Down => position[1] -= movement,
            MovementDirection::Left => {
                position[0] -= movement * self.yaw().cos();
                position[2] += movement * self.yaw().sin()
            }
            MovementDirection::Right => {
                position[0] += movement * self.yaw().cos();
                position[2] -= movement * self.yaw().sin()
            }
            MovementDirection::Front => {
                position[2] -= movement * self.yaw().cos();
                position[0] -= movement * self.yaw().sin()
            }
            MovementDirection::Back => {
                position[2] += movement * self.yaw().cos();
                position[0] += movement * self.yaw().sin()
            }
        }
        self.position = position;
    }

    pub fn mouse_moved(&mut self, d_x: f32, d_y: f32) {
        let d_yaw = -d_x * SENSITIVITY;
        let d_pitch = -d_y * SENSITIVITY;

        self.rotation[0] += d_pitch;
        if self.rotation[0] < (-PI / 2.0) * (1.0 - PITCH_MAX) {
            self.rotation[0] = (-PI / 2.0) * (1.0 - PITCH_MAX);
            println!("locked d_pitch up");
        }
        if self.rotation[0] > (PI / 2.0) * (1.0 - PITCH_MAX) {
            self.rotation[0] = (PI / 2.0) * (1.0 - PITCH_MAX);
            println!("locked d_pitch down");
        }
        self.rotation[1] += d_yaw;
        self.rotation[1] %= 2.0 * PI;
    }
}
