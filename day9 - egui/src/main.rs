use nom::{combinator::all_consuming, Finish};
use parse::{Direction, Instruction};

mod parse;

use eframe::egui;

fn main() {
    let instructions = include_str!("input.txt")
        .lines()
        .map(|l| all_consuming(Instruction::parse)(l).finish().unwrap().1)
        .collect::<Vec<_>>();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "AoC 2022 — Day 9",
        options,
        Box::new(|_cc| Box::new(MyApp { instructions })),
    );
}

struct MyApp {
    instructions: Vec<Instruction>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Instructions:");
            for ins in &self.instructions {
                let arrow = match ins.dir {
                    Direction::Up => "⬆",
                    Direction::Down => "⬇",
                    Direction::Right => "➡",
                    Direction::Left => "⬅",
                };
                ui.label(arrow.repeat(ins.dist as _));
            }
        });
    }
}
