use crate::configuration::Settings;
use crate::model::Product;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Dell XPS 13 Laptop".to_string(),
            price: 1499.99,
            description: "A lightweight premium laptop with Intel Core processor, 16GB RAM, and 512GB SSD. Ideal for work, study, and everyday productivity.".to_string(),
            image: "/dell-xps13.jpg".to_string(),
        },
        Product {
            id: 2,
            name: "Samsung 27-inch 4K Monitor".to_string(),
            price: 349.99,
            description: "A high-resolution 27-inch UHD monitor with vibrant colors and crisp detail, perfect for office work, streaming, and creative tasks.".to_string(),
            image: "/samsung-monitor.jpg".to_string(),
        },
        Product {
            id: 3,
            name: "Sony WH-1000XM5 Headphones".to_string(),
            price: 499.99,
            description: "Industry-leading noise-cancelling wireless headphones with premium sound quality and long battery life.".to_string(),
            image: "/sony-headphones.jpg".to_string(),
        },
        Product {
            id: 4,
            name: "Apple AirPods Pro".to_string(),
            price: 329.99,
            description: "Wireless earbuds with active noise cancellation, transparency mode, and rich immersive audio experience.".to_string(),
            image: "/airpods-pro.jpg".to_string(),
        },
        Product {
            id: 5,
            name: "Logitech MX Keys Keyboard".to_string(),
            price: 149.99,
            description: "A premium wireless keyboard designed for comfort, precision typing, and multi-device productivity.".to_string(),
            image: "/mx-keys.jpg".to_string(),
        },
        Product {
            id: 6,
            name: "Logitech MX Master 3S Mouse".to_string(),
            price: 129.99,
            description: "An advanced wireless mouse with ergonomic design, fast scrolling, and precise tracking for professional use.".to_string(),
            image: "/mx-master-3s.jpg".to_string(),
        },
        Product {
            id: 7,
            name: "LG 55-inch OLED Smart TV".to_string(),
            price: 1799.99,
            description: "A 55-inch OLED smart TV with stunning picture quality, deep blacks, and built-in streaming apps.".to_string(),
            image: "/lg-oled-tv.jpg".to_string(),
        },
        Product {
            id: 8,
            name: "PlayStation 5 Console".to_string(),
            price: 649.99,
            description: "Next-generation gaming console with ultra-fast load times, immersive graphics, and a powerful gaming experience.".to_string(),
            image: "/ps5.jpg".to_string(),
        },
        Product {
            id: 9,
            name: "Apple iPad Air".to_string(),
            price: 799.99,
            description: "A sleek and powerful tablet with a vivid display, strong performance, and support for productivity and entertainment.".to_string(),
            image: "/ipad-air.jpg".to_string(),
        },
        Product {
            id: 10,
            name: "Anker USB-C Fast Charger".to_string(),
            price: 39.99,
            description: "A compact fast charger with USB-C support, ideal for charging phones, tablets, and other compatible devices.".to_string(),
            image: "/anker-charger.jpg".to_string(),
        },
    ]
}