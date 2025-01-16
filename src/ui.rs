use ratatui::Frame;
use crate::app::App;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, canvas::{Canvas, Map, MapResolution, Circle}};
use crate::connection::get_data;
use std::error::Error;
use tokio;
use ratatui::style::{Color, Modifier, Style};

pub fn render(app: &mut App, frame: &mut Frame) {
    // Split the layout into multiple areas.
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(20), 
                Constraint::Percentage(40), 
                Constraint::Percentage(40),  
            ]
            .as_ref(),
        )
        .split(frame.size());

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(60),  
                Constraint::Percentage(40),  
            ]
            .as_ref(),
        )
        .split(main_chunks[1]);

    // render lista de orase
    let city_items: Vec<ListItem> = app.items[app.start_index..app.end_index]
        .iter()
        .enumerate()
        .map(|(i, city)| {
            let actual_index = app.start_index + i;
            let mut item = ListItem::new(city.clone());
            if actual_index == app.selected_index {
                item = item.style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
            }
            item
        })
        .collect();

    let list_component = List::new(city_items)
        .block(Block::default().title("Cities").borders(Borders::ALL));
    frame.render_widget(list_component, main_chunks[0]);

   
    let (weather_info_text, humidity, latitude, longitude, sunrise, sunset) = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            match get_data(&app.items[app.selected_index]).await {
                Ok(city_info) => {
                    let weather_details = format!(
                        "City: {}\n\
                         Temperature: {}°C\n\
                         Feels Like: {}°C\n\
                         Description: {}\n\
                         Humidity: {}%\n\
                         Wind Speed: {} m/s\n\
                         Pressure: {} hPa\n\
                         Latitude: {}\n\
                         Longitude: {}\n\
                         Visibility: {} meters\n\
                         Last Updated: {}",
                        city_info.name,
                        city_info.temperature,
                        city_info.feels_like,
                        city_info.description,
                        city_info.humidity,
                        city_info.wind_speed,
                        city_info.pressure,
                        city_info.latitude,
                        city_info.longitude,
                        city_info.visibility.unwrap_or(0),
                        city_info.last_updated.format("%Y-%m-%d %H:%M:%S")
                    );
                    (weather_details, city_info.humidity, city_info.latitude, city_info.longitude, city_info.sunrise, city_info.sunset)
                }
                Err(err) => {
                    let error_message = format!("Failed to get weather data: {}", err);
                    (error_message, 0, 0.0, 0.0, None, None) 
                }
            }
        })
    });

    // AFISARE INFO METEO
    let weather_info_component = Paragraph::new(weather_info_text)
        .block(Block::default().title("Weather Info").borders(Borders::ALL));
    frame.render_widget(weather_info_component, right_chunks[0]);

    // UMIDITATE ICONS
    let humidy_text = if humidity > 60 {
        "\n\n\n💧💧💧\n💧💧💧\n💧💧💧"
    } else {
        "\n\n\n☀ ☀ ☀\n☀ ☀ ☀\n☀ ☀ ☀"
    };

    let humidy_component = Paragraph::new(humidy_text)
        .block(Block::default().title("Weather Condition").borders(Borders::ALL)).alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(humidy_component, right_chunks[1]);


    let globe_component = Canvas::default()
        .block(Block::default().title("Location on Globe").borders(Borders::ALL))
        .marker(ratatui::symbols::Marker::Braille)  
        .paint(move |ctx| {
            ctx.draw(&Map {
                color: Color::Green,
                resolution: MapResolution::High,
            });

            ctx.draw(&Circle {
                x: longitude,
                y: latitude,
                radius: 3.0, 
                color: Color::Yellow,
            });

          
            ctx.print(longitude, latitude, "📍");
        })
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0]);

    frame.render_widget(globe_component, main_chunks[2]);
}
