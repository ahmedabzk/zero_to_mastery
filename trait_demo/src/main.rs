trait Fall {
    fn hit_ground(&self);
}

struct Tree;

impl Fall for Tree {
    fn hit_ground(&self) {
        println!("the tree has fallen")
    }
}

struct Cat;

impl Fall for Cat {
    fn hit_ground(&self) {
        println!("the cat casually walked away after falling awkwardly");
    }
}

fn fall(thing: impl Fall) {
    thing.hit_ground();
}

fn main() {
    fall(Tree {});
    fall(Cat {});
}
