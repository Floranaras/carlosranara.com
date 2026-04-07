use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    let (menu_open, set_menu_open) = create_signal(false);

    view! {
        <header class="sticky top-0 z-50 flex items-center justify-between px-6 py-4 border-b border-primary/20 bg-rose-pine-base backdrop-blur-sm shadow-sm">
            <a
                href="/"
                class="flex items-center gap-4 hover:opacity-80 transition-opacity cursor-pointer"
                style="text-decoration: none; color: inherit;"
            >
                <span class="material-symbols-outlined text-primary">"terminal"</span>
                <h2 class="text-lg font-bold leading-tight uppercase tracking-[0.2em]">
                    "Ranara"
                </h2>
            </a>

            <div class="flex flex-1 justify-end gap-8 items-center">
                <nav class="hidden md:flex items-center gap-9">
                    <a
                        href="/#about"
                        class="text-rose-pine-subtle hover:text-primary text-sm font-medium transition-colors"
                        style="text-decoration: none;"
                    >
                        "~/about"
                    </a>
                    <a
                        href="/projects"
                        class="text-rose-pine-subtle hover:text-primary text-sm font-medium transition-colors"
                        style="text-decoration: none;"
                    >
                        "~/projects"
                    </a>
                    <a
                        href="/linux-journey"
                        class="text-rose-pine-subtle hover:text-primary text-sm font-medium transition-colors"
                        style="text-decoration: none;"
                    >
                        "~/timeline"
                    </a>
                    <a
                        href="/contact"
                        class="text-rose-pine-subtle hover:text-primary text-sm font-medium transition-colors"
                        style="text-decoration: none;"
                    >
                        "~/contact"
                    </a>
                </nav>

                <a
                    href="https://github.com/AmaneKai"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="min-w-[84px] items-center justify-center rounded border border-primary text-primary hover:bg-primary/10 h-10 px-4 text-sm font-bold transition-all hidden md:flex"
                    style="text-decoration: none;"
                >
                    <span class="truncate">"ls -a GitHub"</span>
                </a>

                <button
                    class="md:hidden text-primary size-8 flex items-center justify-center"
                    style="background: none; border: none; cursor: pointer;"
                    aria-label="Toggle menu"
                    on:click=move |_| set_menu_open.update(|v| *v = !*v)
                >
                    <span class="material-symbols-outlined">
                        {move || if menu_open.get() { "close" } else { "menu" }}
                    </span>
                </button>
            </div>

            {move || {
                menu_open
                    .get()
                    .then(|| {
                        view! {
                            <div class="absolute top-full left-0 right-0 md:hidden bg-background-dark/95 backdrop-blur-md border-b border-primary/20">
                                <nav class="flex flex-col px-6 py-4">
                                    {[
                                        ("~/about", "/#about"),
                                        ("~/projects", "/projects"),
                                        ("~/timeline", "/linux-journey"),
                                        ("~/contact", "/contact"),
                                    ]
                                        .iter()
                                        .map(|(label, href)| {
                                            view! {
                                                <a
                                                    href=*href
                                                    class="text-rose-pine-subtle hover:text-primary py-3 border-b text-sm font-medium transition-colors border-primary/10 last:border-0"
                                                    style="text-decoration: none;"
                                                    on:click=move |_| set_menu_open.set(false)
                                                >
                                                    {*label}
                                                </a>
                                            }
                                        })
                                        .collect_view()}
                                    <a
                                        href="https://github.com/AmaneKai"
                                        target="_blank"
                                        class="mt-4 min-w-[84px] flex items-center justify-center rounded border border-primary text-primary hover:bg-primary/10 h-10 px-4 text-sm font-bold transition-all"
                                        style="text-decoration: none;"
                                    >
                                        "ls -a GitHub"
                                    </a>
                                </nav>
                            </div>
                        }
                    })
            }}
        </header>
    }
}
