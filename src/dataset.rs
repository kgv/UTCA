use linfa::Dataset;
use ndarray::prelude::*;

// The `.csv` data is two dimensional: Axis(0) denotes y-axis (rows), Axis(1) denotes x-axis (columns)
// sepal_length,sepal_width,petal_length,petal_width,species
// 5.1,3.5,1.4,0.2,0
// 4.9,3.0,1.4,0.2,0
// let feature_names = vec!["sepal length", "sepal width", "petal length", "petal width"];

/// Read in a dataset from dataset path.
pub fn dataset<T: Into<String>>(data: &[f64], feature_names: Vec<T>) -> Dataset<f64, usize, Ix1> {
    // let data = include_bytes!("../data/iris.csv.gz");
    // let array = array_from_buf(&data[..]);
    let array = array![
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 10.0, 11.0, 12.0],
    ];
    let (data, targets) = (
        array.slice(s![.., 0..4]).to_owned(),
        array.column(4).to_owned(),
    );
    let records = array![
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 10.0, 11.0, 12.0],
    ];
    Dataset::new(records, targets)
        .map_targets(|x| *x as usize)
        .with_feature_names(feature_names)
}
