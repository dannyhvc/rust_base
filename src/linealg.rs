/// @author: Daniel Herrera
/// @date: 04/08/2021

#[allow(non_camel_case_types, unused, non_snake_case, dead_code)]
use std::mem::{size_of, size_of_val};

///
/// matrix_algebra module
///
pub mod matrix_algebra {
    use crate::custom_container::{darray_2d, Harrt};

    pub trait Matrix {}
    pub trait MatrixFns {
        fn identity(row_size: usize, cols_size: usize) -> Self;
    }

    pub struct Matrix2x2(pub [[f32; 2]; 2]);
    pub struct Matrix3x3(pub [[f32; 3]; 3]);
    pub struct Matrix4x4(pub [[f32; 4]; 4]);

    #[derive(Debug)]
    pub struct MatrixNxN {
        pub m_rows: usize,
        pub m_cols: usize,
        pub m_data: darray_2d<f32>,
    }

    #[derive(Debug)]
    pub struct MatrixNxM {
        pub m_rows: usize,
        pub m_cols: usize,
        pub m_data: darray_2d<f32>,
    }

    /// Matrix higherarchy
    impl Matrix for Matrix2x2 {}
    impl Matrix2x2 {
        pub fn identity() -> Self {
            Self([[1.0, 0.0], [0.0, 1.0]])
        } // identity
    }

    impl Matrix for Matrix3x3 {}
    impl Matrix3x3 {
        pub fn identity() -> Self {
            Self([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
        } // identity
    }

    impl Matrix for Matrix4x4 {}
    impl Matrix4x4 {
        pub fn identity() -> Self {
            Self([
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ])
        } // identity
    }

    impl MatrixFns for MatrixNxN {
        fn identity(rows_size: usize, cols_size: usize) -> Self {
            let mut arr = Harrt::heap_array2d_zeros_f32(rows_size, cols_size);
            for i in 0..rows_size {
                arr[i][i] = 1.0f32;
            }
            Self {
                m_cols: cols_size,
                m_rows: rows_size,
                m_data: arr,
            }
        } // identity
    }

    impl MatrixNxN {
        pub fn zeros(rows_size: usize, cols_size: usize) -> Self {
            assert_eq!(cols_size, rows_size);
            Self {
                m_cols: cols_size,
                m_rows: rows_size,
                m_data: vec![vec![0.0; cols_size].into_boxed_slice(); rows_size].into_boxed_slice(),
            }
        } // identity

        pub fn new(n_size: usize) -> Self {
            MatrixNxN::zeros(n_size, n_size)
        } // new
    }

    impl MatrixFns for MatrixNxM {
        fn identity(rows_size: usize, cols_size: usize) -> Self {
            Self {
                m_cols: cols_size,
                m_rows: rows_size,
                m_data: vec![vec![0.0; cols_size].into_boxed_slice(); rows_size].into_boxed_slice(),
            }
        } // identity
    }

    impl MatrixNxM {
        pub fn zeros(rows_size: usize, cols_size: usize) -> Self {
            Self {
                m_cols: cols_size,
                m_rows: rows_size,
                m_data: vec![vec![0.0; cols_size].into_boxed_slice(); rows_size].into_boxed_slice(),
            }
        } // identity

        pub fn new(n_size: usize, m_size: usize) -> Self {
            MatrixNxM::zeros(n_size, m_size)
        } // new
    }
}

///
/// vector_algebra module
///
pub mod vector_algebra {
    pub trait Vector {}

    #[derive(Clone, Copy)]
    pub struct Vector2D {
        pub x: f32,
        pub y: f32,
    }

    #[derive(Clone, Copy)]
    pub struct Vector3D {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    #[derive(Clone, Copy)]
    pub struct Vector4D {
        pub w: f32,
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    // vector higherarchy
    impl Vector for Vector2D {}
    impl Vector for Vector3D {}
    impl Vector for Vector4D {}

    impl Vector2D {
        pub fn new(x: f32, y: f32) -> Self {
            Self { x, y }
        }

        pub fn default(v: &[f32]) -> Self {
            Self { x: v[0], y: v[1] }
        }
    }

    impl Default for Vector2D {
        fn default() -> Self {
            Self { x: 0.0, y: 0.0 }
        }
    }

    impl Vector3D {
        pub fn new(x: f32, y: f32, z: f32) -> Self {
            Self { x, y, z }
        }
    }

    pub struct Line2D {
        pub start: Vector2D,
        pub end: Vector2D,
    }
    pub struct Line3D {
        pub start: Vector3D,
        pub end: Vector3D,
    }
}

///
/// Tests for the matrix_algebra module
///
#[test]
fn test_ma_identity_2x2() {
    let ma = matrix_algebra::Matrix2x2::identity();
    assert_eq!(ma.0[0][0], 1.0f32);
    assert_eq!(ma.0[1][1], 1.0f32);
}

#[test]
fn test_ma_identity_3x3() {
    let ma = matrix_algebra::Matrix3x3::identity();
    assert_eq!(ma.0[0][0], 1f32);
    assert_eq!(ma.0[1][1], 1f32);
    assert_eq!(ma.0[2][2], 1f32);
}

#[test]
fn test_ma_identity_4x4() {
    let ma = matrix_algebra::Matrix4x4::identity();
    assert_eq!(ma.0[0][0], 1f32);
    assert_eq!(ma.0[1][1], 1f32);
    assert_eq!(ma.0[2][2], 1f32);
    assert_eq!(ma.0[3][3], 1f32);
}

#[test]
fn test_ma_identity_nxn() {
    let ma = matrix_algebra::MatrixNxN::new(5);
    const FAT_PTR_SIZE: usize = 16;

    assert_eq!(ma.m_cols, 5);
    assert_eq!(ma.m_rows, 5);
    assert_eq!(size_of_val(&ma.m_data), FAT_PTR_SIZE);
}

#[test]
fn test_ma_zeros_nxn() {
    // use self::matrix_algebra::Matrix_fns;
    use std::mem::size_of_val;

    let ma = matrix_algebra::MatrixNxN::zeros(10, 10);
    const FAT_PTR_SIZE: usize = 16;

    assert_eq!(ma.m_rows, 10);
    assert_eq!(ma.m_cols, 10);
    assert_eq!(size_of_val(&ma.m_data), FAT_PTR_SIZE); // 4 spots * 4 byte ptrs, but since they're ptrs we don't see them.

    // println!("{:?}", ma);
}

#[test]
fn test_ma_identity_nxm() {
    let ma = matrix_algebra::MatrixNxM::new(5, 10);
    const FAT_PTR_SIZE: usize = 16;

    assert_eq!(ma.m_rows, 5);
    assert_eq!(ma.m_cols, 10);
    assert_eq!(size_of_val(&ma.m_data), FAT_PTR_SIZE);
}

#[test]
fn test_ma_zeros_nxm() {
    // use self::matrix_algebra::Matrix_fns;
    use std::mem::size_of_val;

    let ma = matrix_algebra::MatrixNxM::zeros(10, 5);
    const FAT_PTR_SIZE: usize = 16;

    assert_eq!(ma.m_rows, 10);
    assert_eq!(ma.m_cols, 5);
    assert_eq!(size_of_val(&ma.m_data), FAT_PTR_SIZE); // 4 spots * 4 byte ptrs, but since they're ptrs we don't see them.

    // println!("{:?}", ma);
}
