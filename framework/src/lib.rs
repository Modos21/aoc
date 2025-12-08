pub enum Part {
    One,
    Two
}

pub trait Solution {
    fn run(part: Part);
}
