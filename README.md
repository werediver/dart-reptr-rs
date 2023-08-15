[![Validation](https://github.com/werediver/dart-reptr-rs/actions/workflows/validation.yaml/badge.svg?event=push&branch=main)](https://github.com/werediver/dart-reptr-rs/actions/workflows/validation.yaml)

# Dart repointer

Dart repointer (`reptr`) ~~puts the sharp part in your Dart~~ aims to facilitate _fast_ code generation for Dart. It consists of [the tool itself](reptr/) and a [Dart parser library](dart-parser). This is predominantly due to inefficiencies in the code generation "infrastructure", the `build_runner`. A faster implementation should allow 20–50x speed-up.

## Motivation

`build_runner`-based code generation for a single package can take a couple minutes. This translates to unreasonably long code generation time for a large Dart or Flutter project (e.g. over 45 minutes 🤯). We can do better.

## Project structure

The key components of this project that are already functional are:

- Filesystem scanner
  - Identifies Dart packages and source files on the filesystem
- File loader
  - Loads a file into memory, performs UTF-8 validation
- Parser (lexerless)
  - Constructs an abstract syntax tree from the loaded Dart code

```mermaid
flowchart LR
  CLI --> Scanner[F/s scanner] --> Loader --> Parser["Parser\n(lexerless)"] --> Gen1 & Gen2 --> Combiner --> Writer --> Formatter[External formatter]

  CLI --> Reporter[Status / progress reporter]

  CLI --> Watcher[F/s watcher]

  subgraph Generator 1
    Gen1[...]
  end

  subgraph Generator 2
    Gen2[...]
  end

  style Reporter stroke-dasharray: 5 5
  style Watcher stroke-dasharray: 5 5
  style Gen1 stroke-dasharray: 5 5
  style Gen2 stroke-dasharray: 5 5
  style Combiner stroke-dasharray: 5 5
  style Writer stroke-dasharray: 5 5
  style Formatter stroke-dasharray: 5 5
```

## Design notes

- Loading source files
  - Loading the complete file for parsing is fine
  - Memory-mapping is not faster per se
    - May play nicely with a lazy tokenizer with lazy UTF-8 validation
- Parsing
  - The parser is a _partial_ parser: it recognizes certain parts of the target language and can skip over the rest
  - Capturing slices of the source (`&str`) is extremely cheap (doesn't cause memory allocation), so take advantage of that
  - When feasible, avoid memory allocation (namely, the use of `Vec` and co.)
  - When implementing a parser as a function
    - Do not start parsing with whitespace
      - E.g. the import-stmt. parser should start with consuming `import`, not whitespace
    - Do not consume the trailing whitespace after a construct (e.g. in `import 'dart:math';\n\n` do not consume `\n\n`)
  - When combining parsers
    - Prefer consuming whitespace in trailing position, not leading
- A tempting feature: in-place code generation / code transformation
  - Requires accurate back-to-source transformation
- Output formatting
  - Delegate to [dart format](https://dart.dev/tools/dart-format) (it's fast enough)
