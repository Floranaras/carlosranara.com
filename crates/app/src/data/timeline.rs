#[derive(Clone)]
pub struct TimelineItem {
    pub date: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub learnings: Option<&'static str>,
    pub image: Option<&'static str>,
    pub dot_color: &'static str,
}

pub fn timeline() -> Vec<TimelineItem> {
    vec![
        TimelineItem {
            date: "EARLY 2021",
            title: "Pop!_OS: The Awakening",
            description: "First distro ever. Moving away from Windows into the world of Linux. \
                Accidentally deleted the entire GUI in my first week.",
            learnings: Some(
                "The power of sudo. With great power comes great responsibility... \
                and broken systems.",
            ),
            image: None,
            dot_color: "bg-rose-pine-foam",
        },
        TimelineItem {
            date: "2021 (~1 MONTH)",
            title: "Linux Mint: Brief Encounter",
            description: "Thought it was ugly. Suffered through performance issues and audio \
                problems. Lasted about a month before switching.",
            learnings: Some(
                "Not every distro is for everyone. Aesthetics matter more than I thought.",
            ),
            image: None,
            dot_color: "bg-rose-pine-love",
        },
        TimelineItem {
            date: "2021",
            title: "Garuda Linux: Discovery",
            description: "Switched after Mint disappointment. First taste of Arch-based systems. \
                The beautiful world of KDE and customization.",
            learnings: Some("Arch-based doesn't mean scary. The AUR is amazing."),
            image: None,
            dot_color: "bg-rose-pine-iris",
        },
        TimelineItem {
            date: "2021 - 2023",
            title: "Garuda Linux: The Golden Years",
            description: "Main PC distro for about 2 years. Learned the ins and outs of Arch, \
                package management, and system customization. This was my daily driver.",
            learnings: Some(
                "BTRFS snapshots save lives. Rolling releases are stable when you \
                know what you're doing.",
            ),
            image: None,
            dot_color: "bg-rose-pine-gold",
        },
        TimelineItem {
            date: "2023 - 2024",
            title: "The Break",
            description: "Stopped using Linux during school. Required software forced me back to \
                other systems. The dark times.",
            learnings: Some(
                "Sometimes you need to use the right tool for the job, even if it's \
                not your favorite.",
            ),
            image: None,
            dot_color: "bg-rose-pine-muted",
        },
        TimelineItem {
            date: "NOVEMBER 2024",
            title: "Linux Mint: The Return",
            description: "Came back to Linux in college. Surprised by how much the Linux scene \
                had evolved. The ecosystem had matured significantly.",
            learnings: Some(
                "The Linux world never stops improving. What was broken is now polished.",
            ),
            image: None,
            dot_color: "bg-rose-pine-foam",
        },
        TimelineItem {
            date: "NOVEMBER 2024",
            title: "Arch + Hyprland: The Aesthetic",
            description: "Decided to go full Arch. Riced with pink Rose Pine aesthetic. Wayland, \
                Hyprland, the works. Dotfiles repo created.",
            learnings: Some("Configuration is an art form. Rice is life."),
            image: Some("/public/screenshots/firstrice.png"),
            dot_color: "bg-rose-pine-rose",
        },
        TimelineItem {
            date: "2025",
            title: "Arch + Hyprland on New Laptop",
            description: "Got a new laptop. Initial setup with Arch and Hyprland. Transferred \
                my dotfiles and rice over.",
            learnings: Some("Dotfiles repos make fresh installs a breeze."),
            image: Some("/public/screenshots/hyprlandarch.png"),
            dot_color: "bg-rose-pine-iris",
        },
        TimelineItem {
            date: "2025",
            title: "Arch + i3: Battery Optimization",
            description: "Switched to i3 on the laptop for better battery life. Optimized for \
                minimal resource usage. New dotfiles repo for this setup.",
            learnings: Some("Sometimes simplicity wins over aesthetics. Battery life matters."),
            image: Some("/public/screenshots/i3rice.png"),
            dot_color: "bg-rose-pine-gold",
        },
        TimelineItem {
            date: "2026",
            title: "Fedora: Pragmatism",
            description: "Current setup. Moved to Fedora for stability and pragmatism. Still \
                Arch at heart, but sometimes you need things to just work.",
            learnings: Some(
                "Maturity means knowing when to choose stability over bleeding edge.",
            ),
            image: None,
            dot_color: "bg-rose-pine-foam",
        },
    ]
}
