use leptos::*;
use crate::data::timeline;

#[component]
pub fn LinuxJourney() -> impl IntoView {
    let items = timeline();
    let years = 2026 - 2021;

    view! {
        <div class="w-full flex flex-col items-center animate-fade-in-up">

            // ── Header ────────────────────────────────────────────────────────
            <div class="w-full max-w-[900px] text-center mb-20">
                <h1 class="text-4xl md:text-6xl font-black mb-6">
                    "The "
                    <span style="background: linear-gradient(to right, #317590, #9ccfd8); -webkit-background-clip: text; -webkit-text-fill-color: transparent; background-clip: text; color: transparent;">
                        "Evolution"
                    </span>
                    " of my Workspace"
                </h1>
                <p class="text-rose-pine-subtle text-lg leading-relaxed max-w-3xl mx-auto">
                    "From \"What is a terminal?\" to \"Kernel compilation completed in 4 minutes.\" A chronological journey through distributions, rices, and technical growth."
                </p>
                <div class="flex flex-wrap justify-center gap-6 mt-8">
                    <div class="flex flex-col items-center">
                        <span class="text-3xl font-bold text-primary">{items.len().to_string()}</span>
                        <span class="text-xs text-rose-pine-muted uppercase tracking-wider">"Milestones"</span>
                    </div>
                    <div class="flex flex-col items-center">
                        <span class="text-3xl font-bold text-rose-pine-foam">{years.to_string()}</span>
                        <span class="text-xs text-rose-pine-muted uppercase tracking-wider">"Years"</span>
                    </div>
                    <div class="flex flex-col items-center">
                        <span class="text-3xl font-bold text-rose-pine-iris">"∞"</span>
                        <span class="text-xs text-rose-pine-muted uppercase tracking-wider">"Rices"</span>
                    </div>
                </div>
            </div>

            // ── Timeline ──────────────────────────────────────────────────────
            <div class="w-full max-w-[900px] relative">
                // Center line
                <div class="hidden md:block" style="position: absolute; left: 50%; top: 0; bottom: 0; width: 2px; background: linear-gradient(to bottom, rgba(49,117,144,0.3), rgba(49,117,144,0.5), rgba(49,117,144,0.3)); transform: translateX(-50%);"></div>

                <div class="space-y-16">
                    {items.into_iter().enumerate().map(|(i, item)| view! {
                        <TimelineCard item=item index=i />
                    }).collect_view()}
                </div>
            </div>

            // ── Source CTA ────────────────────────────────────────────────────
            <div class="w-full max-w-[600px] text-center mt-32 mb-16 p-8 border border-primary/20 bg-rose-pine-surface/30 rounded-lg hover:border-primary/50 transition-all group">
                <div class="inline-flex items-center justify-center size-16 rounded-full bg-rose-pine-love/10 border border-rose-pine-love/20 mb-4 group-hover:scale-110 transition-transform">
                    <span class="material-symbols-outlined text-3xl text-rose-pine-love group-hover:animate-pulse">"favorite"</span>
                </div>
                <h3 class="text-2xl font-bold text-rose-pine-text mb-3">"Want to see the source?"</h3>
                <p class="text-rose-pine-subtle mb-6 leading-relaxed">
                    "Every version of my Linux Rice is archived. Feel free to fork, steal modules, or get inspired for your next build."
                </p>
                <div class="flex flex-wrap gap-4 justify-center">
                    <a href="https://github.com/Floranaras" target="_blank" rel="noopener noreferrer"
                        class="px-6 py-3 rounded-lg bg-primary hover:bg-primary/80 text-white font-bold text-sm transition-all flex items-center gap-2 hover:scale-105">
                        <span class="material-symbols-outlined text-sm">"hub"</span>
                        "GitHub Repositories"
                    </a>
                    <a href="/contact"
                        class="px-6 py-3 rounded-lg border border-primary text-primary hover:bg-primary/10 font-bold text-sm transition-all flex items-center gap-2 hover:scale-105">
                        <span class="material-symbols-outlined text-sm">"mail"</span>
                        "Contact Me"
                    </a>
                </div>
            </div>
        </div>
    }
}

