#[derive(Clone, Copy)]
pub struct Mat4 {
    // column major // ONLY USE FOR 2D STUFF,  could be updated for 3D but it isn't ready for it
    matrix: [[f32; 4]; 4],
}

impl Mat4 {
    // translation * Rotation * Scale * orig
    pub fn rotate_by(&self, radians: f32) -> Mat4 {
        let rot = Mat4 {
            matrix: [
                [radians.cos(), radians.sin(), 0.0, 0.0],
                [-radians.sin(), radians.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        let orig = Mat4::clone(self);
        return Mat4::multiply(rot, orig);
    }

    pub fn translate_by(&self, x: f32, y: f32, z: f32) -> Mat4 {
        let trans = Mat4 {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [x, y, z, 1.0],
            ],
        };
        let orig = Mat4::clone(self);
        return Mat4::multiply(trans, orig);
    }

    pub fn scale_by_simple(&self, scale: f32) -> Mat4 {
        return self.scale_by(scale, scale, 0.0);
    }

    pub fn scale_by(&self, x: f32, y: f32, z: f32) -> Mat4 {
        let scale = Mat4 {
            matrix: [
                [x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [0.0, 0.0, z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        let orig = Mat4::clone(self);
        return Mat4::multiply(scale, orig);
    }

    pub fn multiply(left: Mat4, right: Mat4) -> Mat4 {
        let mut out = Mat4::identity();
        for i in 0..4 {
            for j in 0..4 {
                // i hate matrices
                let mut tmp = 0.0;
                for k in 0..4 {
                    tmp += left.matrix[k][j] * right.matrix[i][k];
                }
                out.matrix[i][j] = tmp;
            }
        }
        return out;
    }

    pub fn identity() -> Mat4 {
        let out = Mat4 {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ],
        };
        return out;
    }
}
