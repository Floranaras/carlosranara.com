use leptos::*;
use leptos::html;
use crate::data::{all_projects, Category};

fn category_color(cat: &Category) -> &'static str {
    match cat {
        Category::Systems => "text-rose-pine-foam",
        Category::Games   => "text-rose-pine-rose",
        Category::Tui     => "text-rose-pine-iris",
        Category::Tools   => "text-rose-pine-gold",
        Category::All     => "text-primary",
    }
}

fn category_from_label(cat: &Category) -> &'static str {
    match cat {
        Category::Systems => "from-rose-pine-foam",
        Category::Games   => "from-rose-pine-rose",
        Category::Tui     => "from-rose-pine-iris",
        Category::Tools   => "from-rose-pine-gold",
        Category::All     => "from-primary",
    }
}

#[component]
pub fn Projects() -> impl IntoView {
    let (active_cat, set_active_cat) = create_signal(Category::All);
    let (layout, set_layout) = create_signal("grid");

    let filtered = move || {
        let cat = active_cat.get();
        let projects = all_projects();
        match cat {
            Category::All => projects,
            _ => projects.into_iter().filter(|p| p.category == cat).collect(),
        }
    };

    let featured = move || filtered().into_iter().filter(|p| p.featured).collect::<Vec<_>>();
    let repos    = move || filtered().into_iter().filter(|p| !p.featured).collect::<Vec<_>>();

    view! {
        <div class="max-w-7xl mx-auto w-full animate-fade-in-up">

            // ── Header ────────────────────────────────────────────────────────
            <section class="mb-12">
                <div class="flex flex-col gap-2 border-l-4 border-primary pl-6 py-2">
                    <p class="text-primary text-sm mb-1">"~/root/portfolio"</p>
                    <h1 class="text-4xl md:text-5xl font-black text-rose-pine-text">"/projects_"</h1>
                    <p class="text-rose-pine-subtle max-w-2xl">
                        "A curated collection of low-level systems, experimental games, and developer utilities. Built with performance and aesthetics in mind."
                    </p>
                </div>
            </section>

            // ── Filter ────────────────────────────────────────────────────────
            <section class="mb-12">
                <div class="flex flex-col md:flex-row justify-between items-start md:items-center gap-6 mb-8">
                    <div class="flex gap-2 flex-wrap">
                        {Category::all().into_iter().map(|cat| {
                            let label = cat.label();
                            let cat_clone = cat.clone();
                            view! {
                                <button
                                    class=move || {
                                        if active_cat.get() == cat_clone {
                                            "px-4 py-1 text-xs font-bold border transition-all uppercase border-primary bg-primary text-rose-pine-base".to_string()
                                        } else {
                                            "px-4 py-1 text-xs font-bold border transition-all uppercase border-rose-pine-muted/50 text-rose-pine-subtle hover:border-primary hover:text-primary".to_string()
                                        }
                                    }
                                    on:click=move |_| set_active_cat.set(cat.clone())
                                >
                                    {label}
                                </button>
                            }
                        }).collect_view()}
                    </div>
                    <div class="flex items-center gap-4 text-rose-pine-muted text-xs">
                        <span>"Layout:"</span>
                        <span
                            class=move || if layout.get() == "grid" { "material-symbols-outlined cursor-pointer text-primary" } else { "material-symbols-outlined cursor-pointer hover:text-rose-pine-text" }
                            on:click=move |_| set_layout.set("grid")
                        >"grid_view"</span>
                        <span
                            class=move || if layout.get() == "list" { "material-symbols-outlined cursor-pointer text-primary" } else { "material-symbols-outlined cursor-pointer hover:text-rose-pine-text" }
                            on:click=move |_| set_layout.set("list")
                        >"list"</span>
                    </div>
                </div>
            </section>

            // ── Featured Work ─────────────────────────────────────────────────
            {move || (!featured().is_empty()).then(|| {
                let projects = featured();
                view! {
                    <section class="mb-16">
                        <div class="mb-6 flex items-center gap-2">
                            <span class="font-bold text-rose-pine-gold">">"</span>
                            <h2 class="text-xl font-bold uppercase tracking-widest">"Featured_Work"</h2>
                        </div>
                        <div class=move || if layout.get() == "grid" { "grid grid-cols-1 lg:grid-cols-2 gap-8" } else { "flex flex-col gap-6" }>
                            {projects.into_iter().enumerate().map(|(i, p)| {
                                let color_class = category_color(&p.category);
                                let from_class  = category_from_label(&p.category);
                                let code = format!("0x0{} // {}", i + 1, p.category.label());
                                view! {
                                    <a href=p.github_url target="_blank" rel="noopener noreferrer" class="block group">
                                        <div class="relative p-6 bg-rose-pine-surface/30 transition-all border border-primary/30 hover:bg-rose-pine-surface/50 cursor-pointer">
                                            <span class="absolute -top-1 -left-1 px-1 bg-rose-pine-base text-primary">"+"</span>
                                            <span class="absolute -bottom-1 -right-1 px-1 bg-rose-pine-base text-primary">"+"</span>

                                            <div class="flex flex-col h-full">
                                                <div class="flex justify-between items-start mb-4">
                                                    <span class=format!("text-xs font-bold {color_class}")>{code}</span>
                                                </div>

                                                <div class="aspect-video mb-6 bg-rose-pine-overlay relative overflow-hidden flex items-center justify-center">
                                                    <div class=format!("absolute inset-0 opacity-20 bg-[radial-gradient(circle_at_center,_var(--tw-gradient-stops))] {from_class} via-transparent to-transparent")></div>
                                                        {p.thumbnail.map(|src| {
                                                            if src.ends_with(".webm") {
                                                                let video_ref = create_node_ref::<html::Video>();
                                                                
                                                                // Crucial: Set properties explicitly when the element mounts
                                                                create_effect(move |_| {
                                                                    if let Some(video) = video_ref.get() {
                                                                        video.set_loop(true);
                                                                        video.set_muted(true);
                                                                        let _ = video.play();
                                                                    }
                                                                });

                                                                view! {
                                                                    <video
                                                                        node_ref=video_ref
                                                                        src=src
                                                                        autoplay=true
                                                                        loop=true
                                                                        prop:muted=true
                                                                        playsinline=true
                                                                        preload="auto"
                                                                        class="absolute inset-0 w-full h-full object-cover"
                                                                    ></video>
                                                                }.into_view()
                                                            } else {
                                                                view! {
                                                                    <img src=src alt=p.title class="absolute inset-0 w-full h-full object-cover" />
                                                                }.into_view()
                                                            }
                                                        })}
                                                    </div>

                                                <h3 class="text-2xl font-bold text-rose-pine-text mb-2 group-hover:text-rose-pine-foam transition-colors">
                                                    {p.title}
                                                </h3>
                                                <p class="text-rose-pine-subtle text-sm mb-6 flex-grow leading-relaxed">
                                                    {p.description}
                                                </p>

                                                <div class="flex flex-wrap gap-3 mt-auto">
                                                    {p.tags.iter().map(|tag| view! {
                                                        <span class="text-[10px] border border-rose-pine-muted/30 px-2 py-0.5 text-rose-pine-muted uppercase">
                                                            {*tag}
                                                        </span>
                                                    }).collect_view()}
                                                </div>
                                            </div>
                                        </div>
                                    </a>
                                }
                            }).collect_view()}
                        </div>
                    </section>
                }
            })}

            // ── Other Repos ───────────────────────────────────────────────────
            {move || (!repos().is_empty()).then(|| {
                let repositories = repos();
                view! {
                    <section class="mb-16">
                        <div class="mb-6 flex items-center gap-2">
                            <span class="font-bold text-rose-pine-iris">">"</span>
                            <h2 class="text-xl font-bold uppercase tracking-widest">"Other_Repositories"</h2>
                        </div>
                        <div class=move || if layout.get() == "grid" { "grid gap-6 grid-cols-1 md:grid-cols-2 lg:grid-cols-3" } else { "flex flex-col gap-4" }>
                            {repositories.into_iter().map(|p| {
                                let cat_color = match p.category {
                                    Category::Tools   => "text-rose-pine-gold",
                                    Category::Systems => "text-primary",
                                    Category::Games   => "text-rose-pine-rose",
                                    Category::Tui     => "text-rose-pine-iris",
                                    Category::All     => "text-primary",
                                };
                                view! {
                                    <a href=p.github_url target="_blank" rel="noopener noreferrer" class="block">
                                        <div class="border border-rose-pine-muted/20 p-5 bg-rose-pine-surface/20 hover:border-rose-pine-muted/50 transition-colors">
                                            <div class="flex justify-between mb-3">
                                                <span class="material-symbols-outlined text-rose-pine-muted text-lg">"folder"</span>
                                                <div class="flex gap-2">
                                                    <span class="material-symbols-outlined text-rose-pine-muted text-sm cursor-pointer hover:text-rose-pine-text transition-colors">"code"</span>
                                                </div>
                                            </div>
                                            <h4 class="font-bold text-rose-pine-text mb-2">{p.title}</h4>
                                            <p class="text-rose-pine-muted text-xs mb-4 line-clamp-2">{p.description}</p>
                                            <div class="flex justify-between items-center">
                                                <span class=format!("text-[10px] uppercase font-bold {cat_color}")>
                                                    {p.category.label()}
                                                </span>
                                                <span class="text-[10px] text-rose-pine-subtle">
                                                    {p.language}" / "{p.stars.unwrap_or("0")}" stars"
                                                </span>
                                            </div>
                                        </div>
                                    </a>
                                }
                            }).collect_view()}
                        </div>
                    </section>
                }
            })}

            {move || (filtered().is_empty()).then(|| view! {
                <div class="text-center py-16">
                    <p class="text-rose-pine-subtle text-lg">"No projects found in this category."</p>
                </div>
            })}
        </div>
    }
}
