# Introduction

Welcome to **please**, a powerful and easy-to-use Command Line Interface (CLI) tool
specifically designed for querying Language Models (LLMs).

It has two main modes of operation:

1. Ask an LLM how to perform a specific task using CLI

    please <some task description>

In this mode, the LLM is asked to return shell command(s) that perform the given
task.

Examples:

```shell
> please output hello world
echo "hello world"
```

```shell
> please convert test.m4a to mp3
ffmpeg -i test.m4a -acodec libmp3lame -ab 192k test.mp3
```

2. Ask an LLM to change some piped input according to a given prompt

    echo "cat dog car house mouse" | please filter animals

In this mode, the piped input is passed to the LLM, together with the
**please** command line as instruction.

# Installation

    cargo install please

# License

laze is licensed under the terms of the Apache License (Version 2.0).
