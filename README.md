# Proof of concept: Rust compile-time templates

See the `example` directory for an example.

## Limitations

- The template path must be relative to your Cargo.toml, because the macro handler has no way of
  knowing the path of the file that called the macro.

- You have to annotate types that are not strings with type ascription.
