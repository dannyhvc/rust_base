#[allow(non_camel_case_types, unused, non_snake_case)]

/// Heap Array Tools.
/// methods to work with and create a heap array.

pub type darray<T> = Box<[T]>;
pub type darray_2d<T> = darray<darray<T>>;
pub type darray_3d<T> = darray_2d<darray<T>>;

pub struct Harrt;
impl Harrt {
    pub fn heap_array_create<T: Clone>(size: usize) -> Box<[Option<T>]> {
        vec![None; size].into_boxed_slice()
    }

    pub fn heap_array_zeros_f32(size: usize) -> Box<[f32]> {
        vec![0f32; size].into_boxed_slice()
    }

    pub fn heap_array_zeros_i32(size: usize) -> Box<[i32]> {
        vec![0i32; size].into_boxed_slice()
    }

    pub fn heap_array2d_create<T: Clone>(
        rows_size: usize,
        cols_size: usize,
    ) -> darray_2d<Option<T>> {
        vec![vec![None; cols_size].into_boxed_slice(); rows_size].into_boxed_slice()
    }

    pub fn heap_array2d_zeros_f32(rows_size: usize, cols_size: usize) -> darray_2d<f32> {
        vec![vec![0f32; cols_size].into_boxed_slice(); rows_size].into_boxed_slice()
    }

    pub fn heap_array2d_zeros_i32(rows_size: usize, cols_size: usize) -> darray_2d<i32> {
        vec![vec![0i32; cols_size].into_boxed_slice(); rows_size].into_boxed_slice()
    }

    pub fn heap_array3d_create<T: Clone>(
        xdim_size: usize,
        ydim_size: usize,
        zdim_size: usize,
    ) -> darray_3d<Option<T>> {
        vec![
            vec![vec![None; zdim_size].into_boxed_slice(); ydim_size].into_boxed_slice();
            xdim_size
        ]
        .into_boxed_slice()
    }

    pub fn heap_array3d_zeros_f32(
        xdim_size: usize,
        ydim_size: usize,
        zdim_size: usize,
    ) -> darray_3d<f32> {
        vec![
            vec![vec![0f32; zdim_size].into_boxed_slice(); ydim_size].into_boxed_slice();
            xdim_size
        ]
        .into_boxed_slice()
    }

    pub fn heap_array3d_zeros_i32(
        xdim_size: usize,
        ydim_size: usize,
        zdim_size: usize,
    ) -> darray_3d<i32> {
        vec![
            vec![vec![0i32; zdim_size].into_boxed_slice(); ydim_size].into_boxed_slice();
            xdim_size
        ]
        .into_boxed_slice()
    }
}

#[test]
fn test_cc_d() {
    let data = Harrt::heap_array_create::<i32>(10);
    data.iter().for_each(|x| {
        assert_eq!(*x, None);
    })
}

#[test]
fn test_cc_di32() {
    const SIZE: usize = 100;
    let data = Harrt::heap_array_zeros_i32(SIZE);
    data.iter().for_each(|x| {
        assert_eq!(*x, 0i32);
    });
}

#[test]
fn test_cc_df32() {
    const SIZE: usize = 100;
    let data = Harrt::heap_array_zeros_f32(SIZE);
    data.iter().for_each(|x| assert_eq!(*x, 0f32));
}

#[test]
fn test_cc_2d() {
    let data = Harrt::heap_array2d_create::<i32>(10, 10);
    data.iter().for_each(|x| {
        x.iter().for_each(|y| assert_eq!(*y, None));
    })
}

#[test]
fn test_cc_2di32() {
    const SIZE: usize = 100;
    let data = Harrt::heap_array2d_zeros_i32(SIZE, SIZE);
    data.iter().for_each(|x| {
        x.iter().for_each(|y| assert_eq!(*y, 0i32));
    });
}

#[test]
fn test_cc_2df32() {
    const SIZE: usize = 100;
    let data = Harrt::heap_array2d_zeros_f32(SIZE, SIZE);
    data.iter().for_each(|x| {
        x.iter().for_each(|y| assert_eq!(*y, 0f32));
    });
}

#[test]
fn test_cc_3d() {
    const SIZE: usize = 100;
    let data = Harrt::heap_array3d_create::<i32>(SIZE, SIZE, SIZE);
    data.iter().for_each(|x| {
        x.iter().for_each(|y| {
            y.iter().for_each(|z| assert_eq!(*z, None));
        });
    });
}

#[test]
fn test_cc_3di32() {
    const SIZE: usize = 100;
    let data = Harrt::heap_array3d_zeros_i32(SIZE, SIZE, SIZE);
    data.iter().for_each(|x| {
        x.iter().for_each(|y| {
            y.iter().for_each(|z| assert_eq!(*z, 0i32));
        });
    });
}

#[test]
fn test_cc_3df32() {
    const SIZE: usize = 100;
    let data = Harrt::heap_array3d_zeros_f32(SIZE, SIZE, SIZE);
    data.iter().for_each(|x| {
        x.iter().for_each(|y| {
            y.iter().for_each(|z| assert_eq!(*z, 0f32));
        });
    });
}
