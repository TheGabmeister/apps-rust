use eframe::egui;

use crate::engine::{CalcEngine, Operator};

pub struct CalculatorApp {
    engine: CalcEngine,
}

impl CalculatorApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            engine: CalcEngine::new(),
        }
    }

    fn handle_keyboard_input(&mut self, ctx: &egui::Context) {
        ctx.input_mut(|input| {
            // Digits 0-9 (top row and numpad)
            for (key, ch) in [
                (egui::Key::Num0, '0'),
                (egui::Key::Num1, '1'),
                (egui::Key::Num2, '2'),
                (egui::Key::Num3, '3'),
                (egui::Key::Num4, '4'),
                (egui::Key::Num5, '5'),
                (egui::Key::Num6, '6'),
                (egui::Key::Num7, '7'),
                (egui::Key::Num8, '8'),
                (egui::Key::Num9, '9'),
            ] {
                if input.consume_key(egui::Modifiers::NONE, key) {
                    self.engine.input_digit(ch);
                }
            }

            // Decimal point
            if input.consume_key(egui::Modifiers::NONE, egui::Key::Period) {
                self.engine.input_decimal();
            }

            // Operators
            if input.consume_key(egui::Modifiers::NONE, egui::Key::Plus)
                || input.consume_key(egui::Modifiers::SHIFT, egui::Key::Plus)
            {
                self.engine.input_operator(Operator::Add);
            }
            if input.consume_key(egui::Modifiers::NONE, egui::Key::Minus) {
                self.engine.input_operator(Operator::Subtract);
            }
            if input.consume_key(egui::Modifiers::NONE, egui::Key::Slash) {
                self.engine.input_operator(Operator::Divide);
            }
            // * via Shift+8 on US keyboard
            if input.consume_key(egui::Modifiers::SHIFT, egui::Key::Num8) {
                self.engine.input_operator(Operator::Multiply);
            }

            // Also handle '*' via text events for non-US keyboards
            for event in &input.events {
                if let egui::Event::Text(t) = event {
                    if t == "*" {
                        self.engine.input_operator(Operator::Multiply);
                    }
                }
            }

            // Enter / Equals
            if input.consume_key(egui::Modifiers::NONE, egui::Key::Enter)
                || input.consume_key(egui::Modifiers::NONE, egui::Key::Equals)
                || input.consume_key(egui::Modifiers::SHIFT, egui::Key::Equals)
            {
                self.engine.input_equals();
            }

            // Escape -> clear
            if input.consume_key(egui::Modifiers::NONE, egui::Key::Escape) {
                self.engine.clear();
            }

            // Backspace
            if input.consume_key(egui::Modifiers::NONE, egui::Key::Backspace) {
                self.engine.backspace();
            }

            // Delete -> clear entry
            if input.consume_key(egui::Modifiers::NONE, egui::Key::Delete) {
                self.engine.clear_entry();
            }
        });
    }

    fn render_display(&self, ui: &mut egui::Ui) {
        let available_width = ui.available_width();

        // Expression line (small, gray, right-aligned)
        let expr = self.engine.expression_display();
        ui.allocate_ui_with_layout(
            egui::vec2(available_width, 24.0),
            egui::Layout::right_to_left(egui::Align::Center),
            |ui| {
                if !expr.is_empty() {
                    ui.label(
                        egui::RichText::new(&expr)
                            .size(16.0)
                            .color(egui::Color32::GRAY),
                    );
                }
            },
        );

        // Main display (large, right-aligned, monospace)
        ui.allocate_ui_with_layout(
            egui::vec2(available_width, 48.0),
            egui::Layout::right_to_left(egui::Align::Center),
            |ui| {
                let color = if self.engine.is_error() {
                    egui::Color32::from_rgb(255, 80, 80)
                } else {
                    ui.visuals().text_color()
                };
                ui.label(
                    egui::RichText::new(self.engine.display())
                        .size(32.0)
                        .color(color)
                        .monospace(),
                );
            },
        );

        ui.add_space(4.0);
        ui.separator();
        ui.add_space(4.0);
    }

    fn render_buttons(&mut self, ui: &mut egui::Ui) {
        let spacing = 4.0;
        let cols = 4.0;
        let available = ui.available_width() - spacing * (cols - 1.0);
        let btn_w = available / cols;
        let btn_h = 48.0;
        let btn_size = egui::vec2(btn_w, btn_h);

        let op_fill = egui::Color32::from_rgb(60, 120, 200);
        let eq_fill = egui::Color32::from_rgb(40, 160, 80);
        let func_fill = egui::Color32::from_rgb(80, 80, 90);

        egui::Grid::new("calc_buttons")
            .spacing(egui::vec2(spacing, spacing))
            .show(ui, |ui| {
                // Row 0: C  %  ⌫  ÷
                if ui
                    .add_sized(btn_size, egui::Button::new("C").fill(func_fill))
                    .clicked()
                {
                    self.engine.clear();
                }
                if ui
                    .add_sized(btn_size, egui::Button::new("%").fill(func_fill))
                    .clicked()
                {
                    self.engine.percentage();
                }
                if ui
                    .add_sized(btn_size, egui::Button::new("⌫").fill(func_fill))
                    .clicked()
                {
                    self.engine.backspace();
                }
                if ui
                    .add_sized(btn_size, egui::Button::new("÷").fill(op_fill))
                    .clicked()
                {
                    self.engine.input_operator(Operator::Divide);
                }
                ui.end_row();

                // Row 1: 7 8 9 ×
                for digit in ['7', '8', '9'] {
                    if ui
                        .add_sized(btn_size, egui::Button::new(digit.to_string()))
                        .clicked()
                    {
                        self.engine.input_digit(digit);
                    }
                }
                if ui
                    .add_sized(btn_size, egui::Button::new("×").fill(op_fill))
                    .clicked()
                {
                    self.engine.input_operator(Operator::Multiply);
                }
                ui.end_row();

                // Row 2: 4 5 6 −
                for digit in ['4', '5', '6'] {
                    if ui
                        .add_sized(btn_size, egui::Button::new(digit.to_string()))
                        .clicked()
                    {
                        self.engine.input_digit(digit);
                    }
                }
                if ui
                    .add_sized(btn_size, egui::Button::new("−").fill(op_fill))
                    .clicked()
                {
                    self.engine.input_operator(Operator::Subtract);
                }
                ui.end_row();

                // Row 3: 1 2 3 +
                for digit in ['1', '2', '3'] {
                    if ui
                        .add_sized(btn_size, egui::Button::new(digit.to_string()))
                        .clicked()
                    {
                        self.engine.input_digit(digit);
                    }
                }
                if ui
                    .add_sized(btn_size, egui::Button::new("+").fill(op_fill))
                    .clicked()
                {
                    self.engine.input_operator(Operator::Add);
                }
                ui.end_row();

                // Row 4: ±  0  .  =
                if ui
                    .add_sized(btn_size, egui::Button::new("±").fill(func_fill))
                    .clicked()
                {
                    self.engine.toggle_sign();
                }
                if ui
                    .add_sized(btn_size, egui::Button::new("0"))
                    .clicked()
                {
                    self.engine.input_digit('0');
                }
                if ui
                    .add_sized(btn_size, egui::Button::new("."))
                    .clicked()
                {
                    self.engine.input_decimal();
                }
                if ui
                    .add_sized(btn_size, egui::Button::new("=").fill(eq_fill))
                    .clicked()
                {
                    self.engine.input_equals();
                }
                ui.end_row();
            });
    }
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_keyboard_input(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().text_styles.insert(
                egui::TextStyle::Button,
                egui::FontId::proportional(20.0),
            );

            self.render_display(ui);
            self.render_buttons(ui);
        });
    }
}
