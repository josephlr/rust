// error-pattern: error reading Clippy's configuration file

#![feature(plugin)]
#![plugin(clippy(conf_file="./tests/ui-windows/conf_bad_toml.toml"))]

fn main() {}
