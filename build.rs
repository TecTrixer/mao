extern crate prost_build;

fn main() {
    let mut config = prost_build::Config::new();
    config.out_dir("src/");
    // config.type_attribute(".", "#[derive(Debug)]");
    config
        .compile_protos(&["proto/items.proto"], &["proto/"])
        .unwrap();
}
