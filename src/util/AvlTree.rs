use std::cmp::max;

#[derive(Debug)]
pub struct AvlNode<T: Ord> {
    pub val: T,
    ln: AvlTree<T>,
    rn: AvlTree<T>,
    pub height: usize,
}

pub type AvlTree<T> = Option<Box<AvlNode<T>>>;

impl<'a, T: 'a + Ord> AvlNode<T> {

    pub fn calc_height(&mut self) -> () {
        self.height = 1 + max(self.l_height(), self.r_height())
    }

    pub fn balance_factor(&self) -> i8 {
        let lh = self.l_height();
        let rh = self.r_height();

        if lh >= rh { (lh - rh) as i8 } else { -((rh - lh) as i8) }
    }

    fn l_height(&self) -> usize {
        self.ln.as_ref().map_or(0, |node| node.height)
    }

    fn r_height(&self) -> usize {
        self.rn.as_ref().map_or(0, |node| node.height)
    }

    pub fn new(value: T) -> Self {
        return AvlNode {
            val: value,
            ln: Option::None,
            rn: Option::None,
            height: 1,
        };
    }
}

pub enum RotationType {
    Simple,
    Double
}

#[test]
fn avl_tree_test() {
    let tree: AvlNode<u8> = AvlNode::new(3);

    println!("{:?}", tree);
}
