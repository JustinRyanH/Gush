use cgmath::{Matrix4};

pub struct Camera{
    position: [f32; 3],
    direction: [f32; 3],
    target: [f32; 3],
    cached: Matrix4<f32>,
}

impl Camera {
    /// Create a new camera abstraction
    pub fn new() -> Camera {
        let position =  [0., 0., -3.];
        let direction = [0., 1., 0.];
        let target = [0., 0., 0.];
        let cached = Matrix4::look_at(position.into() , target.into(), direction.into());
        Camera {
            position,
            direction,
            target,
            cached,
        }
    }

    pub fn look_at(&mut self, target: [f32; 3]) {
        self.target = target;
        self.cached = Matrix4::look_at(self.position.into(), target.into(), self.direction.into())
    }

    pub fn move_to(&mut self, position: [f32; 3]) {
        self.position = position;
        self.cached = Matrix4::look_at(self.position.into(), self.target.into(), self.direction.into());
    }

    pub fn move_by(&mut self, position: [f32; 3]) {
        self.position = [self.position[0] + position[0], self.position[1] + position[1], self.position[2] + position[2]];
        self.cached = Matrix4::look_at(self.position.into(), self.target.into(), self.direction.into());
    }

    pub fn set_direction(&mut self, direction: [f32; 3]) {
        self.direction = direction;
        self.cached = Matrix4::look_at(self.position.into(), self.target.into(), self.direction.into());
    }

    pub fn as_matrix(&self) -> Matrix4<f32> {
        self.cached
    }
}
