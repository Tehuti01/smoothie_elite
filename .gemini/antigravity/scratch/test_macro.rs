use smoothie_macros::seraphic_mandate;

#[seraphic_mandate]
fn valid_function() {
    let _x = 1 + 1;
}

// This should fail to compile if I were to run it in a real crate
/*
#[seraphic_mandate]
fn invalid_function() {
    let _v = vec![1, 2, 3];
}
*/

fn main() {
    valid_function();
}
