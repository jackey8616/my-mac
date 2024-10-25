# HomeBrew
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# karabiner-elements: Keyboard customize manager
brew install --cask karabiner-elements

# Install karabiner-elements vim mode config (use browser and paste this command)
karabiner://karabiner/assets/complex_modifications/import?url=https://raw.githubusercontent.com/jackey8616/my-mac/refs/heads/main/karabiner-import-config/vim.json

# Install karabiner-elements chinese-english switch shortcuts
# Ref: https://www.v2ex.com/t/565667
karabiner://karabiner/assets/complex_modifications/import?url=https://raw.githubusercontent.com/jackey8616/my-mac/refs/heads/main/karabiner-import-config/chinese-input.json

# Amethyst: Till window manager
brew install --cask amethyst
