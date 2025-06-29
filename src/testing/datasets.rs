use crate::testing::generators::*;
use chrono::{DateTime, Utc, Duration, TimeZone};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

pub struct StockPrice {
    pub date: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

pub struct ServerMetric {
    pub timestamp: DateTime<Utc>,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
}

pub struct SalesData {
    pub month: String,
    pub revenue: f64,
    pub units_sold: f64,
}

pub struct IrisLikeData {
    pub sepal_length: f64,
    pub sepal_width: f64,
    pub petal_length: f64,
    pub petal_width: f64,
    pub species: String,
}

pub fn stock_prices(days: usize, initial_price: f64, volatility: f64) -> Vec<StockPrice> {
    let mut rng = StdRng::seed_from_u64(42);
    let mut prices = Vec::with_capacity(days);
    let mut current_price = initial_price;
    
    let start_date = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    
    for i in 0..days {
        let date = start_date + Duration::try_days(i as i64).unwrap();
        
        // Random walk with volatility
        let change_percent = (rng.gen::<f64>() - 0.5) * volatility;
        current_price *= 1.0 + change_percent;
        
        // Generate OHLC data
        let daily_volatility = volatility * 0.5;
        let high_change = rng.gen::<f64>() * daily_volatility;
        let low_change = rng.gen::<f64>() * daily_volatility;
        
        let open = current_price;
        let high = current_price + (current_price * high_change);
        let low = current_price - (current_price * low_change);
        let close = current_price;
        
        // Volume varies inversely with price stability
        let volume_base = 1_000_000;
        let volume_multiplier = 1.0 + (change_percent.abs() * 5.0);
        let volume = (volume_base as f64 * volume_multiplier) as u64;
        
        prices.push(StockPrice {
            date,
            open,
            high: high.max(open).max(close),
            low: low.min(open).min(close),
            close,
            volume,
        });
    }
    
    prices
}

pub fn server_metrics(hours: usize) -> Vec<ServerMetric> {
    let mut rng = StdRng::seed_from_u64(123);
    let mut metrics = Vec::with_capacity(hours);
    
    let start_time = Utc::now() - Duration::try_hours(hours as i64).unwrap();
    
    for i in 0..hours {
        let timestamp = start_time + Duration::try_hours(i as i64).unwrap();
        
        // CPU usage follows daily patterns with noise
        let hour_of_day = (i % 24) as f64;
        let cpu_base = 30.0 + 40.0 * (hour_of_day / 24.0 * 2.0 * std::f64::consts::PI).sin().max(0.0);
        let cpu_noise = (rng.gen::<f64>() - 0.5) * 20.0;
        let cpu_usage = (cpu_base + cpu_noise).clamp(0.0, 100.0);
        
        // Memory usage slowly increases with periodic cleanup
        let memory_trend = (i as f64 / hours as f64) * 30.0; // Gradual increase
        let memory_cycle = 20.0 * ((i as f64 / 6.0) * 2.0 * std::f64::consts::PI).sin(); // 6-hour cycle
        let memory_noise = (rng.gen::<f64>() - 0.5) * 10.0;
        let memory_usage = (40.0 + memory_trend + memory_cycle + memory_noise).clamp(0.0, 100.0);
        
        // Disk usage slowly increases
        let disk_base = 50.0 + (i as f64 / hours as f64) * 20.0;
        let disk_noise = (rng.gen::<f64>() - 0.5) * 5.0;
        let disk_usage = (disk_base + disk_noise).clamp(0.0, 100.0);
        
        metrics.push(ServerMetric {
            timestamp,
            cpu_usage,
            memory_usage,
            disk_usage,
        });
    }
    
    metrics
}

pub fn sales_data(months: usize) -> Vec<SalesData> {
    let mut rng = StdRng::seed_from_u64(456);
    let mut sales = Vec::with_capacity(months);
    
    let month_names = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun",
        "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"
    ];
    
