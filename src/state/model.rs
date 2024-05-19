use gpui::*;

use crate::state::*;

#[derive(Clone)]
pub struct StateModel {
    pub inner: Model<Calculator>,
}

impl StateModel {
    pub fn build(cx: &mut AppContext) {
        let model = cx.new_model(|_cx| Calculator::new());

        cx.subscribe(&model, |model, event: &Event, cx| {
            cx.update_model(&model, |calculator, cx| {
                event.handler(calculator);
                cx.notify();
            });
        })
        .detach();

        cx.set_global::<StateModel>(Self { inner: model });
    }

    pub fn emit(event: Event, cx: &mut AppContext) {
        cx.update_global::<Self, _>(|model, cx| {
            cx.update_model(&model.inner, |_state, cx| {
                cx.emit(event);
            })
        });
    }
}

impl Global for StateModel {}
