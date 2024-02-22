use leptos::*;
use crate::data::Coordinates;
use crate::open_meteo::{self, WeatherData};
use crate::util::NeverEqual;

type WeatherDataResource = Resource<Option<NeverEqual<Coordinates>>, Option<Result<WeatherData, open_meteo::Error>>>;

#[component]
pub fn App() -> impl IntoView {
    use crate::data::Coordinates;
    use crate::open_meteo::{self, WeatherData};

    let (get_coordinates, set_coordinates) = create_signal::<Option<NeverEqual<Coordinates>>>(None);

    async fn fetch_weather_data(coordinates: Option<NeverEqual<Coordinates>>) -> Option<Result<WeatherData, open_meteo::Error>> {
        Some(open_meteo::call_api(coordinates?.into_inner()).await)
    }

    let weather_data = create_local_resource(
        get_coordinates,
        fetch_weather_data,
    );

    // TODO add fallback, see https://github.com/leptos-rs/leptos/blob/main/examples/fetch/src/lib.rs

    view! {
        <Header set_coordinates=set_coordinates/>
        <Main weather_data=weather_data/>
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
                on:click = move |_| {
                    set_coordinates(get_floating_coordinates().map(NeverEqual));
                }
            >Forecast</button>
        </header>
    }
}

#[component]
fn Main(weather_data: WeatherDataResource) -> impl IntoView {
    view! {
        <main>
            <CurrentWeatherCard weather_data=weather_data/>
        </main>
    }
}

#[component]
fn CurrentWeatherCard(weather_data: WeatherDataResource) -> impl IntoView {
    let weather_data_str = move || weather_data.map(|weather_data| {
        match weather_data {
            Some(Ok(weather_data)) => format!("Weather Code: {}", weather_data.weather.wmo_code()),
            Some(Err(error)) => format!("Error loading the data: {}", error),
            None => "Enter your coordinates above".to_owned(),
        }
    });

    view! {
        <div>
            <h1>{ weather_data_str }</h1>
        </div>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <a href="https://open-meteo.com/">Weather data by Open-Meteo.com</a>
    }
}
