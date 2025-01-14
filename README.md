# Protoflow

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.70%2B-blue)](https://rust-lang.org)
[![Package](https://img.shields.io/crates/v/protoflow)](https://crates.io/crates/protoflow)

_"Τὰ πάντα ῥεῖ καὶ οὐδὲν μένει" — Heraclitus_

🚧 _We are building in public. This is presently under heavy construction._

## 🛠️ Prerequisites

- [Rust](https://rust-lang.org) 1.70+

## ⬇️ Installation

### Installation via Homebrew

```bash
brew install protoflow --HEAD
```

### Installation via Cargo

```bash
cargo install protoflow
```

## 👉 Examples

TBD

## 📚 Reference

### Glossary

- **System**: A collection of blocks that are connected together.
  Systems are the top-level entities in a Protoflow program.

- **Block**: An encapsulated system component that processes messages.
  Blocks are the autonomous units of computation in a system.

- **Port**: A named connection point on a block that sends or receives
  messages. Ports are the only interfaces through which blocks communicate
  with each other.

- **Message**: A unit of data that flows between blocks in a system.
  Messages are Protocol Buffers packets that are processed by blocks.

### Blocks

| Block           | Description                                                |
| :-------------- | :--------------------------------------------------------- |
| [`Buffer`]      | Stores all messages it receives.                           |
| [`Const`]       | Sends a constant value.                                    |
| [`Count`]       | Counts the number of messages it receives, while optionally passing them through. |
| [`Decode`]      | Decodes messages from a byte stream.                       |
| [`Delay`]       | Passes messages through while delaying them by a fixed or random duration. |
| [`Drop`]        | Discards all messages it receives.                         |
| [`Encode`]      | Encodes messages to a byte stream.                         |
| [`Random`]      | Generates and sends a random value.                        |
| [`ReadDir`]     | Reads file names from a file system directory.             |
| [`ReadEnv`]     | Reads the value of an environment variable.                |
| [`ReadFile`]    | Reads bytes from the contents of a file.                   |
| [`ReadStdin`]   | Reads bytes from standard input (aka stdin).               |
| [`WriteFile`]   | Writes or appends bytes to the contents of a file.         |
| [`WriteStderr`] | Writes bytes to standard error (aka stderr).               |
| [`WriteStdout`] | Writes bytes to standard output (aka stdout).              |

#### [`Buffer`]

A block that simply stores all messages it receives.

```mermaid
block-beta
    columns 4
    Source space:2 Buffer
    Source-- "input" -->Buffer

    classDef block height:48px,padding:8px;
    classDef hidden visibility:none;
    class Buffer block
    class Source hidden
```

#### [`Const`]

A block for sending a constant value.

```mermaid
block-beta
    columns 4
    Const space:2 Sink
    Const-- "output" -->Sink

    classDef block height:48px,padding:8px;
    classDef hidden visibility:none;
    class Const block
    class Sink hidden
```

#### [`Count`]

A block that counts the number of messages it receives, while optionally passing them through.

```mermaid
block-beta
    columns 7
    Source space:2 Count space:2 Sink
    space:7
    space:7
    space:3 Result space:3
    Source-- "input" -->Count
    Count-- "output" -->Sink
    Count-- "count" -->Result

    classDef block height:48px,padding:8px;
    classDef hidden visibility:none;
    class Count block
    class Source hidden
    class Sink hidden
    class Result hidden
```

#### [`Decode`]

A block that decodes `T` messages from a byte stream.

```mermaid
block-beta
    columns 7
    Source space:2 Decode space:2 Sink
    Source-- "input" -->Decode
    Decode-- "output" -->Sink

    classDef block height:48px,padding:8px;
    classDef hidden visibility:none;
    class Decode block
    class Source hidden
    class Sink hidden
```

#### [`Delay`]

A block that passes messages through while delaying them by a fixed or random duration.

```mermaid
block-beta
    columns 7
    Source space:2 Delay space:2 Sink
    Source-- "input" -->Delay
    Delay-- "output" -->Sink

    classDef block height:48px,padding:8px;
    classDef hidden visibility:none;
    class Delay block
    class Source hidden
    class Sink hidden
```

#### [`Drop`]

A block that simply discards all messages it receives.

```mermaid
block-beta
    columns 4
    Source space:2 Drop
    Source-- "input" -->Drop

    classDef block height:48px,padding:8px;
    classDef hidden visibility:none;
    class Drop block
    class Source hidden
```

#### [`Encode`]

A block that encodes `T` messages to a byte stream.

```mermaid
block-beta
    columns 7
    Source space:2 Encode space:2 Sink
    Source-- "input" -->Encode
    Encode-- "output" -->Sink

    classDef block height:48px,padding:8px;
    classDef hidden visibility:none;
    class Encode block
    class Source hidden
    class Sink hidden
```

#### [`Random`]

A block for generating and sending a random value.

```mermaid
block-beta
    columns 4
    Random space:2 Sink
    Random-- "output" -->Sink

    classDef block height:48px,padding:8px;
    classDef hidden visibility:none;
    class Random block
    class Sink hidden
```

#### [`ReadDir`]

A block that reads file names from a file system directory.

```mermaid
block-beta
    columns 4
    Config space:3
    space:4
    space:4
    ReadDir space:2 Sink
    Config-- "path" -->ReadDir
    ReadDir-- "output" -->Sink

    classDef block height:48px,padding:8px;
    classDef hidden visibility:none;
    class ReadDir block
    class Config hidden
    class Sink hidden
```

#### [`ReadEnv`]

A block that reads the value of an environment variable.

```mermaid
block-beta
    columns 4
    Config space:3
    space:4
    space:4
    ReadEnv space:2 Sink
    Config-- "name" -->ReadEnv
    ReadEnv-- "output" -->Sink

    classDef block height:48px,padding:8px;
    classDef hidden visibility:none;
    class ReadEnv block
    class Config hidden
    class Sink hidden
```

#### [`ReadFile`]

A block that reads bytes from the contents of a file.

```mermaid
block-beta
    columns 4
    Config space:3
    space:4
    space:4
    ReadFile space:2 Sink
    Config-- "path" -->ReadFile
    ReadFile-- "output" -->Sink

    classDef block height:48px,padding:8px;
    classDef hidden visibility:none;
    class ReadFile block
    class Config hidden
    class Sink hidden
```

#### [`ReadStdin`]

A block that reads bytes from standard input (aka stdin).

```mermaid
block-beta
    columns 4
    ReadStdin space:2 Sink
    ReadStdin-- "output" -->Sink

    classDef block height:48px,padding:8px;
    classDef hidden visibility:none;
    class ReadStdin block
    class Sink hidden
```

#### [`WriteFile`]

A block that writes or appends bytes to the contents of a file.

```mermaid
block-beta
    columns 4
    space:3 Config
    space:4
    space:4
    Source space:2 WriteFile
    Config-- "path" -->WriteFile
    Source-- "input" -->WriteFile

    classDef block height:48px,padding:8px;
    classDef hidden visibility:none;
    class WriteFile block
    class Config hidden
    class Source hidden
```

#### [`WriteStderr`]

A block that writes bytes to standard error (aka stderr).

```mermaid
block-beta
    columns 4
    Source space:2 WriteStderr
    Source-- "input" -->WriteStderr

    classDef block height:48px,padding:8px;
    classDef hidden visibility:none;
    class WriteStderr block
    class Source hidden
```

#### [`WriteStdout`]

A block that writes bytes to standard output (aka stdout).

```mermaid
block-beta
    columns 4
    Source space:2 WriteStdout
    Source-- "input" -->WriteStdout

    classDef block height:48px,padding:8px;
    classDef hidden visibility:none;
    class WriteStdout block
    class Source hidden
```

## 👨‍💻 Development

```console
$ git clone https://github.com/AsimovPlatform/protoflow.git
```

- - -

[![Share on Twitter](https://img.shields.io/badge/share%20on-twitter-03A9F4?logo=twitter)](https://twitter.com/share?url=https://github.com/AsimovPlatform/protoflow&text=Protoflow)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/AsimovPlatform/protoflow&title=Protoflow)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hacker%20news-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/AsimovPlatform/protoflow&t=Protoflow)
[![Share on Facebook](https://img.shields.io/badge/share%20on-facebook-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/AsimovPlatform/protoflow)

[`Buffer`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.Buffer.html
[`Const`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.Const.html
[`Count`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.Count.html
[`Decode`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.Decode.html
[`Delay`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.Delay.html
[`Drop`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.Drop.html
[`Encode`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.Encode.html
[`Random`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.Random.html
[`ReadDir`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.ReadDir.html
[`ReadEnv`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.ReadEnv.html
[`ReadFile`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.ReadFile.html
[`ReadStdin`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.ReadStdin.html
[`WriteFile`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.WriteFile.html
[`WriteStderr`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.WriteStderr.html
[`WriteStdout`]: https://docs.rs/protoflow-blocks/latest/protoflow_blocks/struct.WriteStdout.html
