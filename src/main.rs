use vello::peniko::Color;
use xilem::{button, checkbox, stack, App, AppLauncher, Axis, View, ViewExt};

fn app_logic(data: &mut (i32, Axis)) -> impl View<(i32, Axis)> {
    stack(
        (
            checkbox(|_data, state| println!("{state}")),
            button(format!("{}+", data.0), |data: &mut (i32, Axis)| data.0 += 1),
            button(format!("{}-", data.0), |data: &mut (i32, Axis)| data.0 -= 1),
            button(format!("flip!"), |(_, ref mut axis): &mut (i32, Axis)| {
                *axis = match axis {
                    Axis::Horizontal => Axis::Vertical,
                    Axis::Vertical => Axis::Horizontal,
                }
            }),
        ),
        4.0,
        data.1,
    )
    .background(Color::rgb8(0x20, 0x20, 0x28))
    .padding(4.0)
}

fn main() {
    /*
    let app = Application::new().unwrap();
    let mut window_builder = glazier::WindowBuilder::new(app.clone());
    window_builder.resizable(false);
    window_builder.set_size((WIDTH as f64 / 2., HEIGHT as f64 / 2.).into());
    window_builder.set_handler(Box::new(xilem::WindowState::new()));
    let window_handle = window_builder.build().unwrap();
    window_handle.show();
    app.run(None);
    */
    let app = App::new((0, Axis::Vertical), app_logic);
    AppLauncher::new(app).run()
}
