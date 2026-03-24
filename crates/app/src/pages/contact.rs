use leptos::*;

#[component]
pub fn Contact() -> impl IntoView {
    let (current_time, set_current_time) = create_signal(String::new());
    let (copied, set_copied) = create_signal(false);

    create_effect(move |_| {
        let update_time = move || {
            let date = js_sys::Date::new_0();
            let h = date.get_hours();
            let m = date.get_minutes();
            let s = date.get_seconds();
            set_current_time.set(format!("{:02}:{:02}:{:02}", h, m, s));
        };
        update_time();
        let interval = gloo_timers::callback::Interval::new(1000, move || update_time());
        interval.forget();
    });

    let copy_discord = move |_| {
        let _ = js_sys::eval("navigator.clipboard.writeText('musicalflower')");
        set_copied.set(true);
        set_timeout(
            move || set_copied.set(false),
            std::time::Duration::from_millis(2000),
        );
    };

    view! {
        <div class="w-full max-w-[1200px] mx-auto px-4 py-8 md:py-12">

            // Header
            <div class="mb-6 md:mb-8">
                <div class="flex items-center gap-2 text-primary mb-2">
                    <span class="material-symbols-outlined text-xl md:text-2xl">"terminal"</span>
                    <h1 class="text-xl md:text-2xl font-bold uppercase tracking-widest">
                        "/contact_"
                    </h1>
                </div>
                <p class="text-rose-pine-subtle font-mono text-sm md:text-base">
                    "~/root/portfolio/connect"
                </p>
            </div>

            // Main terminal card
            <div class="rounded-xl border p-6 transition-all bg-rose-pine-surface/30 border-rose-pine-muted/10 mb-6 md:mb-8">
                <div class="font-mono mb-6 text-sm md:text-base">
                    <span class="text-primary">"$"</span>
                    <span class="text-rose-pine-text ml-2">"./CONNECT_WITH_ME.sh"</span>
                </div>

                // Contact methods
                <div class="space-y-3 md:space-y-4 mb-6 md:mb-8">
                    // Email
                    <div class="flex items-center gap-3 md:gap-4 p-3 md:p-4 rounded-lg bg-rose-pine-base/50 hover:bg-rose-pine-base/70 transition-colors group">
                        <span class="material-symbols-outlined text-xl md:text-2xl flex-shrink-0 text-rose-pine-foam">
                            "alternate_email"
                        </span>
                        <div class="flex-1 min-w-0">
                            <div class="text-[10px] md:text-xs text-rose-pine-subtle uppercase tracking-wider mb-1">
                                "[EMAIL]"
                            </div>
                            <a
                                href="mailto:carlosranara0@gmail.com"
                                class="font-mono hover:underline text-sm md:text-base break-all text-rose-pine-foam"
                            >
                                "carlosranara0@gmail.com"
                            </a>
                        </div>
                    </div>

                    // GitHub
                    <div class="flex items-center gap-3 md:gap-4 p-3 md:p-4 rounded-lg bg-rose-pine-base/50 hover:bg-rose-pine-base/70 transition-colors group">
                        <span class="material-symbols-outlined text-xl md:text-2xl flex-shrink-0 text-rose-pine-iris">
                            "hub"
                        </span>
                        <div class="flex-1 min-w-0">
                            <div class="text-[10px] md:text-xs text-rose-pine-subtle uppercase tracking-wider mb-1">
                                "[GITHUB]"
                            </div>
                            <a
                                href="https://github.com/Floranaras"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="font-mono hover:underline text-sm md:text-base break-all text-rose-pine-iris"
                            >
                                "github.com/Floranaras"
                            </a>
                        </div>
                    </div>

                    // Discord
                    <div class="flex items-center gap-3 md:gap-4 p-3 md:p-4 rounded-lg bg-rose-pine-base/50 hover:bg-rose-pine-base/70 transition-colors group">
                        <span class="material-symbols-outlined text-xl md:text-2xl flex-shrink-0 text-rose-pine-gold">
                            "forum"
                        </span>
                        <div class="flex-1 min-w-0">
                            <div class="text-[10px] md:text-xs text-rose-pine-subtle uppercase tracking-wider mb-1">
                                "[DISCORD]"
                            </div>
                            <span class="font-mono text-sm md:text-base break-all text-rose-pine-gold">
                                "musicalflower"
                            </span>
                        </div>
                        <button
                            class="opacity-0 group-hover:opacity-100 md:opacity-100 transition-opacity flex-shrink-0 p-2 rounded hover:bg-rose-pine-overlay/30"
                            title="Copy to clipboard"
                            on:click=copy_discord
                        >
                            <span class="material-symbols-outlined text-sm text-rose-pine-subtle">
                                {move || if copied.get() { "check" } else { "content_copy" }}
                            </span>
                        </button>
                    </div>

                    // Resume
                    <div class="flex items-center gap-3 md:gap-4 p-3 md:p-4 rounded-lg bg-rose-pine-base/50 hover:bg-rose-pine-base/70 transition-colors group">
                        <span class="material-symbols-outlined text-xl md:text-2xl flex-shrink-0 text-rose-pine-rose">
                            "description"
                        </span>
                        <div class="flex-1 min-w-0">
                            <div class="text-[10px] md:text-xs text-rose-pine-subtle uppercase tracking-wider mb-1">
                                "[RESUME]"
                            </div>
                            <a
                                href="/public/CV.pdf"
                                download="Carlos Ranara_CV.pdf"
                                target="_blank"
                                class="font-mono hover:underline text-sm md:text-base break-all text-rose-pine-rose"
                            >
                                "Download PDF"
                            </a>
                        </div>
                    </div>
                </div>

                // Available for
                <div class="border-t border-primary/10 pt-4 md:pt-6 mb-4 md:mb-6">
                    <div class="text-xs md:text-sm text-rose-pine-subtle mb-3 font-mono">
                        <span class="text-primary">">"</span>
                        " Available for:"
                    </div>
                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-2">
                        {[
                            "Open source contributions",
                            "Rust projects",
                            "System programming",
                            "Terminal UI development",
                            "Web development",
                            "Anything challenging",
                        ]
                            .iter()
                            .map(|item| {
                                view! {
                                    <div class="flex items-center gap-2 text-rose-pine-text">
                                        <span class="text-primary flex-shrink-0">"•"</span>
                                        <span class="text-xs md:text-sm">{*item}</span>
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>
                </div>

                // Info rows
                <div class="border-t border-primary/10 pt-4 md:pt-6 space-y-3 md:space-y-2">
                    <div class="flex flex-col md:flex-row md:items-center gap-1 md:gap-2 text-xs md:text-sm">
                        <div class="flex items-center gap-2">
                            <span class="text-primary font-mono">">"</span>
                            <span class="text-rose-pine-subtle">"Response time:"</span>
                        </div>
                        <span class="text-rose-pine-text md:ml-0 pl-6 md:pl-0">
                            "Usually within 24-48h"
                        </span>
                    </div>
                    <div class="flex flex-col md:flex-row md:items-center gap-1 md:gap-2 text-xs md:text-sm">
                        <div class="flex items-center gap-2">
                            <span class="text-primary font-mono">">"</span>
                            <span class="text-rose-pine-subtle">"Timezone:"</span>
                        </div>
                        <span class="text-rose-pine-text md:ml-0 pl-6 md:pl-0">
                            "PHT (UTC+8)"
                            {move || {
                                let t = current_time.get();
                                if t.is_empty() { String::new() } else { format!(" • {}", t) }
                            }}
                        </span>
                    </div>
                    <div class="flex flex-col md:flex-row md:items-center gap-1 md:gap-2 text-xs md:text-sm">
                        <div class="flex items-center gap-2">
                            <span class="text-primary font-mono">">"</span>
                            <span class="text-rose-pine-subtle">"Philosophy:"</span>
                        </div>
                        <span class="text-rose-pine-rose md:ml-0 pl-6 md:pl-0">
                            "I love challenges and I'm down for anything"
                        </span>
                    </div>
                </div>
            </div>

            // Status card
            <div class="rounded-xl border p-6 transition-all bg-rose-pine-surface/20 border-rose-pine-muted/10">
                <div class="font-mono text-xs md:text-sm space-y-1">
                    <div class="text-rose-pine-subtle">
                        <span class="text-primary">"$"</span>
                        <span class="ml-2">"status"</span>
                    </div>
                    <div class="text-rose-pine-foam pl-4">"✓ Available for collaboration"</div>
                    <div class="text-rose-pine-gold pl-4">"✓ Open to new opportunities"</div>
                    <div class="text-rose-pine-iris pl-4">"✓ Always learning something new"</div>
                </div>
            </div>
        </div>
    }
}
