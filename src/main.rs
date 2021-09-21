mod recs;
mod engine;
mod day_report;
mod tag_search;
mod search;
mod week;
mod styles;

use search::*;
use week::*;
use orbtk::prelude::*;


fn main() {

    Application::new()
    .window(move |ctx| {
        Window::new()
            .title("Record")
            .position((10.0, 50.0))
            .resizeable(true)
            .size(1500.0, 800.0)
            .background(Color::rgb(0,0,0))
            .child(TabWidget::new()
                .tab("Week Report",WeekView::new().build(ctx))
                .tab("Tag Analysis",SearchView::new().build(ctx))
                .build(ctx))
            .build(ctx)
    })
    .run();
}
