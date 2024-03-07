use std::ops::Deref;
use leptos::*;
use crate::data::{Coordinates, WeatherRegistry};
use crate::open_meteo::{self, WeatherData};
use crate::util::{AlwaysEqual, NeverEqual};
use std::sync::Arc;

enum ApiCallState {
    NotCalled,
    ResponsePending,
    Error(open_meteo::Error),
    Responded(WeatherData),
}

#[component]
pub fn App(weather_registry: Arc<WeatherRegistry>) -> impl IntoView {
    let (get_coordinates, set_coordinates) = create_signal::<Option<NeverEqual<Coordinates>>>(None);

    let source = move || (AlwaysEqual(weather_registry.clone()), get_coordinates());

    async fn fetcher((weather_registry, coordinates): (AlwaysEqual<Arc<WeatherRegistry>>, Option<NeverEqual<Coordinates>>))
                     -> Option<Result<WeatherData, open_meteo::Error>> {
        let coordinates = coordinates?.into_inner();
        let weather_registry = weather_registry.into_inner();
        let weather_registry = weather_registry.deref();
        let weather_data = open_meteo::call_api(weather_registry, coordinates).await;
        Some(weather_data)
    }

    let weather_data_resource = create_local_resource(
        source,
        fetcher,
    );

    let weather_data_state = move || {
        match weather_data_resource.get() {
            None => ApiCallState::ResponsePending,
            Some(None) => ApiCallState::NotCalled,
            Some(Some(Err(error))) => ApiCallState::Error(error),
            Some(Some(Ok(response))) => ApiCallState::Responded(response),
        }
    };

    view! {
        <Header set_coordinates=set_coordinates/>
        <Main weather_data_state=weather_data_state/>
        <Footer/>
    }
}

#[component]
fn Header(set_coordinates: WriteSignal<Option<NeverEqual<Coordinates>>>) -> impl IntoView {
    use crate::data::{Coordinates, Latitude, Longitude};

    let (get_latitude_text, set_latitude_text) = create_signal(String::default());
    let (get_longitude_text, set_longitude_text) = create_signal(String::default());

    let get_latitude = move || {
        let value = get_latitude_text().parse::<f32>().ok()?;
        Latitude::try_from(value).ok()
    };

    let get_longitude = move || {
        let value = get_longitude_text().parse::<f32>().ok()?;
        Longitude::try_from(value).ok()
    };

    let get_floating_coordinates = move || {
        let latitude = get_latitude()?;
        let longitude = get_longitude()?;
        Some(Coordinates { latitude, longitude })
    };

    view! {
        <header>
            <h1>Stormy Skies</h1>

            <input
                placeholder = "Latitude"
                on:input = move |event| {
                    let value = event_target_value(&event);
                    set_latitude_text(value);
                }
            />

            <input
                placeholder = "Longitude"
                on:input = move |event| {
                    let value = event_target_value(&event);
                    set_longitude_text(value);
                }
            />

            <button
                on:click = move |_| { // TODO add a cool-down for this button to prevent spamming
                    set_coordinates(get_floating_coordinates().map(NeverEqual));
                }
            >Forecast</button>
        </header>
    }
}

#[component]
fn Main<F>(weather_data_state: F) -> impl IntoView where F: Fn() -> ApiCallState + 'static {
    let load_main = move || {
        match weather_data_state() {
            ApiCallState::NotCalled =>
                MainBeforeFirstRequest().into_view(),

            ApiCallState::ResponsePending =>
                MainWhileRequestPending().into_view(),

            ApiCallState::Error(error) =>
                view! { <MainWithError error={ move || error.clone() }/> },

            ApiCallState::Responded(weather_data) =>
                view! { <MainWithLoadedData weather_data={ move || weather_data.clone() } /> },
        }
    };

    view! {
        <main>
            { load_main }
        </main>
    }
}

#[component]
fn MainBeforeFirstRequest() -> impl IntoView {
    view! {
        <h1>No Request made</h1>
    }
}

#[component]
fn MainWhileRequestPending() -> impl IntoView {
    view! {
        <h1>Pending...</h1>
    }
}

#[component]
fn MainWithError<F>(error: F) -> impl IntoView where F: Fn() -> open_meteo::Error + 'static {
    view! {
        <h1>Error: { move || error().to_string() }</h1>
    }
}

#[component]
fn MainWithLoadedData<F>(weather_data: F) -> impl IntoView where F: Fn() -> WeatherData + 'static {
    let weather_data = Signal::derive(weather_data);
    let icon_path = move || weather_data().weather.icon_path.to_string();
    let current_weather_description = move || weather_data().weather.description.to_string();

    view! {
        <img src={ icon_path }/>
        <h1>Current Weather: { current_weather_description }</h1>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <a href="https://open-meteo.com/">Weather data by Open-Meteo.com</a>
    }
}
