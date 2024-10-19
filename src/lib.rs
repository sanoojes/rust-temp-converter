pub mod temperature {
    pub mod celsius {
        pub fn convert_from_fahrenheit(value: f32) -> f32 {
            (value - 32.0) * (5.0 / 9.0)
        }

        pub fn convert_from_kelvin(value: f32) -> f32 {
            value - 273.15
        }
    }

    pub mod kelvin {
        pub fn convert_from_celsius(value: f32) -> f32 {
            value + 273.15
        }

        pub fn convert_from_fahrenheit(value: f32) -> f32 {
            (value - 32.0) * (5.0 / 9.0) + 273.15
        }
    }

    pub mod fahrenheit {
        pub fn convert_from_celsius(value: f32) -> f32 {
            (value * 9.0 / 5.0) + 32.0
        }
        pub fn convert_from_kelvin(value: f32) -> f32 {
            (value - 273.15) * (9.0 / 5.0) + 32.0
        }
    }
}
