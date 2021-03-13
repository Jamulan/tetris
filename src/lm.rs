#[derive(Clone, Copy)]
pub struct Mat4 { // column major
    matrix: [[f32; 4]; 4],
}

impl Mat4 {
    fn rotateBy(&self, radians: f32) {
        let rot = [
            [ radians.cos(), radians.sin(), 0.0, 0.0],
            [-radians.sin(), radians.cos(), 0.0, 0.0],
            [ 0.0 ,0.0, 1.0, 0.0],
            [ 0.0, 0.0, 0.0, 1.0],
        ];
    }

    pub fn Multiply(left: Mat4, right: Mat4) -> Mat4 {
        let mut out = Mat4::Indentity();
        for i in 0..4 {
            for j in 0..4 {
                // i hate matrices
                let mut tmp = 0.0;
                for k in 0..4 {
                    tmp += left[k][j] * right[i][k];
                }
                out.matrix[i][j] = tmp;
            }
        }
        return out;
    }

    pub fn Indentity() -> Mat4 {
        let out = Mat4 {
            matrix:  [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ],
        };
        return out;
    }
}
