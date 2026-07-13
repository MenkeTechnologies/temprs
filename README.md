```
 ████████╗███████╗███╗   ███╗██████╗ ██████╗ ███████╗
 ╚══██╔══╝██╔════╝████╗ ████║██╔══██╗██╔══██╗██╔════╝
    ██║   █████╗  ██╔████╔██║██████╔╝██████╔╝███████╗
    ██║   ██╔══╝  ██║╚██╔╝██║██╔═══╝ ██╔══██╗╚════██║
    ██║   ███████╗██║ ╚═╝ ██║██║     ██║  ██║███████║
    ╚═╝   ╚══════╝╚═╝     ╚═╝╚═╝     ╚═╝  ╚═╝╚══════╝
```

[![CI](https://github.com/MenkeTechnologies/temprs/actions/workflows/ci.yml/badge.svg)](https://github.com/MenkeTechnologies/temprs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/temprs.svg)](https://crates.io/crates/temprs)
[![Downloads](https://img.shields.io/crates/d/temprs.svg)](https://crates.io/crates/temprs)
[![Docs.rs](https://docs.rs/temprs/badge.svg)](https://docs.rs/temprs)
 [![Docs](https://img.shields.io/badge/docs-online-blue.svg)](https://menketechnologies.github.io/temprs/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

### `[TEMPORARY FILE STACK MANAGER // FULL SPECTRUM DATA CONTROL]`

> *"The stack is vast and infinite."*

`temprs` (binary: `tp`) is a temporary file stack manager in Rust. Stack-based `push`/`pop`/`shift`/`unshift`, dual indexing by position or `@name`, `head`/`tail`/`wc`/`size`, find-and-replace, `grep`, `diff`, `concat`, and an atomic `flock`-protected master record. Two binaries (`tp` + `temprs`), zsh completions, short + full man pages (`man temprs`, `man temprsall`).

 ┌──────────────────────────────────────────────────────────────┐
 │ STATUS: ONLINE &nbsp;&nbsp; THREAT LEVEL: NEON &nbsp;&nbsp; SIGNAL: ████████░░ │
 └──────────────────────────────────────────────────────────────┘

### [`Read the Docs`](https://menketechnologies.github.io/temprs/) &middot; [`Engineering Report`](https://menketechnologies.github.io/temprs/report.html) · [`strykelang`](https://github.com/MenkeTechnologies/strykelang) · [`zshrs`](https://github.com/MenkeTechnologies/zshrs)

---

## Table of Contents

- [\[0x00\] Screenshots](#0x00-screenshots)
- [\[0x01\] System Requirements](#0x01-system-requirements)
- [\[0x02\] Installation](#0x02-installation)
- [\[0x03\] Usage](#0x03-usage)
- [\[0x04\] Environment](#0x04-environment)
- [\[0x05\] Data Integrity](#0x05-data-integrity)
- [\[0x06\] Stack Architecture](#0x06-stack-architecture)
- [\[0x07\] Development & CI](#0x07-development--ci)
- [\[0xFF\] License](#0xff-license)

---

## [0x00] SCREENSHOTS

#### HELP // SYSTEM INTERFACE
![help](img/help.png)

#### LIST FILES // STACK ENUMERATION
![list files](img/list-files.png)

#### LIST NUMBERED // INDEXED STACK VIEW
![list numbered](img/list-numbered.png)

#### LIST CONTENTS // FULL DATA DUMP
![list contents](img/list-contents.png)

#### OUTPUT // DATA EXTRACTION
![output](img/output.png)

---

## [0x01] SYSTEM REQUIREMENTS

- Rust toolchain // `rustc` + `cargo`

## [0x02] INSTALLATION

#### HOMEBREW TAP (RECOMMENDED)

```sh
brew tap MenkeTechnologies/menketech
brew install temprs
```

#### DOWNLOADING PAYLOAD FROM CRATES.IO

```sh
cargo install temprs
```

#### COMPILING FROM SOURCE

```sh
git clone https://github.com/MenkeTechnologies/temprs
cd temprs
cargo build --release
```

[temprs on Crates.io](https://crates.io/crates/temprs)

#### ZSH COMPLETION // TAB-COMPLETE ALL THE THINGS

```sh
# copy to a directory in your fpath (file is `_temprs`, provides #compdef for both `tp` and `temprs`)
cp completions/_temprs /usr/local/share/zsh/site-functions/_temprs

# or add the completions directory to fpath in your .zshrc
fpath=(/path/to/temprs/completions $fpath)

# then reload completions
autoload -Uz compinit && compinit
```

Completions dynamically resolve stack indices, file names, and `@name` tags.

---

## [0x03] USAGE

> Replace `CMD` with any command, `FILE` with any file, `INDEX` with any index

#### SCANNING DATA STREAMS // STDIN OPERATIONS

```sh
# jack data into a new tempfile on top of stack
CMD | tp

# jack data in and echo contents to stdout
CMD | tp -v

# read from top of stack to stdout
tp | CMD
```

#### TARGETING INDEXED TEMPFILES // PRECISION I/O

```sh
# write stdin into tempfile at index 1
CMD | tp -i 1

# write stdin into tempfile at index 1 and echo to stdout
CMD | tp -i 1 -v

# output tempfile at index 1 to stdout
tp -o 1 | CMD

# show first 5 lines of tempfile at index 1
tp --head 1 5

# show last 10 lines by name
tp --tail mydata 10

# print line count
tp --wc INDEX
tp --wc mydata

# print byte size
tp --size INDEX
tp --size mydata

# print file path (for use in scripts)
tp --path INDEX
tp --path mydata
cat "$(tp --path 1)"
```

#### LOADING FILE PAYLOADS // FILE OPERATIONS

```sh
# read FILE into new tempfile on top of stack
tp FILE | CMD

# read FILE into new tempfile and write contents to stdout
tp -v FILE | CMD

# write FILE contents to tempfile 1
tp -i 1 FILE | CMD

# write FILE contents to tempfile 1 then to stdout
tp -vi 1 FILE | CMD
```

#### CHAINING DATA STREAMS // PIPELINE OPERATIONS

```sh
# read stdin to tempfile 1 then write to stdout
CMD | tp -vi 1 | CMD

# choose input tempfile and write to tempfile at index 2 and stdout
CMD | tp -vi 2
```

#### APPENDING DATA // ACCUMULATE

```sh
# append stdin to tempfile at INDEX
CMD | tp -A INDEX

# append by name
CMD | tp -A mydata
```

#### ENUMERATING STACK CONTENTS // LISTING

```sh
# list all tempfiles on the stack
tp -l

# list all tempfiles with contents
tp -L

# list all tempfiles numbered
tp -n

# list all tempfiles numbered with contents
tp -N

# print the number of files on the stack
tp -k
```

#### EDITOR INTEGRATION // DIRECT ACCESS

```sh
# open tempfile at INDEX in $EDITOR (falls back to vi)
tp -e INDEX

# open the most recent tempfile (top of stack)
tp -e -1
```

#### NAMING TEMPFILES // ALIAS TAGS

Every tempfile can optionally be given a **tag name** — a string alias that can be used anywhere you'd normally pass a numeric index. This means all operations support **dual indexing**: by position (`1`, `2`, `-1`) or by name (`mydata`, `config`).

When you pass an argument to an operation, it is first tried as a numeric index. If that fails, it is looked up as a tag name. Names must be unique across the stack and must not contain null bytes.

```sh
# tag a new tempfile with a name
CMD | tp -w mydata

# retrieve by name instead of index
tp -o mydata | CMD

# remove by name
tp -r mydata

# rename a tag
tp -R mydata newname

# rename by index
tp -R 1 newname
```

Names travel with their files — moves, swaps, and duplicates preserve the tag. Names are displayed with a `@` prefix in listings (e.g. `@mydata`).

#### INSPECTING TEMPFILES // METADATA

```sh
# show metadata for tempfile by name or index
tp -I mydata
tp -I 1
```

#### FIND AND REPLACE // TRANSFORM

```sh
# replace all occurrences of PATTERN with REPLACEMENT in tempfile
tp --replace INDEX PATTERN REPLACEMENT

# replace by name, prints number of replacements made
tp --replace mydata old new
```

#### SEARCHING CONTENTS // GREP

```sh
# search all tempfiles for a pattern
tp -g PATTERN

# exits 0 if matches found, 1 if none
tp -g needle && echo "found"
```

#### CONCATENATING TEMPFILES // MERGE

```sh
# concatenate tempfiles by index
tp -C 1 2 3 | CMD

# concatenate by name
tp -C alpha beta | CMD

# mix indices and names, any order
tp -C 3 alpha 1
```

#### COMPARING TEMPFILES // DIFF

```sh
# unified diff of two tempfiles by index
tp -D 1 2

# diff by name
tp -D alpha beta

# exits 0 if identical, 1 if different
```

#### STACK MANIPULATION // PUSH / POP / SHIFT

```sh
# purge all tempfiles
tp -c

# purge tempfiles older than 24 hours
tp --expire 24

# purge tempfiles older than 30 minutes
tp --expire 0.5

# remove tempfile at INDEX
tp -r INDEX

# insert tempfile at INDEX
CMD | tp -a INDEX

# insert FILE at INDEX
tp -a INDEX FILE

# pop from top of stack
tp -p

# push to bottom of stack (stdin, no stdout)
CMD | tp -u

# push to bottom of stack from terminal (waits for stdin)
tp -u

# push to bottom of stack (equivalent)
CMD | tp -a 1

# shift from bottom of stack
tp -s

# move tempfile from one position to another
tp -M 1 3

# move by name
tp -M mydata 1

# duplicate tempfile onto top of stack
tp -x INDEX
tp -x mydata

# swap two tempfiles
tp -S 1 3
tp -S alpha beta

# reverse the entire stack
tp --rev

# sort stack by filename (default), size, or modification time
tp --sort name
tp --sort size
tp --sort mtime
```

#### WORK QUEUE // LEASE / ACK / NACK

The stack doubles as a daemonless, broker-free, at-least-once work queue.
`--lease` atomically moves the bottom item into an inflight sidecar record with
a generated token and expiry deadline, then prints `path<TAB>token`. Files are
re-pointed, never copied. A worker acks on success, nacks to requeue, and any
lease whose deadline passes is auto-returned to the stack on the next `tp`
invocation — so a crashed worker means redelivery, not lost work.

```sh
# lease the bottom item: prints "path<TAB>token", moves it to inflight
read -r path token < <(tp --lease)

# lease with a custom deadline (seconds; default 300)
tp --lease --lease-ttl 30

# acknowledge success: delete the leased file and drop the inflight record
tp --ack "$token"

# negative-ack: return the leased file to the stack for redelivery
tp --nack "$token"

# a simple worker loop draining the queue
while read -r path token < <(tp --lease); do
  [ -z "$path" ] && break
  if process "$path"; then tp --ack "$token"; else tp --nack "$token"; fi
done
```

---

## [0x04] ENVIRONMENT

```sh
# override the default temp directory (default: $TMPDIR/temprs)
export TEMPRS_DIR=/path/to/custom/dir
```

---

## [0x05] DATA INTEGRITY

 ┌──────────────────────────────────────────────────────────────┐
 │ CORRUPTION RESISTANCE: MAXIMUM &nbsp;&nbsp; DATA LOSS RISK: ZERO     │
 └──────────────────────────────────────────────────────────────┘

The master record is hardened against corruption and concurrent access:

- **Null-byte delimited format** // `\0` field separator, `\0\0` record separator — supports filenames with newlines, tabs, spaces, and any special characters
- **Atomic writes** // data is written to a temp file and atomically renamed — no partial writes on crash
- **Exclusive file locking** // `flock`-based locking prevents concurrent access corruption from multiple shells or scripts
- **Auto-recovery** // corrupt or empty records in the master file are silently skipped and cleaned up on next write
- **Inflight leases** // `--lease` moves an item to the `temprs-inflight` sidecar (same atomic-write + `flock` discipline) with a token and deadline; expired leases are swept back onto the stack on the next invocation for at-least-once delivery

---

## [0x06] STACK ARCHITECTURE

```
 ┌─────────────────────────────────────┐
 │  INDEX N   ▓▓  TOP OF STACK (newest)│
 │  INDEX N-1 ▓▓  ...                  │
 │  INDEX 2   ▓▓  ...                  │
 │  INDEX 1   ▓▓  BOTTOM OF STACK      │
 └─────────────────────────────────────┘
```

- Tempfiles are numbered in ascending order // highest index = top of stack
- Negative indices are valid at any `INDEX` position // range: `-stack_size .. -1`
- Positive indices range from `1 .. stack_size`
- Index `0` is always **invalid**
- Both `tp` and `temprs` binaries are installed

---

## [0x07] DEVELOPMENT & CI

Pull requests and pushes to `main` run the workflow in [`.github/workflows/ci.yml`](.github/workflows/ci.yml). You can also run it manually from the repository **Actions** tab (**workflow dispatch**). On a pull request, the **Checks** tab (or the merge box) shows the aggregate status; open the **CI** workflow run for per-job logs (Check, Test, Format, Clippy, Doc, Release Build, Docs gates, Polish gates, Semantic gates, Newline + README gates, Structure gates).

---

## [0xFF] LICENSE

 ┌──────────────────────────────────────────────────────────┐
 │ MIT LICENSE // UNAUTHORIZED REPRODUCTION WILL BE MET     │
 │ WITH FULL ICE                                            │
 └──────────────────────────────────────────────────────────┘

---

```
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
░░ >>> JACK IN. PUSH YOUR DATA. OWN YOUR TEMP FILES. <<<   ░░
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
```

##### created by [MenkeTechnologies](https://github.com/MenkeTechnologies)
