### **Ownership**

- It enables Rust to make memory safety guarantees without needing a garbage collector.
- Ownership is how rust manages memory
- Ownership does not slow down program while running.

> types stored on stack can have `copy` trait.

- if a type implements `copy` trait, then variables that use that type do not move, instead thay are copied.
- They are still valid, after assignment to another variable.
- `copy` is generally implemented by simple scalar values and nothing that requires allocation or is some form of resource can implement `copy`

Integers, boolean, floating point types, char type, tuples that contain other types that implement `copy` trait.
