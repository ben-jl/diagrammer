use diagrammer::{DatasetBase, Dataset};
use ndarray::array;

fn main() {
    let data = array![[0.0, 1.0], [1.0,2.0], [2.0, 3.0]];
    let ds : DatasetBase<f32, String> = data.into_dyn().into();
    println!("{:#?}", ds);
}