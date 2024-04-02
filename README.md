# Tauri_Chess

The goal of this project is to create a Chess game with Tauri. This is meant as an exercise to better understand Rust
and serves also as my first multi-platform application.


## Project initialization

### Installation
```bash
# Install tauri-cli
cargo install tauri-cli

# Create bootstrap for tauri
cargo install create-tauri-app --locked
cargo create-tauri-app

# Install nvm from https://github.com/nvm-sh/nvm
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
# In my case it installed it on .bashrc instead of .zshrc. just copy the configs over and start a new terminal

# Install latest node version
nvm install latest

```

### Start App
Run:
```
cargo tauri dev
```

## Licenses
Note that this project uses the unlicense whereas the dependencies might use different
licenses. [Tauri](https://github.com/tauri-apps/tauri?tab=readme-ov-file#licenses) uses MIT or MIT/Apache 2.0 License.
