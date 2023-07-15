# Dart repointer

Dart repointer (`reptr`) ~~puts the sharp part in your Dart~~ aims to facilitate _fast_ code generation for Dart. It consists of [the tool itself](reptr/) and a [Dart parser library](dart-parser).

## Design notes

- Loading source files
  - Loading the complete file for parsing is fine
  - Memory-mapping is not faster per se
    - May play nicely with a lazy tokenizer with lazy UTF-8 validation
- Parsing
  - The parser is a _partial_ parser: it recognizes certain parts of the target language and can skip over the rest
  - Capturing slices of the source (`&str`) is extremely cheap (doesn't cause memory allocation); do that
  - When feasible, avoid memory allocation (namely, the use of `Vec` and co.)
- A tempting feature: in-place code generation
  - Requires accurate back-to-source transformation
- Output formatting
  - Delegate to [dart format](https://dart.dev/tools/dart-format) (it's fast enough)
