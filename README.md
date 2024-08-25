# subtitle-merger

A simple tool to merge two vtt file into one with multi speakers

## Usage

```shell
% ./target/debug/subtitle-merger -h      
                                         
Simple tool to merge two WebVTT transcripts into one with speakers names

Usage: subtitle-merger [OPTIONS] <FILE1> <FILE2> <OUTPUT>

Arguments:
  <FILE1>   The first transcript
  <FILE2>   The second transcript
  <OUTPUT>  The output file without extension

Options:
      --s1 <FILE1SPEAKER>  The speaker name for the first transcript
      --s2 <FILE2SPEAKER>  The speaker name for the second transcript
      --rm-comment-sub     Remove comment subtitles
  -h, --help               Print help
  -V, --version            Print version
```

## Build

You need at least Rust 1.78.0, older versions are not tested.

Run this command to build the binary:

```shell
% cargo build --release
```

## License

MIT License, see the LICENSE file

Copyright (c) 2024 Xylobyte
