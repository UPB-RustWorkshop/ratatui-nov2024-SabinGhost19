use std::error;

/// Application result type.
pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

// /// Application.
 #[derive(Debug)]

pub struct App {
    pub running: bool,
    pub selected_index: usize,
    pub items: Vec<String>,
    pub start_index: usize,
    pub end_index: usize,
}

impl App {
    pub fn new() -> Self {
        //LISTA DE ORASE....TOO MUCH:)
        let items = vec![
            String::from("London"), String::from("New York"), String::from("Tokyo"),
            String::from("Paris"), String::from("Berlin"), String::from("Rome"),
            String::from("Madrid"), String::from("Moscow"), String::from("Sydney"),
            String::from("Beijing"), String::from("Los Angeles"), String::from("Chicago"),
            String::from("Toronto"), String::from("Vancouver"), String::from("Mexico City"),
            String::from("Buenos Aires"), String::from("São Paulo"), String::from("Cape Town"),
            String::from("Cairo"), String::from("Dubai"), String::from("Mumbai"),
            String::from("Bangkok"), String::from("Singapore"), String::from("Hong Kong"),
            String::from("Seoul"), String::from("Kuala Lumpur"), String::from("Istanbul"),
            String::from("Athens"), String::from("Warsaw"), String::from("Budapest"),
            String::from("Prague"), String::from("Vienna"), String::from("Brussels"),
            String::from("Stockholm"), String::from("Oslo"), String::from("Helsinki"),
            String::from("Copenhagen"), String::from("Lisbon"), String::from("Dublin"),
            String::from("Edinburgh"), String::from("Reykjavik"), String::from("Havana"),
            String::from("Santiago"), String::from("Lima"), String::from("Bogota"),
            String::from("Caracas"), String::from("Quito"), String::from("Nairobi"),
            String::from("Casablanca"), String::from("Lagos"),
        ];

        let end_index = if items.len() < 10 { items.len() } else { 10 };
        Self {
            running: true,
            selected_index: 0,
            items,
            start_index: 0,
            end_index,
        }
    }

    pub fn next_city(&mut self) {
        if self.selected_index < self.items.len() - 1 {
            self.selected_index += 1;

            //AJUSTARE FEREASTRA DE VIZUALIZARE
            if self.selected_index >= self.end_index {
                self.start_index += 1;
                self.end_index += 1;
            }
        }
    }

    pub fn previous_city(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;

            //AJUSTARE FEREASTRA DE VIZUALIZARE
            if self.selected_index < self.start_index {
                self.start_index -= 1;
                self.end_index -= 1;
            }
        }
    }
}
