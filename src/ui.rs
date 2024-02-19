use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    use crate::data::Coordinates;
    use crate::open_meteo;

    let test_coordinates = Coordinates::from_latitude_longitude(53.33, 10.00).unwrap();

    let weather_data = create_local_resource(
        move || test_coordinates,
        open_meteo::call_api,
    );

    // TODO add fallback, see https://github.com/leptos-rs/leptos/blob/main/examples/fetch/src/lib.rs

    let weather_data_view = move || {
        weather_data.map(|weather_data| {
            format!("{:?}", weather_data)
        })
    };

    view! { <h1>{ weather_data_view }</h1> }
}
