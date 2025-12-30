pub static API_KEY: &str = {
    #[cfg(test)]
    {
        "test_api_key"
    }
    #[cfg(not(test))]
    {
        env!("OPENWEATHER_API_KEY")
    }
};
