# rbxm-rs

[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/nanowrimo.svg)](./LICENSE-APACHE)

A reader for Roblox model files, implemented in Rust.

## Features

- Strongly-typed handling of many Roblox instance kinds
- Feature flags to add support for less stable formats, E.G. CSGPHS Meshes.
- `#[no_std]` support, as long as alloc is present

## Example

```rust
use rbxm::from_file;

fn main() {
    // Read a model from a file
    let model = match from_file("./Model.rbxm") {
        Ok(model) => model,
        Err(SerdeError::IoError(err)) => panic!("IO Error: {}", err),
        Err(err) => panic!("Error parsing model: {}", err),
    };

    // Get a part from a path, this looks for a root node named ModelSection with a child named Part, and returns
    // that child.
    let part = match model.get_path("ModelSection/Part") {
        Ok(part) => part,
        Err(ModelError::NotFound) => panic!("Couldn't find instance at \"ModelSection/Part\""),
        Err(ModelError::AmbiguousPath) => panic!("Found more than one instance matching \"ModelSection/Part\""),
        Err(err) => panic!("Model Error: {}", err),
    };

    println!("Part Class: {}", part.borrow().kind.class_name())
}
```

## TODO

- Better tree implementation, EG slotmap backed
- Complete instance info documentation

## License
Licensed under either of
- Apache License, Version 2.0 (LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
