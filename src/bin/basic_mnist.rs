use mnist::{Mnist, MnistBuilder};
use rulinalg::matrix::{BaseMatrix, BaseMatrixMut, Matrix};
use std::time::SystemTime;

fn main() {
    let (trn_size, rows, cols) = (50_000, 28, 28);

    // Deconstruct the returned Mnist struct.
    let Mnist {
        trn_img, trn_lbl, ..
    } = MnistBuilder::new()
        .label_format_digit()
        .training_set_length(trn_size)
        .validation_set_length(10_000)
        .test_set_length(10_000)
        .finalize();

    // Get the label of the first digit.
    let first_label = trn_lbl[0];
    println!("The first digit is a {}.", first_label);
    println!("label length: {}", trn_lbl.len());

    // Convert the flattened training images vector to a matrix.
    let trn_img = Matrix::new((trn_size * rows) as usize, cols as usize, trn_img);

    // Get the image of the first digit.
    let row_indexes = (0..((28 * 3) - 1)).collect::<Vec<_>>();
    let first_image = trn_img.select_rows(&row_indexes);
    println!("The image looks like... \n{}", first_image);

    let start = SystemTime::now();
    // Convert the training images to f32 values scaled between 0 and 1.
    let trn_img: Matrix<f32> = trn_img.try_into().unwrap() / 255.0;
    println!(
        "Time to convert data: {}",
        start.elapsed().unwrap().as_secs_f32()
    );

    // // Get the image of the first digit and round the values to the nearest tenth.
    let first_image = trn_img
        .select_rows(&row_indexes)
        .apply(&|p| (p * 10.0).round() / 10.0);
    println!("The image looks like... \n{}", first_image);
}
