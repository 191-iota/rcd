# rcd
`rcd` matches one or more paths and `cd`s into the first match.

## Performance
With multithreading enabled via `WalkBuilder`, performance has **significantly increased**.  
For example, with a default scan depth of 5 and matching at depth 3:  

- **Before:** 107 ms (single-threaded) 
- **After:** 8 ms (multi-threaded)

| Before | After |
|--------|-------|
| ![Before](assets/perf_before.png) | ![After](assets/perf_after.png) |

note: Tests were run on a Ryzen 9 3900X.

## Install
```bash
cargo install --path .
```

## Shell Integration

### Bash
Add to `~/.bashrc`:
```bash
rcd() { builtin cd -- "$(command rcd "$@" | head -n1)"; }
```
Reload:
```bash
source ~/.bashrc
```

### Zsh
Add to `~/.zshrc`:
```zsh
rcd() { builtin cd -- "$(command rcd "$@" | head -n1)"; }
```
Reload:
```zsh
source ~/.zshrc
```

## Usage
```bash
rcd <dir_name> [max_depth]
# examples:
rcd leet_code 3
rcd leet_code   # default depth = 5
```
