use quaternion_rs::Quaternion;

fn main() {
    let q1 = Quaternion::new(3.06, 1.0, 1.0, 2.0);
    let q2 = Quaternion::new(0.70, 3.0, -1.0, 2.0);
    println!("q1 = {q1}");
    println!("q2 = {q2}");

    println!("q1 + q2 = {}", q1 + q2);
    println!("q1 - q2 = {}", q1 - q2);
    println!("q1 * q2 = {}", q1 * q2);
    println!("q2 * q1 = {}", q2 * q1);
    println!("q1 / q2 = {}", q1 / q2);
    println!("q1 * q1.inverse() = {}", q1 * q1.inverse());
    println!("q2 * q2.inverse() = {}", q2 * q2.inverse());
    println!("Magnitude of q1 = {}", q1.magnitude());
    println!("Conjugate of q1 = {}", q1.conjugate());
}
