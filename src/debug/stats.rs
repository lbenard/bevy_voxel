use std::ops::{Add, Div};

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

#[derive(Resource, Default)]
pub struct Average<T>
where
    T: Add<T> + Div<u32> + Default + Copy + Send + Sync + 'static,
{
    total: T,
    samples: u32,
}

impl<T> Average<T>
where
    T: Add<Output = T> + Div<u32> + Default + Copy + Send + Sync + 'static,
{
    pub fn add(&mut self, item: T) {
        self.total = self.total + item;
        self.samples += 1;
    }

    pub fn egui_debug(average: Res<Average<T>>, mut contexts: EguiContexts)
    where
        <T as Div<u32>>::Output: derive_more::Debug,
    {
        egui::Window::new(std::any::type_name::<T>()).show(contexts.ctx_mut(), |ui| {
            ui.label(format!("{:?}", average.total / average.samples.max(1)))
        });
    }
}
