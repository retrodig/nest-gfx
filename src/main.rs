use eframe::egui;

const GRID_SIZE: usize = 8;

#[derive(Default)]
struct ChrEditor {
    pixels: [[u8; GRID_SIZE]; GRID_SIZE],
    selected_color: u8,
}

impl eframe::App for ChrEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Nest Graphics");

            let (response, painter) =
                ui.allocate_painter(ui.available_size_before_wrap(), egui::Sense::click_and_drag());

            let rect = response.rect;
            let grid_w = rect.width() / GRID_SIZE as f32;
            let grid_h = rect.height() / GRID_SIZE as f32;

            // 描画
            for y in 0..GRID_SIZE {
                for x in 0..GRID_SIZE {
                    let color = if self.pixels[y][x] == 1 {
                        egui::Color32::BLACK
                    } else {
                        egui::Color32::WHITE
                    };
                    let r = egui::Rect::from_min_size(
                        rect.left_top() + egui::vec2(x as f32 * grid_w, y as f32 * grid_h),
                        egui::vec2(grid_w, grid_h),
                    );
                    painter.rect_filled(r, 0.0, color);
                }
            }

            // マウスクリックでドット編集
            if let Some(pos) = response.interact_pointer_pos() {
                let x = ((pos.x - rect.left()) / grid_w) as usize;
                let y = ((pos.y - rect.top()) / grid_h) as usize;
                if x < GRID_SIZE && y < GRID_SIZE {
                    self.pixels[y][x] ^= 1;
                }
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Nest Graphics",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(ChrEditor::default()))),
    )
}
