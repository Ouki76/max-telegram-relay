fn main() {
    println!(
        "cargo:rerun-if-changed={}",
        dotenv::dotenv().unwrap().display()
    );

    for (key, value) in dotenv::vars() {
        println!("cargo:rustc-env={}={}", key, value)
    }
}
