use crate::engine::core::vec3d::Vec3d;

pub struct Camera {
    znear: f32,
    zfar: f32,
    field_of_view: f32,
    position: Vec3d,
    direction: Vec3d,
    up: Vec3d,
    inputs: Vec<String>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            znear: 0.1,
            zfar: 1024.0,
            field_of_view: 3.141592 / 2.5,
            position: Vec3d(0.0, 0.0, 0.0),
            direction: Vec3d(0.0, 0.0, 1.0),
            up: Vec3d(0.0, 1.0, 0.0),
            inputs: vec![],
        }
    }

    pub fn set_inputs(&mut self, pressed: &Vec<String>, released: &Vec<String>) {
        for press in pressed {
            if !self.inputs.contains(press) {
                self.inputs.push(press.to_string());
            }

            let angle: f32 = 3.14 / 2.5;

            if press == &String::from("Up") {
                let cross = self.direction.cross_product(self.up);
                self.direction = self.direction * angle.cos()
                    + cross.cross_product(self.direction) * angle.cos()
                    + cross * cross.dot_product(self.direction) * (1.0 - angle.cos());
                self.up = self.up * angle.cos()
                    + cross.cross_product(self.up) * angle.cos()
                    + cross * cross.dot_product(self.up) * (1.0 - angle.cos());
                self.direction.normalize();
                self.up.normalize();
            } else if press == &String::from("Down") {
                let cross = self.up.cross_product(self.direction);
                self.direction = self.direction * angle.cos()
                    + cross.cross_product(self.direction) * angle.cos()
                    + cross * cross.dot_product(self.direction) * (1.0 - angle.cos());
                self.up = self.up * angle.cos()
                    + cross.cross_product(self.up) * angle.cos()
                    + cross * cross.dot_product(self.up) * (1.0 - angle.cos());
                self.direction.normalize();
                self.up.normalize();
            } else if press == &String::from("Left") {
                let cross = self.up * -1.0;
                self.direction = self.direction * angle.cos()
                    + cross.cross_product(self.direction) * angle.cos()
                    + cross * cross.dot_product(self.direction) * (1.0 - angle.cos());
                self.direction.normalize();
            } else if press == &String::from("Right") {
                let cross = self.up;
                self.direction = self.direction * angle.cos()
                    + cross.cross_product(self.direction) * angle.cos()
                    + cross * cross.dot_product(self.direction) * (1.0 - angle.cos());
                self.direction.normalize();
            }
        }
        for release in released {
            if self.inputs.contains(release) {
                self.inputs
                    .remove(self.inputs.iter().position(|r| r == release).unwrap());
            }
        }
    }

    pub fn process_inputs(&mut self) {
        let factor = 0.01;

        for input in self.inputs.clone() {
            if input == String::from("W") {
                self.position += self.direction * factor;
            } else if input == String::from("A") {
                self.position += self.direction.cross_product(self.up) * factor;
            } else if input == String::from("S") {
                self.position -= self.direction * factor;
            } else if input == String::from("D") {
                self.position -= self.direction.cross_product(self.up) * factor;
            } else if input == String::from("E") {
                self.position += self.up * factor;
                self.position.1 += 0.01;
            } else if input == String::from("Q") {
                self.position -= self.up * factor;
            }
        }
    }

    pub fn projection_matrix(&self, width: u32, height: u32) -> [[f32; 4]; 4] {
        let a = height as f32 / width as f32;
        let e = 1.0 / (self.field_of_view / 2.0).tan();
        let q = self.zfar - self.znear;

        [
            [e * a, 0.0, 0.0, 0.0],
            [0.0, e, 0.0, 0.0],
            [0.0, 0.0, (self.zfar + self.znear) / q, 1.0],
            [0.0, 0.0, -(2.0 * self.zfar * self.znear) / q, 0.0],
        ]
    }

    pub fn view_matrix(&self) -> [[f32; 4]; 4] {
        let mut normalized_direction = Vec3d(self.direction.0, self.direction.1, self.direction.2);
        normalized_direction.normalize();

        let mut s_norm = Vec3d(
            self.up.1 * normalized_direction.2 - self.up.2 * normalized_direction.1,
            self.up.2 * normalized_direction.0 - self.up.0 * normalized_direction.2,
            self.up.0 * normalized_direction.1 - self.up.1 * normalized_direction.0,
        );
        s_norm.normalize();

        let u = [
            normalized_direction.1 * s_norm.2 - normalized_direction.2 * s_norm.1,
            normalized_direction.2 * s_norm.0 - normalized_direction.0 * s_norm.2,
            normalized_direction.0 * s_norm.1 - normalized_direction.1 * s_norm.0,
        ];

        let p = [
            -self.position.0 * s_norm.0 - self.position.1 * s_norm.1 - self.position.2 * s_norm.2,
            -self.position.0 * u[0] - self.position.1 * u[1] - self.position.2 * u[2],
            -self.position.0 * normalized_direction.0
                - self.position.1 * normalized_direction.1
                - self.position.2 * normalized_direction.2,
        ];

        [
            [s_norm.0, u[0], normalized_direction.0, 0.0],
            [s_norm.1, u[1], normalized_direction.1, 0.0],
            [s_norm.2, u[2], normalized_direction.2, 0.0],
            [p[0], p[1], p[2], 1.0],
        ]
    }
}
