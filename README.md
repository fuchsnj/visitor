# Visitor

A generic library to easily visit elements of a structure and perform an action on each one

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
visitor = "*"
```

and this to your crate root:

```rust
extern crate visitor;
```

## Example

```rust
let data = Data{
	a: 3,
	b: 4
};
let mut adder = AddVisitor{
	value: 0
};

data.visit(&mut adder).unwrap();

assert_eq!(adder.value, 7);
```