    let mut base_revenue = 100_000.0;
    
    for i in 0..months {
        let month_index = i % 12;
        let month_name = month_names[month_index].to_string();
        
        // Seasonal patterns
        let seasonal_multiplier = match month_index {
            11 | 0 | 1 => 1.3, // Holiday season boost
            5 | 6 | 7 => 0.8,  // Summer slowdown
            _ => 1.0,
        };
        
        // Year-over-year growth
        let growth_rate = 1.0 + 0.05 * (i / 12) as f64; // 5% annual growth
        
        // Random variation
        let variation = 1.0 + (rng.gen::<f64>() - 0.5) * 0.3; // ±15% variation
        
        let revenue = base_revenue * seasonal_multiplier * growth_rate * variation;
        
        // Units sold correlate with revenue but have different seasonality
        let avg_price = 50.0 + (rng.gen::<f64>() - 0.5) * 20.0; // Price varies $40-$60
        let units_sold = revenue / avg_price;
        
        sales.push(SalesData {
            month: month_name,
            revenue,
            units_sold,
        });
    }
    
    sales
}

pub fn iris_like_data(samples_per_class: usize) -> Vec<IrisLikeData> {
    let mut rng = StdRng::seed_from_u64(789);
    let mut data = Vec::with_capacity(samples_per_class * 3);
    
    // Define characteristics for three species
    let species_params = [
        ("Setosa", (5.0, 3.5, 1.4, 0.2), (0.3, 0.3, 0.1, 0.1)),
        ("Versicolor", (6.0, 2.8, 4.3, 1.3), (0.4, 0.3, 0.4, 0.2)),
        ("Virginica", (6.5, 3.0, 5.5, 2.0), (0.5, 0.3, 0.5, 0.3)),
    ];
    
    for (species_name, means, std_devs) in species_params {
        for _ in 0..samples_per_class {
            // Generate normally distributed features
            let sepal_length = means.0 + std_devs.0 * (rng.gen::<f64>() - 0.5) * 2.0;
            let sepal_width = means.1 + std_devs.1 * (rng.gen::<f64>() - 0.5) * 2.0;
            let petal_length = means.2 + std_devs.2 * (rng.gen::<f64>() - 0.5) * 2.0;
            let petal_width = means.3 + std_devs.3 * (rng.gen::<f64>() - 0.5) * 2.0;
            
            data.push(IrisLikeData {
                sepal_length: sepal_length.max(0.1), // Ensure positive values
                sepal_width: sepal_width.max(0.1),
                petal_length: petal_length.max(0.1),
                petal_width: petal_width.max(0.1),
                species: species_name.to_string(),
            });
        }
    }
    
    data
}

pub fn weather_data(days: usize, latitude: f64) -> Vec<(DateTime<Utc>, f64, f64, f64)> {
    let mut rng = StdRng::seed_from_u64(987);
    let mut weather = Vec::with_capacity(days);
    
    let start_date = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    
    for i in 0..days {
        let date = start_date + Duration::try_days(i as i64).unwrap();
        let day_of_year = i % 365;
        
        // Seasonal temperature variation based on latitude
        let seasonal_temp = 20.0 - latitude.abs() / 3.0; // Warmer near equator
        let seasonal_variation = 15.0 * (2.0 * std::f64::consts::PI * day_of_year as f64 / 365.0).cos();
        let daily_noise = (rng.gen::<f64>() - 0.5) * 10.0;
        let temperature = seasonal_temp + seasonal_variation + daily_noise;
        
        // Humidity inversely correlated with temperature
        let humidity = 70.0 - (temperature - 20.0) * 0.5 + (rng.gen::<f64>() - 0.5) * 20.0;
        let humidity = humidity.clamp(0.0, 100.0);
        
        // Precipitation probability increases with humidity
        let precip_prob = (humidity - 40.0) / 60.0;
        let precipitation = if rng.gen::<f64>() < precip_prob.max(0.0) {
            rng.gen::<f64>() * 50.0 // 0-50mm
        } else {
            0.0
        };
        
        weather.push((date, temperature, humidity, precipitation));
    }
    
    weather
}

