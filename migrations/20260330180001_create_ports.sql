-- Create ports table with foreign key to categories
CREATE TABLE ports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    author VARCHAR(255) NOT NULL,
    url VARCHAR(512) NOT NULL,
    category VARCHAR(100) NOT NULL REFERENCES categories(id) ON DELETE RESTRICT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index on category for filtering
CREATE INDEX idx_ports_category ON ports(category);

-- Create index on created_at for sorting
CREATE INDEX idx_ports_created_at ON ports(created_at DESC);

-- Insert sample data
INSERT INTO ports (name, description, author, url, category) VALUES
    ('Neovim', 'Official Gruvbox colorscheme for Neovim with full treesitter support and semantic highlighting.', 'morhetz', 'https://github.com/morhetz/gruvbox', 'editor'),
    ('VS Code', 'Gruvbox theme for Visual Studio Code with dark and light variants. Carefully crafted for optimal readability.', 'jdinhlife', 'https://github.com/jdinhlife/vscode-theme-gruvbox', 'editor'),
    ('Alacritty', 'Gruvbox color scheme for Alacritty terminal emulator. Simple configuration with true colors.', 'alacritty', 'https://github.com/alacritty/alacritty-theme', 'terminal'),
    ('Kitty', 'Warm and cozy Gruvbox theme for Kitty terminal. Includes both dark and light variants.', 'wdomitrz', 'https://github.com/wdomitrz/kitty-gruvbox-theme', 'terminal'),
    ('iTerm2', 'Gruvbox color preset for iTerm2 on macOS. Easy installation via color profile import.', 'herrbischoff', 'https://github.com/herrbischoff/iterm2-gruvbox', 'terminal'),
    ('Sublime Text', 'Gruvbox theme for Sublime Text with custom UI elements and syntax highlighting.', 'Briles', 'https://github.com/Briles/gruvbox', 'editor'),
    ('Emacs', 'Gruvbox theme for Emacs with extensive customization options and mode-specific colors.', 'greduan', 'https://github.com/greduan/emacs-theme-gruvbox', 'editor'),
    ('Tmux', 'Minimal Gruvbox theme for Tmux with status bar customization and pane borders.', 'egel', 'https://github.com/egel/tmux-gruvbox', 'terminal');
