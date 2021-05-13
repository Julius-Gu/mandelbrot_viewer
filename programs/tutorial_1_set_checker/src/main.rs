fn main() {
    println!("Hello, imaginaries!");

    // check some numbers
    // 0 is in the set
    println!("the number {} is in the set: {:?}", 0, check_if_in_set(0.0,0.0));

    // 0.1 is in the set
    println!("the number {} is in the set: {:?}", "1-i", check_if_in_set(1.0,-1.0));

    // -1 is in the set
    println!("the number {} is in the set: {:?}", "-0.3+0.1i", check_if_in_set(-0.3,0.1));

    // 0.3 is not in the set
    println!("the number {} is in the set: {:?}", "5+6i", check_if_in_set(5.0,6.0));

    // 1 is not in the set
    println!("the number {} is in the set: {:?}", "0.3+0.6i", check_if_in_set(0.3,0.6));
}
// input: number of the form a+bi with real numbers a and b
fn check_if_in_set(ac: f64, bc: f64) -> bool {
    // z starts at 0
    let mut az: f64 = 0.0;
    let mut bz: f64 = 0.0;

    // repeat 100 times:
    for _i in 0..100 {
        // break when one of these becomes infinite
        if az*az>=std::f64::MAX || bz*bz>=std::f64::MAX {
            break;
        }
        let new_az = az*az - bz*bz + ac;
        bz = 2.0*az*bz + bc;
        az = new_az;
    }
    // check if its magnitude is infinite
    if (az*az+bz*bz) > std::f64::MAX {
        return false;
    } else {
        return true;
    }
}
