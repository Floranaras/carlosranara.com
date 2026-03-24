mod about;
mod featured;
mod hero;

use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="w-full max-w-[1200px] animate-fade-in-up">
            <hero::HeroSection />
            <about::AboutSection />
            <featured::FeaturedProjectsSection />
        </div>
    }
}
