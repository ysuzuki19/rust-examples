mod matrix;

fn main() -> Result<(), ()> {
    let mut mat = matrix![[0, 1, 2], [3, 4, 5], [6, 7, 8]];
    let mat_i = matrix![[1, 0, 0], [0, 1, 0], [0, 0, 1]];
    mat.print();
    mat_i.print();

    mat.add(&mat_i)?;

    mat.print();

    mat.diff(&mat_i)?;

    mat.print();
    *mat.at_mut(0, 0) = 3;
    mat.print();

    let v = mat.flatten();
    println!("{v:?}");

    Ok(())
}
