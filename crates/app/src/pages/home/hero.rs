use leptos::*;

#[component]
pub fn HeroSection() -> impl IntoView {
    view! {
        <section class="w-full max-w-[1200px] grid grid-cols-1 lg:grid-cols-2 gap-12 items-center mb-16">
            <div class="order-2 lg:order-1 flex flex-col gap-6">
                <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-primary/10 text-primary text-xs font-bold border border-primary/20 w-fit">
                    <span class="relative flex h-2 w-2">
                        <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-primary opacity-75"></span>
                        <span class="relative inline-flex rounded-full h-2 w-2 bg-primary"></span>
                    </span>
                    "v2.0.24-stable"
                </div>

                <h1 class="text-4xl md:text-7xl font-black leading-tight tracking-tighter text-rose-pine-text">
                    "RAMIL "
                    <span class="text-primary">"CARLOS RANARA"</span>
                </h1>

                <p class="text-rose-pine-subtle text-lg max-w-xl leading-relaxed">
                    "Bringing back the ASCII art era philosophy."
                    <br />
                    <span class="text-rose-pine-iris italic">
                        "A portfolio made by the terminal-obsessed."
                    </span>
                </p>

                <div class="flex flex-wrap gap-3">
                    <a
                        href="https://github.com/AmaneKai/dotfiles"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        <button class="px-4 py-2 rounded-lg font-bold flex items-center gap-2 transition-all bg-primary hover:bg-primary/90 text-white text-sm">
                            <span class="material-symbols-outlined">"download"</span>
                            "VIEW_DOTFILES.sh"
                        </button>
                    </a>
                    <a
                        href="https://github.com/AmaneKai/carlosranara.com"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        <button class="px-4 py-2 rounded-lg font-bold flex items-center gap-2 transition-all bg-rose-pine-overlay hover:bg-rose-pine-surface text-rose-pine-text text-sm">
                            <span class="material-symbols-outlined">"code"</span>
                            "SRC_CODE"
                        </button>
                    </a>
                </div>
            </div>

            <div class="order-1 lg:order-2 flex justify-center items-center">
                <div class="relative group">
                    <div class="absolute -inset-4 bg-gradient-to-br from-primary/20 via-rose-pine-iris/20 to-primary/20 rounded-full blur-2xl group-hover:blur-3xl transition-all duration-500 opacity-75 group-hover:opacity-100"></div>
                    <div style="
                        position: relative; border-radius: 9999px; overflow: hidden;
                        border: 4px solid color-mix(in srgb, #317590 30%, transparent);
                        background: color-mix(in srgb, #1f1d2e 50%, transparent);
                        width: clamp(10rem, 40vw, 24rem);
                        height: clamp(10rem, 40vw, 24rem);
                    ">
                        <img
                            src="/public/screenshots/pfp.webp"
                            srcset="/public/screenshots/pfp.webp 1x"
                            alt="Carlos Ranara"
                            width="366"
                            height="274"
                            loading="eager"
                            fetchpriority="high"
                            class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500"
                        />
                        <div class="absolute inset-0 bg-gradient-to-t from-rose-pine-base/20 to-transparent pointer-events-none"></div>
                    </div>
                    <div class="absolute inset-0 rounded-full border-2 border-primary/10 animate-pulse"></div>
                </div>
            </div>
        </section>
    }
}
