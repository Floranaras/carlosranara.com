use crate::config::{GITHUB_USERNAME, WORKER_URL};
use gloo_net::http::Request;
use leptos::*;
use shared::github::GitHubStats;

fn format_number(n: u32) -> String {
    if n >= 1_000 {
        format!("{},{:03}", n / 1_000, n % 1_000)
    } else {
        n.to_string()
    }
}

fn lang_color_class(color: &str) -> &'static str {
    match color {
        "rose-pine-foam" => "text-rose-pine-foam",
        "rose-pine-gold" => "text-rose-pine-gold",
        "rose-pine-iris" => "text-rose-pine-iris",
        "rose-pine-rose" => "text-rose-pine-rose",
        "primary" => "text-primary",
        "rose-pine-love" => "text-rose-pine-love",
        "rose-pine-pine" => "text-rose-pine-pine",
        "rose-pine-subtle" => "text-rose-pine-subtle",
        _ => "text-rose-pine-subtle",
    }
}

#[component]
pub fn AboutSection() -> impl IntoView {
    let stats: RwSignal<Option<GitHubStats>> = create_rw_signal(None);

    create_effect(move |_| {
        spawn_local(async move {
            let url = format!("{WORKER_URL}?username={GITHUB_USERNAME}");
            let Ok(resp) = Request::get(&url).send().await else {
                return;
            };
            let Ok(data) = resp.json::<GitHubStats>().await else {
                return;
            };
            stats.set(Some(data));
        });
    });

    view! {
        <section id="about" class="w-full max-w-[1200px] grid grid-cols-1 md:grid-cols-3 gap-8 mb-16">
            <div class="md:col-span-2 flex flex-col gap-4">
                <div class="flex items-center gap-2 text-primary">
                    <span class="material-symbols-outlined">"person"</span>
                    <h2 class="text-xl font-bold uppercase tracking-widest">"About Me"</h2>
                </div>
                <div class="rounded-xl border p-6 transition-all bg-rose-pine-surface/30 border-rose-pine-muted/10">
                    <p class="text-rose-pine-subtle leading-relaxed mb-4">
                        "CS student obsessed with "
                        <span class="text-rose-pine-rose">"Rust"</span>
                        ", low-level systems, linux, and terminal user interfaces. Obsessed with "
                        <span class="text-rose-pine-gold">"Rose Pine"</span>
                        " and minimalist Neovim setups."
                    </p>
                    <div class="flex gap-4 items-center opacity-70">
                        <TechDot color="bg-rose-pine-rose" label="Rust" />
                        <TechDot color="bg-rose-pine-foam" label="C" />
                        <TechDot color="bg-primary" label="Java" />
                    </div>
                </div>
            </div>

            <div class="md:col-span-1">
                <div class="flex items-center gap-2 text-primary mb-4">
                    <span class="material-symbols-outlined">"analytics"</span>
                    <h2 class="text-xl font-bold uppercase tracking-widest">"Stats"</h2>
                </div>
                <div class="rounded-xl border p-6 transition-all bg-rose-pine-surface/30 border-rose-pine-muted/10">
                    {move || match stats.get() {
                        None => view! {
                            <div class="flex flex-col gap-4">
                                <Skeleton />
                                <Skeleton />
                                <div class="pt-4 border-t border-primary/10">
                                    <Skeleton />
                                    <Skeleton />
                                    <Skeleton />
                                </div>
                            </div>
                        }
                        .into_view(),
                        Some(s) => {
                            let repos = s.total_repos.to_string();
                            let contribs = format_number(s.total_contributions);
                            let langs = s.languages.clone();
                            view! {
                                <div>
                                    <StatBar
                                        label="Project Count"
                                        value=repos
                                        bar_color="bg-primary"
                                        text_color="text-primary"
                                        bg_color="bg-primary/10"
                                    />
                                    <StatBar
                                        label="Contributions (Y)"
                                        value=contribs
                                        bar_color="bg-rose-pine-rose"
                                        text_color="text-rose-pine-rose"
                                        bg_color="bg-rose-pine-rose/10"
                                    />
                                    <div class="pt-4 border-t border-primary/10">
                                        <div class="grid grid-cols-2 gap-3">
                                            {langs
                                                .into_iter()
                                                .map(|l| {
                                                    let cls = lang_color_class(&l.color).to_string();
                                                    view! {
                                                        <LangItem
                                                            name=l.name
                                                            pct=l.percentage
                                                            color_class=cls
                                                        />
                                                    }
                                                })
                                                .collect_view()}
                                        </div>
                                    </div>
                                </div>
                            }
                            .into_view()
                        }
                    }}
                </div>
            </div>
        </section>
    }
}

#[component]
fn Skeleton() -> impl IntoView {
    view! {
        <div
            class="animate-pulse h-3 rounded mb-3"
            style="background: color-mix(in srgb, #6e6a86 20%, transparent);"
        ></div>
    }
}

#[component]
fn TechDot(color: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <div class="flex items-center gap-1">
            <span class=format!("w-3 h-3 rounded-full {color}")></span>
            <span class="text-xs">{label}</span>
        </div>
    }
}

#[component]
fn StatBar(
    label: &'static str,
    value: String,
    bar_color: &'static str,
    text_color: &'static str,
    bg_color: &'static str,
) -> impl IntoView {
    view! {
        <div class="mb-6">
            <div class="flex justify-between text-xs mb-2">
                <span class="text-rose-pine-subtle uppercase">{label}</span>
                <span class=format!("font-bold {text_color}")>{value}</span>
            </div>
            <div class=format!("w-full h-1.5 rounded-full overflow-hidden {bg_color}")>
                <div
                    class=format!("h-full transition-all duration-500 {bar_color}")
                    style="width: 85%;"
                ></div>
            </div>
        </div>
    }
}

#[component]
fn LangItem(name: String, pct: u32, color_class: String) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between">
            <span class="text-xs text-rose-pine-subtle truncate pr-2">{name}</span>
            <span class=format!("text-sm font-bold {color_class}")>{pct}"%"</span>
        </div>
    }
}
