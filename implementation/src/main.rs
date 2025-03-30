extern "C" {
    fn pqm4_demo();
}

fn main() {
    println!("Calling PQM4 function from Rust...");
    unsafe {
        pqm4_demo();
    }
}
