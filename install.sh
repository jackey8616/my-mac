# HomeBrew
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# karabiner-elements: Keyboard customize manager
brew install --cask karabiner-elements

# Install karabiner-elements vim mode config (use browser and paste this command)
karabiner://karabiner/assets/complex_modifications/import?url=https://git.sr.ht/~harmtemolder/karabiner-vim-mode-plus/blob/master/vim_mode_plus.json

# Install karabiner-elements chinese-english switch shortcuts
# Ref: https://www.v2ex.com/t/565667
karabiner://karabiner/assets/complex_modifications/import?url=https://gist.github.com/gxfxyz/e5f2ac1ce4f5053e9fb6608c10609837/raw/1300231ecbf9c7c87713f02c1107b9e0f8cfe553/karabiner_switch_input_source.json

# Amethyst: Till window manager
brew install --cask amethyst
