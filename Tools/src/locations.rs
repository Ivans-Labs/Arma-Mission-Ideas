use rand::Rng;

pub fn generate_location() -> String {
    let locations = vec!["Tanoa", "Altis", "Stratis"];
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..locations.len());
    locations[index].to_string()
}
