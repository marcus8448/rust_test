fn main() {
    let box_ = Box {
        width: 10,
        height: 20,
        depth: 4
    };

    assert_eq!(volume(&box_), 10 * 20 * 4);
    assert_eq!(volume(&box_), 10 * 20 * 4);
    assert_eq!(box_.volume(), 10 * 20 * 4);
    assert_eq!(box_.volume(), 10 * 20 * 4);

}

struct Box {
    width: u32,
    height: u32,
    depth: u32
}

impl Box {
    fn volume (&self) -> u32 {
        return self.width * self.height * self.depth;
    }
}

fn volume(box_: &Box) -> u32 {
    return box_.width * box_.depth * box_.height;
}