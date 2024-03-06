# mdbook-markmap

A mdbook preprocessor to render markdown with markmap.

## Usage

Install markmap-cli with npm:

```
npm install -g markmap-cli
```

Install the mdbook-markmap plugin with cargo:

```bash
cargo install --git https://github.com/r4ve1/mdbook-markmap
```

Add the following to your `book.toml`:

```toml
[preprocessor.markmap]
```

Then, in your markdown files, use the following syntax to render a markmap:

```markmap
# This is a markmap

## Some heading

### Item 1

### Item 2

```

## Contributing

Contributions are welcome! Please open an issue or pull request to discuss your ideas.
