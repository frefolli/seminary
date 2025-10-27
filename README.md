# CST

## Rust

```rust
(function_item
  name: (identifier)
  parameters: (parameters)
  return_type: (unit_type)
  body: (block))
```

## Java

```java
(method_declaration
  type: (void_type)
  name: (identifier)
  parameters: (formal_parameters)
  body: (block))
```

## Python 

```python
(function_definition
  name: (identifier)
  parameters: (parameters)
  return_type: (type (none))
  body: (block (pass_statement)))
```

## C++

```c++
(function_definition
  type: (primitive_type)
  declarator: (function_declarator
    declarator: (identifier)
    parameters: (parameter_list))
  body: (compound_statement))
```

# Translation

## Java

```rust
if tsnode.type == "method_declaration" {
  let function = Function {
    name: process_name(tsnode.get_field("name")),
    type: process_type(tsnode.get_field("type")),
  }
  process_parameters(tsndoe.get_field("parameters"));
  process_body(tsnode.get_field("body"));
}
```

## Rust

```rust
if tsnode.type == "function_item" {
  let function = Function {
    name: process_name(tsnode.get_field("name")),
    type: process_type(tsnode.get_field("return_type")),
  }
  process_parameters(tsndoe.get_field("parameters"));
  process_body(tsnode.get_field("body"));
}
```

## C++

```rust
if tsnode.type == "function_definition" {
  let declarator = tsndoe.get_field("declarator");
  let function = Function {
    name: process_name(declarator.get_field("declarator")),
    type: process_type(tsnode.get_field("type")),
  }
  process_parameters(declarator.get_field("parameters"));
  process_body(tsnode.get_field("body"));
}
```

## Python


```rust
if tsnode.type == "function_definition" {
  let function = Function {
    name: process_name(tsnode.get_field("name")),
    type: process_type(tsnode.get_field("return_type")),
  }
  process_parameters(tsndoe.get_field("parameters"));
  process_body(tsnode.get_field("body"));
}
```

# Generalization

## Java/Rust/Python

```rust
if tsnode.type == Lang.tags.defun {
  let function = Function {
    name: process_name(tsnode.get_field(Lang.tags.defun.direct.name)),
    type: process_type(tsnode.get_field(Lang.tags.defun.direct.type)),
  }
  process_parameters(tsndoe.get_field(Lang.tags.defun.direct.parameters));
  process_body(tsnode.get_field(Lang.tags.defun.direct.body));
}
```

## C/C++

```rust
if tsnode.type == Lang.function_definition {
  let declarator = tsndoe.get_field(Lang.tags.defun.declarator.declarator);
  let function = Function {
    name: process_name(declarator.get_field(Lang.tags.defun.declarator.name)),
    type: process_type(tsnode.get_field(Lang.tags.defun.declarator.type)),
  }
  process_parameters(declarator.get_field(Lang.tags.defun.declarator.parameters));
  process_body(tsnode.get_field(Lang.tags.defun.declarator.body));
}
```
