# Introduction

Welcome to **please**, a powerful and easy-to-use Command Line Interface (CLI) tool
to assist working on the command line.

**please** currently supports OpenAI's gpt-3.5-turbo and gpt-4.

# Usage

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

This can be used from inside (Neo)Vim: just mark some text with v/V, then pipe
through **please** like "!please complete this".

**NOTE** all piped in data and every argument to **please** is sent to the OpenAI
API endpoint!

**please** currently defaults to using `gpt-3.5-turbo`. To make it use `gpt-4`, 
use the `--model gpt-4` argument or (better) set `PLEASE_MODEL=gpt-4` alongside
`OPENAI_KEY`.

How would you make that permanent?

```
> please set PLEASE_MODEL=gpt-4 for every new shell
echo 'export PLEASE_MODEL=gpt-4' >> ~/.bashrc && source ~/.bashrc
```

# Installation

    cargo install --git https://github.com/kaspar030/please

Make sure to set your OpenAI API key:

    export OPENAI_KEY=sk-....

# License

please is licensed under the terms of the Apache License (Version 2.0).
