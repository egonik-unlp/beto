extern crate itertools;
use itertools::Itertools;
fn main() {
    let d = vec![[1,2], [3,4], [2,8], [2,4]];
    let ordi = d
        .into_iter()
        .unique_by(|x,y|x)
        .collect::<Vec<_>>();
    println!("{:?}", ordi)
}
