use vello::peniko::{Brush, Color, Gradient};
use xilem::{button, checkbox, stack, App, AppLauncher, Axis, View, ViewExt};

struct Data {
    checked: bool,
    counter: i32,
    axis: Axis,
    shine: f32,
}

fn app_logic(data: &mut Data) -> impl View<Data> {
    data.shine += 0.01;
    data.shine = data.shine % 1.0;
    let x = data.shine * 2.0 - 0.5;
    let border_dark = Color::rgb8(0x08, 0x58, 0x60);
    let border_light = Color::rgb8(0x08, 0xd8, 0xea);
    stack(
        (
            checkbox(data.checked, |data: &mut Data, checked| {
                data.checked = checked
            }),
            button(format!("{}+", data.counter), |data: &mut Data| {
                data.counter += 1
            }),
            button(format!("{}-", data.counter), |data: &mut Data| {
                data.counter -= 1
            }),
            button(format!("flip!"), |data: &mut Data| {
                data.axis = match data.axis {
                    Axis::Horizontal => Axis::Vertical,
                    Axis::Vertical => Axis::Horizontal,
                }
            }),
        ),
        4.0,
        data.axis,
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
    let data = Data {
        checked: false,
        counter: 0,
        axis: Axis::Vertical,
        shine: 0.0,
    };
    let app = App::new(data, app_logic);
    AppLauncher::new(app).run()
}
