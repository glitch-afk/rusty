**SHA1 HASH DECODER**

```bash copy
cargo run -- <wordlist> <hash_to_crack>
```

![output](result.png)

> NOTE

- We open the wordlist file but we never close it.
- `RAII` - _Resource Acquisition Is Initialization_

- Rust variables represent parts of the memory and also may **own resources**.
- Whenever an object goes out of scope, its destructor is called, and the owned resources are freed.

- `wordlist_file` variable owns the file and has main function as scope. Whenever main function exits, the owned file is closed.
