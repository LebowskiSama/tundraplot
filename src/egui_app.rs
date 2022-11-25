use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

use egui::{CtxRef, plot::{Plot, Line, Values, Value}};
use eframe::epi;

pub struct App {
    pub idx: Arc<Mutex<f64>>,
    pub buffer_x: Arc<Mutex<VecDeque<Value>>>
}

impl epi::App for App {

    fn update(&mut self, ctx: &egui::CtxRef, _: &eframe::epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let plot = Plot::new("Acc");
            
            plot.show(ui, |plot_ui| {
                plot_ui.line(Line::new(
                    self.get_values()
                ))
            });
        });
        ctx.request_repaint();
    }

    fn name(&self) -> &str {
        "Tundraplot"
    }

}

impl App {
    pub fn new() -> Self {
        return App { idx: Arc::new(Mutex::new(0.0)), buffer_x: Arc::new(Mutex::new(VecDeque::new())) }
    }

    pub fn get_values(&mut self) -> Values {
        // let df = df.lock().unwrap();

        // return Values::from_values_iter (
        //     df.idx.iter().zip(df.x.iter())
        //         .map(|(idx, x)| Value { x: *idx, y: *x })
        //         .collect::<Vec<Value>>()
        //         .iter()
        //         .copied()
        // )
        let buffer_x = self.buffer_x.lock().unwrap();
        
        return Values::from_values_iter(buffer_x.clone().into_iter())

    }
}