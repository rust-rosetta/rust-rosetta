// implements http://rosettacode.org/wiki/Zig-zag_matrix
// with the sorting indexes algorithm
// explained in the discussion page
// http://rosettacode.org/wiki/Talk:Zig-zag_matrix

#[deriving(Show, PartialEq, Eq)]
struct SortIndex {
    x:  uint,
    y:  uint
}

impl SortIndex {
    fn new(x:uint, y:uint) -> SortIndex {
        SortIndex{x:x, y:y}
    }
}

impl PartialOrd for SortIndex {
    fn lt(&self, other: &SortIndex) -> bool {
        if self.x + self.y == other.x + other.y {
            if (self.x + self.y) % 2 == 0 {
                self.x < other.x
            } else {
                self.y < other.y
            }
        } else {
            (self.x + self.y) < (other.x + other.y)
        }
    }
}

impl Ord for SortIndex {
    fn cmp(&self, other: &SortIndex) -> Ordering {
        if self < other { Less }
        else if self > other { Greater }
        else {Equal}
    }
}

fn zigzag(n:uint) -> Vec<Vec<uint>> {
    let mut l:Vec<SortIndex> = range(0u, n*n).map(|i| SortIndex::new(i%n,i/n)).collect();
    l.sort();

    let mut result : Vec<Vec<uint>> = Vec::from_elem(n, Vec::from_elem(n,0u));
    for t_ind in l.iter().enumerate() {
        let (i,&SortIndex{x,y}) = t_ind;
        *result.get_mut(y).get_mut(x) = i
    }
    result
}

#[cfg(not(test))]
fn main() {
    println!("{}", zigzag(5));
}

#[test]
fn result() {
   let exp =vec![vec![0,  1, 5, 6,14],
                 vec![2,  4, 7,13,15],
                 vec![3,  8,12,16,21],
                 vec![9, 11,17,20,22],
                 vec![10,18,19,23,24]];
    assert_eq!(zigzag(5), exp);
}
