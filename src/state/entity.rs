use gpui::{prelude::*, App, Entity, Global};

use crate::state::{Calculator, Event};

#[derive(Clone)]
pub struct StateEntity {
    pub inner: Entity<Calculator>,
}

impl StateEntity {
    pub fn build(app: &mut App) {
        let inner = app.new(|_cx| Calculator::new());

        app.subscribe(&inner, |entity, event: &Event, cx| {
            entity.update(cx, |calculator, cx| {
                event.handler(calculator);
                cx.notify();
            });
        })
        .detach();

        app.set_global::<StateEntity>(Self { inner });
    }

    pub fn emit(event: Event, app: &mut App) {
        app.update_global::<Self, _>(|entity, app| {
            entity.inner.update(app, |_state, cx| {
                cx.emit(event);
            })
        });
    }
}

impl Global for StateEntity {}
