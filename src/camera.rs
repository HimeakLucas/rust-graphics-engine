use cgmath::{InnerSpace, Matrix4, Point3, Vector3, Deg, Angle};

const WORLD_UP: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);

pub struct Camera {

    pub position: Point3<f32>,
    pub yaw: f32,
    pub pitch: f32,

    front: Vector3<f32>,
    right: Vector3<f32>,
    up : Vector3<f32>,


    pub speed: f32,
    pub sensitivity: f32,
}

impl Camera {

    pub fn new(position: Point3<f32>, yaw: f32, pitch: f32) -> Self {

        let mut camera = Camera {
            position,
            yaw,
            pitch,
            front: Vector3::new(0.0, 0.0, -1.0),
            right: Vector3::new(0.0, 0.0,  0.0),
            up   : Vector3::new(0.0, 1.0,  0.0),
            speed: 2.5,
            sensitivity: 0.1,
        };
        camera.update_vectors();
        camera
    }
    
    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(self.position, self.position + self.front, self.up)
    }

    pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f32) {

        let velocity = self.speed * delta_time;
        match direction {
            CameraMovement::Forward => self.position += self.front * velocity,
            CameraMovement::Backward => self.position -= self.front * velocity,
            CameraMovement::Left => self.position -= self.right * velocity,
            CameraMovement::Right => self.position += self.right * velocity,
            CameraMovement::Up => self.position += WORLD_UP * velocity,
            CameraMovement::Down => self.position -= WORLD_UP * velocity,
        }

    }

    pub fn process_mouse(&mut self, mut x_offset: f32, mut y_offset: f32) {

        x_offset *= self.sensitivity;
        y_offset *= self.sensitivity;

        self.yaw += x_offset;
        self.pitch += y_offset;
        
        if self.pitch >  89.0 { self.pitch =  89.0; }
        if self.pitch < -89.0 { self.pitch = -89.0}

        self.update_vectors();
    }
    
    fn update_vectors(&mut self) {
            let front = Vector3::new(
                Deg(self.yaw).cos() * Deg(self.pitch).cos(),
                Deg(self.pitch).sin(),
                Deg(self.yaw).sin() * Deg(self.pitch).cos(),
            );
            self.front = front.normalize();
            self.right = self.front.cross(WORLD_UP).normalize();
            self.up = self.right.cross(self.front).normalize();
        }
}



pub enum CameraMovement {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}

