# NAWS

Search against a wordlist with some _arbitrary_ requirements.

## Install
```bash
cargo install --path .
```

## Usage
```bash
naws [--global <spec>] [--require <letters>] <spec> ...
```

### Options

#### `--global <spec>`

Specify the global search space `<spec>`. See the definition of `<spec>` below. If not specified, the global search space is set to `abcdefghiklmnopqrstuvwxyz`.

#### `--require <letters>`

Specify letters that must occur at least once at any position in the word.

#### `--no-repeats`

Eliminate words with repeated characters.

#### `<spec>`

Defines the possible letters to match. The number of `<spec>` parameters defines how many letters to search for.

For example:
```bash
naws . . . . .
```
lists all five-letter words in the wordlist.

```
naws w . . . rs
```
Lists five-letter words starting with `w` and ending with either `r` or `s`.

The special character `.` matches any letter defined in the `global` search-space. To exclude letters, prefix with `^`.
```bash
naws z . ^aeon
```
List three-letter words starting with `z` and not ending with `a`, `e`, `o`, or `n`.


## Q&As

### Why "NAWS"?
NAWS stands for Not A W***** Solver
