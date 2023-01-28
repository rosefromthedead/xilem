use vello::peniko::{Brush, Color, Gradient};
use xilem::{button, checkbox, stack, App, AppLauncher, Axis, View, ViewExt};

fn app_logic(data: &mut (i32, f32, Axis)) -> impl View<(i32, f32, Axis)> {
    data.1 += 0.01;
    data.1 = data.1 % 1.0;
    let x = data.1 * 2.0 - 0.5;
    let border_dark = Color::rgb8(0x08, 0x58, 0x60);
    let border_light = Color::rgb8(0x08, 0xd8, 0xea);
    stack(
        (
            checkbox(|_data, state| println!("{state}")),
            button(format!("{}+", data.0), |data: &mut (i32, f32, Axis)| {
                data.0 += 1
            }),
            button(format!("{}-", data.0), |data: &mut (i32, f32, Axis)| {
                data.0 -= 1
            }),
            button(
                format!("flip!"),
                |(_, _, ref mut axis): &mut (i32, f32, Axis)| {
                    *axis = match axis {
                        Axis::Horizontal => Axis::Vertical,
                        Axis::Vertical => Axis::Horizontal,
                    }
                },
            ),
        ),
        4.0,
        data.2,
    )
    .padding(4.0)
    .background(Color::rgb8(0x20, 0x20, 0x28))
    .border(
        4.0,
        1.0,
        Brush::Gradient(
            Gradient::new_linear((0.0, -20.0), (0.0, 160.0)).with_stops([
                (0.0, border_dark),
                (x, border_dark),
                (x + 0.1, border_light),
                (x + 0.2, border_dark),
                (1.0, border_dark),
            ]),
        ),
    )
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
    let app = App::new((0, 0.0, Axis::Vertical), app_logic);
    AppLauncher::new(app).run()
}
