#[derive(Clone, PartialEq)]
pub enum Category {
    All,
    Systems,
    Games,
    Tui,
    Tools,
}

impl Category {
    pub fn label(&self) -> &'static str {
        match self {
            Category::All => "ALL",
            Category::Systems => "SYSTEMS",
            Category::Games => "GAMES",
            Category::Tui => "TUI",
            Category::Tools => "TOOLS",
        }
    }

    pub fn all() -> Vec<Category> {
        vec![
            Category::All,
            Category::Systems,
            Category::Games,
            Category::Tui,
            Category::Tools,
        ]
    }
}

#[derive(Clone)]
pub struct Project {
    pub title: &'static str,
    pub description: &'static str,
    pub category: Category,
    pub tags: &'static [&'static str],
    pub language: &'static str,
    pub github_url: &'static str,
    pub featured: bool,
    pub thumbnail: Option<&'static str>,
    pub stars: Option<&'static str>,
}

pub fn all_projects() -> Vec<Project> {
    vec![
        Project {
            title: "CarlOS",
            description: "A high-performance kernel written entirely in Rust. Focusing on memory \
                safety and extreme minimalist overhead. Features custom bootloader, VGA text mode \
                driver, and interrupt handling.",
            category: Category::Systems,
            tags: &["Rust", "x86_64", "QEMU", "Kernel Dev"],
            language: "Rust",
            github_url: "https://github.com/Floranaras/CarlOS",
            featured: true,
            thumbnail: Some("/public/gifs/CarlOS-demo.gif"),
            stars: None,
        },
        Project {
            title: "Python Interpreter",
            description: "A Python interpreter implementation written in C that supports core \
                Python language features including functions, recursion, control flow, and \
                Python's indentation-based syntax.",
            category: Category::Systems,
            tags: &["C", "Interpreter", "Systems"],
            language: "C",
            github_url: "https://github.com/Floranaras/Python-compiler",
            featured: true,
            thumbnail: Some("/public/gifs/python-intep.gif"),
            stars: None,
        },
        Project {
            title: "Rust Tetris",
            description: "Classic Tetris implementation in Rust with terminal rendering. Features \
                smooth piece rotation, score tracking, and increasing difficulty. Built for \
                maximum performance and minimal dependencies.",
            category: Category::Games,
            tags: &["Rust", "Terminal", "Game Dev"],
            language: "Rust",
            github_url: "https://github.com/Floranaras/rust-tetris",
            featured: true,
            thumbnail: Some("/public/gifs/tetris-demo.gif"),
            stars: None,
        },
        Project {
            title: "Sokobot",
            description: "A state-based classical artificial intelligence used to solve various \
                maps for the Japanese puzzle game Sokoban. Started as a requirement that turned \
                into a chronic obsession with low level optimization.",
            category: Category::Games,
            tags: &["State-based", "Intelligent Sys", "Game Dev"],
            language: "C",
            github_url: "https://github.com/Floranaras/SokoBot",
            featured: true,
            thumbnail: Some("/public/gifs/sokobot.gif"),
            stars: None,
        },
        Project {
            title: "Valentine's Day TUI",
            description: "A terminal-based Valentine's Day visualizer built with Ratatui. \
                Featured on AUR (Arch User Repository). High-performance ASCII rendering with \
                aesthetic gradients and animations.",
            category: Category::Tui,
            tags: &["Ratatui", "Yay/AUR", "TUI", "Rust"],
            language: "Rust",
            github_url: "https://github.com/Floranaras/funnyvalentine",
            featured: true,
            thumbnail: Some("/public/gifs/valentines.gif"),
            stars: None,
        },
        Project {
            title: "Dotfiles Hyprland",
            description: "Rose Pine themed Hyprland configuration with custom Waybar, Rofi, and \
                terminal setup. Optimized for aesthetics and workflow efficiency.",
            category: Category::Tools,
            tags: &["Hyprland", "Wayland", "Rice", "Dotfiles"],
            language: "Shell",
            github_url: "https://github.com/Floranaras/dotfiles-hyprland",
            featured: false,
            thumbnail: None,
            stars: None,
        },
        Project {
            title: "Dotfiles i3",
            description: "Battery-optimized i3 configuration for laptops. Minimalist setup \
                focused on resource efficiency while maintaining the Rose Pine aesthetic.",
            category: Category::Tools,
            tags: &["i3wm", "X11", "Battery Life", "Dotfiles"],
            language: "Shell",
            github_url: "https://github.com/Floranaras/dotfiles-i3",
            featured: false,
            thumbnail: None,
            stars: None,
        },
        Project {
            title: "Neovim Config",
            description: "Highly customized Neovim setup with LSP, Treesitter, and Rose Pine \
                theme. Optimized for Rust and web development.",
            category: Category::Tools,
            tags: &["Neovim", "Lua", "LSP", "Rose Pine"],
            language: "Lua",
            github_url: "https://github.com/Floranaras/dotfiles",
            featured: false,
            thumbnail: None,
            stars: None,
        },
    ]
}

pub fn featured_projects() -> Vec<Project> {
    all_projects().into_iter().filter(|p| p.featured).collect()
}
