# Text Mutator

A tool that deliberately introduces minor errors into text to aid proofreading.

## Why?

When we read our own writing, our brains often auto-correct mistakes because we know what we intended to write. By introducing controlled mutations (swapped letters, removed punctuation, homophone substitutions), this tool makes your text just unfamiliar enough that you must read it more carefully.

## Features

- Swap adjacent letters
- Remove punctuation
- Replace words with homophones (your/you're, their/there/they're, etc.)
- Configurable mutation rate
- Logging with tracing

## Logging

The application uses the `tracing` crate for logging. You can control the log level using the `RUST_LOG` environment variable:

```
RUST_LOG=info cargo run  # Default level
RUST_LOG=debug cargo run # More detailed logs
RUST_LOG=trace cargo run # Most verbose logging
```

## License

MIT.
