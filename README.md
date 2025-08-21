# rcd
Straight to the point: Matches one or multiple paths and `cd`s into it.

## Prerequisites
```bash
curl -fsSL https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

## Install
From source:
```bash
cargo install --path .
```

## Shell integration (Bash)
Add to `~/.bashrc`:
```bash
rcd() { builtin cd -- "$(command rcd "$@" | head -n1)"; }
```
Reload:
```bash
source ~/.bashrc
```

## Shell integration (Zsh)
Add to `~/.zshrc`:
```zsh
rcd() { builtin cd -- "$(command rcd "$@" | head -n1)"; }
```
Reload:
```zsh
source ~/.zshrc
```

## Shell integration (macOS Bash)
Add to `~/.bashrc`:
```bash
rcd() { builtin cd -- "$(command rcd "$@" | head -n1)"; }
```
Ensure login shells load it:
```bash
echo '[ -f ~/.bashrc ] && . ~/.bashrc' >> ~/.bash_profile
source ~/.bash_profile
```

## Shell integration (Fish)
Create `~/.config/fish/functions/rcd.fish`:
```fish
function rcd; set p (command rcd $argv | head -n 1); test -n "$p"; and cd -- $p; end
```
Reload:
```fish
exec fish
```

## Shell integration (PowerShell, Windows)
Add to `$PROFILE`:
```powershell
function rcd {
  $exe = (Get-Command rcd -CommandType Application).Source
  $p = & $exe @args | Select-Object -First 1
  if ($p) { Set-Location -LiteralPath $p }
}
. $PROFILE
```

## Shell integration (Git Bash / MSYS2 / Cygwin on Windows)
Add to `~/.bashrc`:
```bash
rcd() { builtin cd -- "$(command rcd "$@" | head -n1)"; }
```
Reload:
```bash
source ~/.bashrc
```

## Usage
```bash
rcd <dir_name> [max_depth]
# example:
rcd leet_code 3
rcd leet_code -> with default depth of 5
```
