# linux-personality
This crate intends to be a safe wrapper for the `personality` linux function.

# Usage
First add this to your `Cargo.toml`:

```toml
[dependencies]
linux-personality = "1.0"
```

And this to your crate root:

```rust
extern crate linux_personality;
```

# Example
```rust
extern crate linux_personality;

use linux_personality::get_personality();

fn main() {
    let persona = get_personality().unwrap();

    println!("Current personality: {:?}", persona);
}
```

# Authors
**Jean Pierre Dudey** - *Initial work* - jeandudey@hotmail.com

# License
This project is licensed under the MIT License. For more information see the LICENSE file on this repository.