pub fn network_traffic_data(hours: usize) -> Vec<(DateTime<Utc>, f64, f64)> {
    let mut rng = StdRng::seed_from_u64(654);
    let mut traffic = Vec::with_capacity(hours);
    
    let start_time = Utc::now() - Duration::try_hours(hours as i64).unwrap();
    
    for i in 0..hours {
        let timestamp = start_time + Duration::try_hours(i as i64).unwrap();
        let hour_of_day = (i % 24) as f64;
        let day_of_week = ((i / 24) % 7) as f64;
        
        // Business hours pattern
        let business_hours_multiplier = if hour_of_day >= 9.0 && hour_of_day <= 17.0 {
            1.5
        } else {
            0.3
        };
        
        // Weekend reduction
        let weekend_multiplier = if day_of_week >= 5.0 { 0.4 } else { 1.0 };
        
        // Base traffic with patterns
        let base_incoming = 100.0 * business_hours_multiplier * weekend_multiplier;
        let base_outgoing = 80.0 * business_hours_multiplier * weekend_multiplier;
        
        // Add noise and spikes
        let spike_probability = 0.05; // 5% chance of traffic spike
        let spike_multiplier = if rng.gen::<f64>() < spike_probability { 3.0 } else { 1.0 };
        
        let noise_in = (rng.gen::<f64>() - 0.5) * 20.0;
        let noise_out = (rng.gen::<f64>() - 0.5) * 15.0;
        
        let incoming = (base_incoming + noise_in) * spike_multiplier;
        let outgoing = (base_outgoing + noise_out) * spike_multiplier;
        
        traffic.push((timestamp, incoming.max(0.0), outgoing.max(0.0)));
    }
    
    traffic
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stock_prices() {
        let prices = stock_prices(30, 100.0, 0.02);
        assert_eq!(prices.len(), 30);
        
        for price in &prices {
            assert!(price.high >= price.low);
            assert!(price.high >= price.open);
            assert!(price.high >= price.close);
            assert!(price.low <= price.open);
            assert!(price.low <= price.close);
            assert!(price.volume > 0);
        }
    }

    #[test]
    fn test_server_metrics() {
        let metrics = server_metrics(24);
        assert_eq!(metrics.len(), 24);
        
        for metric in &metrics {
            assert!(metric.cpu_usage >= 0.0 && metric.cpu_usage <= 100.0);
            assert!(metric.memory_usage >= 0.0 && metric.memory_usage <= 100.0);
            assert!(metric.disk_usage >= 0.0 && metric.disk_usage <= 100.0);
        }
    }

    #[test]
    fn test_sales_data() {
        let sales = sales_data(12);
        assert_eq!(sales.len(), 12);
        
        for sale in &sales {
            assert!(sale.revenue > 0.0);
            assert!(sale.units_sold > 0.0);
            assert!(!sale.month.is_empty());
        }
    }

    #[test]
    fn test_iris_like_data() {
        let data = iris_like_data(10);
        assert_eq!(data.len(), 30); // 10 samples × 3 species
        
        let species_counts: std::collections::HashMap<String, usize> = 
            data.iter().fold(std::collections::HashMap::new(), |mut acc, item| {
                *acc.entry(item.species.clone()).or_insert(0) += 1;
                acc
            });
        
        assert_eq!(species_counts.len(), 3);
        for count in species_counts.values() {
            assert_eq!(*count, 10);
        }
    }

    #[test]
    fn test_weather_data() {
        let weather = weather_data(7, 40.0); // 7 days at 40° latitude
        assert_eq!(weather.len(), 7);
        
        for (_, temp, humidity, precip) in &weather {
            assert!(humidity >= &0.0 && humidity <= &100.0);
            assert!(precip >= &0.0);
        }
    }
}