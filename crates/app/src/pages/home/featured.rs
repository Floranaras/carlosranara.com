use crate::data::featured_projects;
use leptos::*;

fn tag_classes(index: usize) -> (&'static str, &'static str, &'static str) {
    match index % 3 {
        0 => ("bg-rose-pine-rose/20", "text-rose-pine-rose", "border-rose-pine-rose/30"),
        1 => ("bg-rose-pine-iris/20", "text-rose-pine-iris", "border-rose-pine-iris/30"),
        _ => ("bg-rose-pine-foam/20", "text-rose-pine-foam", "border-rose-pine-foam/30"),
    }
}

#[component]
pub fn FeaturedProjectsSection() -> impl IntoView {
    let projects = featured_projects();

    view! {
        <section class="w-full max-w-[1200px] mb-16" id="projects">
            <div class="flex items-center justify-between mb-8">
                <div class="flex items-center gap-2 text-primary">
                    <span class="material-symbols-outlined">"folder_open"</span>
                    <h2 class="text-xl font-bold uppercase tracking-widest">"Featured Projects"</h2>
                </div>
                <a
                    href="/projects"
                    class="text-xs text-rose-pine-muted hover:text-primary transition-colors font-mono"
                >
                    "view_all_git.sh"
                </a>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                {projects
                    .into_iter()
                    .map(|p| {
                        view! {
                            <div class="group border border-primary/20 hover:border-primary/50 bg-rose-pine-surface/30 p-6 rounded-lg transition-all">
                                <div class="flex justify-between items-start mb-4">
                                    <h3 class="text-xl font-bold text-rose-pine-text">{p.title}</h3>
                                    <a
                                        href=p.github_url
                                        target="_blank"
                                        rel="noopener noreferrer"
                                        class="text-primary hover:scale-110 transition-transform"
                                    >
                                        <span class="material-symbols-outlined">"open_in_new"</span>
                                    </a>
                                </div>
                                <p class="text-rose-pine-subtle text-sm mb-6 leading-relaxed">
                                    {p.description}
                                </p>
                                <div class="flex flex-wrap gap-2">
                                    {p.tags
                                        .iter()
                                        .enumerate()
                                        .map(|(i, tag)| {
                                            let (bg, text, border) = tag_classes(i);
                                            view! {
                                                <span class=format!(
                                                    "text-[10px] px-2 py-0.5 rounded border {bg} {text} {border}",
                                                )>{*tag}</span>
                                            }
                                        })
                                        .collect_view()}
                                </div>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}
