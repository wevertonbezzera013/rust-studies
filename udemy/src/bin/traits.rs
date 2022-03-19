trait Fall {
    fn hit_ground(&self);
}

struct Vase;
impl Fall for Vase {
    fn hit_ground(&self) {
        println!("Hit ground!")
    }
}
struct Cat;
impl Fall for Cat {
    fn hit_ground(&self) {
        println!("cat!")
    }
}

fn fall(things: impl Fall) {
    thing.hit_ground();
}

fn main() {
    fall(Vase {});
    fall(Cat {});
}