// ── Timeline Card ─────────────────────────────────────────────────────────────

#[component]
fn TimelineCard(item: crate::data::TimelineItem, index: usize) -> impl IntoView {
    let (expanded, set_expanded) = create_signal(false);
    let is_left = index % 2 == 0;

    let row_class = if is_left {
        "relative flex items-center md:flex-row animate-fade-in-up"
    } else {
        "relative flex items-center md:flex-row-reverse animate-fade-in-up"
    };

    let card_class = if is_left {
        "w-full md:w-[45%] md:pr-12"
    } else {
        "w-full md:w-[45%] md:pl-12"
    };

    // Map dot color string to a Tailwind bg class
    let dot_bg = match item.dot_color {
        c if c.contains("foam")   => "bg-rose-pine-foam",
        c if c.contains("love")   => "bg-rose-pine-love",
        c if c.contains("iris")   => "bg-rose-pine-iris",
        c if c.contains("gold")   => "bg-rose-pine-gold",
        c if c.contains("rose")   => "bg-rose-pine-rose",
        c if c.contains("muted")  => "bg-rose-pine-muted",
        c if c.contains("pine")   => "bg-primary",
        _                          => "bg-primary",
    };

    view! {
        <div class=row_class>
            // Card
            <div class=card_class>
                <div
                    class="border border-primary/20 bg-rose-pine-surface/30 p-6 rounded-lg hover:border-primary/50 transition-all cursor-pointer group"
                    on:click=move |_| set_expanded.update(|v| *v = !*v)
                >
                    <div class="flex items-center gap-2 mb-3">
                        <span class="material-symbols-outlined text-sm text-rose-pine-muted">"calendar_today"</span>
                        <span class="text-[10px] text-rose-pine-muted uppercase tracking-wider">{item.date}</span>
                    </div>

                    <h3 class="text-xl font-bold text-rose-pine-text mb-3 group-hover:text-primary transition-colors">
                        {item.title}
                    </h3>

                    <p class=move || format!(
                        "text-sm text-rose-pine-subtle leading-relaxed mb-4 transition-all {}",
                        if expanded.get() { "" } else { "line-clamp-3" }
                    )>
                        {item.description}
                    </p>

                    {move || expanded.get().then(|| view! {
                        <div class="space-y-4 animate-fade-in">
                            {item.image.map(|src| {
                                if src.ends_with(".webp") || src.ends_with(".png") || src.ends_with(".jpg") {
                                    view! {
                                        <div class="rounded-lg overflow-hidden border border-primary/10">
                                            <img src=src alt=item.title class="w-full h-auto hover:scale-105 transition-transform duration-500" />
                                        </div>
                                    }.into_view()
                                } else {
                                    view! {
                                        <div class="rounded-lg overflow-hidden border border-primary/10">
                                            <video src=src autoplay=true loop_=true muted=true playsinline=true class="w-full h-auto"></video>
                                        </div>
                                    }.into_view()
                                }
                            })}
                            {item.learnings.map(|l| view! {
                                <div class="pl-4 border-l-2 border-primary/30 bg-primary/5 p-3 rounded-r">
                                    <p class="text-xs text-rose-pine-subtle uppercase tracking-wider mb-1">"Lessons Learned"</p>
                                    <p class="text-sm text-rose-pine-text italic">{l}</p>
                                </div>
                            })}
                        </div>
                    })}

                    <div class="flex items-center gap-2 mt-4 text-xs text-rose-pine-muted">
                        <span class="material-symbols-outlined text-sm transition-transform">"expand_more"</span>
                        <span>{move || if expanded.get() { "Show less" } else { "Click to expand" }}</span>
                    </div>
                </div>
            </div>

            // Center dot
            <div class="hidden md:flex absolute left-1/2 -translate-x-1/2 size-4 rounded-full border-2 border-primary/30 z-10 items-center justify-center hover:scale-125 transition-transform bg-rose-pine-base">
                <div class=format!("size-2 rounded-full {dot_bg} animate-pulse")></div>
            </div>
        </div>
    }
}
