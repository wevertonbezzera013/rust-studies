const MAX_SPEED: i32 = 9000;

fn clamp_speed(speed: i32) -> i32 {
    if speed > MAX_SPEED {
        9000
    } else {
        speed
    }
}

fn main() {

}