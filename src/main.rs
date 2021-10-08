// mod linealg;
pub mod custom_container;
pub mod linealg;

pub fn semi_circle_integral(R: f64, x: f64) -> f64 {
    (0.5) * (x * f64::sqrt(R * R - x * x) + R * R * (x / f64::sqrt(R * R - x * x)).tan().atan())
}

pub fn tankvol(h: i32, d: i32, vt: i32) -> i32 {
    use std::f32::consts::PI;
    let R = d as f32 / 2_f32;

    fn Height(R: f32 , h: f32) -> f32 {
        return if h as f32 >= R {
            2_f32*R - h as f32
        }
        else {
            R - h as f32
        }
    }

    let k = Height(R, h as f32); // height of the triangle
    let b = 2_f32 * f32::sqrt(R*R - k*k); // base of the triangle
    let theata = f32::acos(R*R + k*k / (b*b - 2_f32*R*k));
    let aot = (b * k) / 2_f32;

    let area = if theata > 180_f32{
        (((360_f32 - theata) * PI * R * R) / 360_f32) + aot
    }
    else if theata == 180_f32 {
        PI * R * R / 2.0_f32
    }
    else {
        ((theata * PI * R * R) / 360_f32) - aot
    };
    return area.floor().trunc() as i32

}

# [test]
fn test_tankvol() {
    println!("tankvol: {}", tankvol(-4, 25, 1,));
}

fn main() {
    let _foo = Box::new(0);
    println!("{}", *_foo);
}
