fn round(value: f32) -> f32 {
    (value * 10.0).round() / 10.0
}

pub fn get_max(endurance: f32) -> f32 {
    round((-((endurance - 100.0) / 10.0).powf(2.0) + 100.0) * 10.0)
}


#[cfg(test)]
mod tests {
    use super::get_max;

    #[test]
    fn max_health() {
        let max_health = get_max(5.0);
        dbg!(max_health);

        let max_health = get_max(10.0);
        dbg!(max_health);

        let max_health = get_max(20.0);
        dbg!(max_health);

        let max_health = get_max(40.0);
        dbg!(max_health);

        let max_health = get_max(50.0);
        dbg!(max_health);

        let max_health = get_max(80.0);
        dbg!(max_health);


        let max_health = get_max(99.0);
        dbg!(max_health);

        assert!(max_health == 999.9)
    }
}
