mod about;
mod featured;
mod hero;

use about::AboutSection;
use featured::FeaturedProjectsSection;
use hero::HeroSection;
use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="w-full max-w-[1200px] animate-fade-in-up">
            <HeroSection />
            <AboutSection />
            <FeaturedProjectsSection />
        </div>
    }
}
