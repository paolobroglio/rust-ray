use leonardo_engine::App;

fn main() -> std::io::Result<()> {
    let aspect_ratio = 16.0 / 9.0_f32;
    let image_width = 384;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let app = App::new(aspect_ratio, image_width, samples_per_pixel, max_depth);
    app.run()
}