use crate::day_report::*;
use orbtk::prelude::*;
use std::path::Path;
use std::fmt::Display;


// Rendere widget
// Workaround della string che non va a capo: crea un TextBlock per ogni riga

enum ActionWeek {
    Change(i64)
}

#[derive(AsAny)]
pub struct WeekState {
    action: Option<ActionWeek>
}
impl Default for WeekState {
    fn default() -> Self {
        WeekState { action: None }
    }
}

impl WeekState {
    fn action(&mut self, act: impl Into<Option<ActionWeek>>) {
        self.action = act.into();
    }
}

impl State for WeekState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(act)=&self.action {
            match act {
                ActionWeek::Change(a) => {
                    *week_view(ctx.widget()).n_week_mut() += a;
                    let week = WeekReport::new(Path::new(&rec_folder()),*week_view(ctx.widget()).n_week_mut()).unwrap_or(WeekReport::default());
                    let stack =ctx.child("Box").entity();
                    ctx.clear_children_of(stack);
                    for i in 0..7 {
                        if let Some(day)=&week.day_reports[i] {
                            let i_st = format!("{}",i);
                            ctx.append_child_to(Stack::new().id(i_st.as_str()).orientation(Orientation::Vertical), stack);
                            let day_stack =ctx.child(i_st.as_str()).entity();
                            block_builder(day, ctx, day_stack);
                        } else {
                            block_builder(&String::from("No data available"), ctx, stack);
                        }
                    }
                    ctx.append_child_to(Stack::new().id("tot").orientation(Orientation::Vertical), stack);
                    let day_stack =ctx.child("tot").entity();
                    block_builder(&week.tot_report, ctx, day_stack);
                }
            }
        }
        self.action = None;
    }
}



fn state<'a>(id: Entity, states: &'a mut StatesContext) -> &'a mut WeekState {
    states.get_mut(id)
}

fn block_builder<T: Display>(inp: &T, ctx: &mut Context, stack: Entity) {
    let splits = String::from(format!("{}",inp));
    let splits = splits.split("\n");
    splits.for_each(|b| ctx.append_child_to(TextBlock::new().text(b)
                                                .font_size(14)
                                                .v_align("center")
                                                .h_align("left"), stack))

}

// Non supporta Option
widget!(WeekView<WeekState> {
    n_week: i64
    }
);

impl Template for WeekView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("WeekView")
            .n_week(1)
            .child(Stack::new().orientation(Orientation::Vertical)
                    .spacing(30)
                    .child(Stack::new().orientation(Orientation::Horizontal)
                                .child(Button::new()
                                        .text("Previous")
                                        .margin((0, 8, 0, 0))
                                        .on_click(move |states, _| {
                                            state(id, states).action(ActionWeek::Change(-1));
                                            true
                                        })
                                        .build(ctx)
                                )
                                .child(Button::new()
                                        .text("Next")
                                        .margin((0, 8, 0, 0))
                                        .on_click(move |states, _| {
                                            state(id, states).action(ActionWeek::Change(1));
                                            true
                                        })
                                        .build(ctx)
                                )
                            .spacing(1200)
                            .build(ctx)
                    )
                    .child(Stack::new()
                            .orientation(Orientation::Horizontal)
                            .id("Box")
                            .v_align("center")
                            .h_align("center")
                            .spacing(20)
                            .build(ctx)
                    )
                    .build(ctx)
            )
    }
}
