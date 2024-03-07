use std::rc::Rc;
use leptos::*;
use stormy_skies::data::WeatherRegistry;

fn main() {
    console_error_panic_hook::set_once();
    let weather_registry = Rc::new(WeatherRegistry::load().unwrap()); // TODO graceful error handling
    mount_to_body(|| view! { <stormy_skies::ui::App weather_registry=weather_registry />} )
}
