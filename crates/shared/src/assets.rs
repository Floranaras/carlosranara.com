pub enum Asset {
    // Profile
    Pfp,
    // GIFs
    CarlOsDemo,
    PythonInterpreter,
    RustTetris,
    Sokobot,
    ValentinesTui,
    // Screenshots
    FirstRice,
    HyprlandArch,
    I3Rice,
    // Documents
    Cv,
}

impl Asset {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Pfp              => "/public/screenshots/pfp.jpg",
            Self::CarlOsDemo       => "/public/gifs/CarlOS-demo.gif",
            Self::PythonInterpreter => "/public/gifs/python-intep.gif",
            Self::RustTetris       => "/public/gifs/tetris-demo.gif",
            Self::Sokobot          => "/public/gifs/sokobot.gif",
            Self::ValentinesTui    => "/public/gifs/valentines.gif",
            Self::FirstRice        => "/public/screenshots/firstrice.png",
            Self::HyprlandArch     => "/public/screenshots/hyprlandarch.png",
            Self::I3Rice           => "/public/screenshots/i3rice.png",
            Self::Cv               => "/public/CV.pdf",
        }
    }
}
