use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="border-t border-primary/20 px-6 py-8 mt-24">
            <div class="max-w-5xl mx-auto flex flex-col md:flex-row justify-between items-center gap-8">
                <div class="flex items-center gap-3">
                    <span class="material-symbols-outlined text-primary">"terminal"</span>
                    <p class="text-xs text-rose-pine-subtle">
                        "© 2024 Built with Rose Pine & Rust Philosophy"
                    </p>
                </div>

                <div class="flex gap-6">
                    <SocialLink href="https://github.com/AmaneKai" icon="hub" label="github" />
                    <SocialLink
                        href="https://www.youtube.com/@Amane-Kai"
                        icon="video_library"
                        label="youtube"
                    />
                    <SocialLink
                        href="https://github.com/AmaneKai/dotfiles/tree/master/nvim/.config/nvim"
                        icon="settings_ethernet"
                        label="neovim"
                    />
                    <SocialLink href="/contact" icon="alternate_email" label="contact" />
                </div>

                <div class="text-right flex flex-col items-end gap-1">
                    <div class="text-[10px] text-rose-pine-subtle uppercase">"Kernel Version"</div>
                    <div class="text-xs font-mono text-primary px-2 py-0.5 rounded border border-primary/20 bg-primary/5">
                        "6.8.9-arch1-1"
                    </div>
                </div>
            </div>
        </footer>
    }
}

#[component]
fn SocialLink(href: &'static str, icon: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <a
            href=href
            target="_blank"
            rel="noopener noreferrer"
            class="text-rose-pine-subtle transition-colors flex flex-col items-center gap-1 group"
            style="text-decoration: none;"
        >
            <span class="material-symbols-outlined text-xl">{icon}</span>
            <span class="text-[10px] uppercase font-bold tracking-tighter transition-opacity opacity-0 group-hover:opacity-100">
                {label}
            </span>
        </a>
    }
}
