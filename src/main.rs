//! Show a custom window frame instead of the default OS window chrome decorations.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
//use std::error::Error; 
use std::io::Error;
use egui::{menu, Button};
//use syntax_highlighting::*;
//use syntect::*;
use serde;

use egui_demo_lib::syntax_highlighting;

fn main() -> Result<(), Error> {
    let options = eframe::NativeOptions {
        // Hide the OS-specific "chrome" around the window:
        decorated: false,
        // To have rounded corners we need transparency:
        transparent: true,
        min_window_size: Some(egui::vec2(320.0, 100.0)),
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Custom window frame", // unused title
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
    Ok(())
}

//#[derive(Default)]
//#[cfg_attr(feature = "serde", serde(default))]

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
struct MyApp {
  language: String,
  code: String,
}

impl Default for MyApp {
  fn default() -> Self {
    Self {
      language: "rs".into(),
      code: "// A very simple example\n".into(),
    }
  }
}



impl eframe::App for MyApp {
  fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
    egui::Rgba::TRANSPARENT // Make sure we don't paint anything behind the rounded corners
  }

  fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    let Self { language, code } = self;

    custom_window_frame(ctx, frame, "egui Text Editor", |ui| {

      menu::bar(ui, |ui| {
        ui.menu_button("File", |ui| {
          if ui.button("New").clicked() {
            ui.close_menu()
          }
          if ui.button("Save").clicked() {
            ui.close_menu()
          }
          if ui.button("Open").clicked() {
            ui.close_menu()
          }
          if ui.button("Exit").clicked() {
            ui.close_menu()
          }
        });

        ui.menu_button("View", |ui| {
          if ui.button("folders").clicked() {
            ui.close_menu()
          }
          if ui.button("contents").clicked() {
            ui.close_menu()
          }
        });

        ui.menu_button("Help", |ui| {
          if ui.button("About").clicked() {
            ui.close_menu()
          }
        });

        ui.label("Theme:");
        egui::widgets::global_dark_light_mode_buttons(ui);
      });

      ui.label("This is just the contents of the window");
      //ui.horizontal(|ui| {
        //ui.label("egui theme:");
        //egui::widgets::global_dark_light_mode_buttons(ui);
      //});


      
      let mut theme = syntax_highlighting::CodeTheme::from_memory(ui.ctx());
        ui.collapsing("Theme", |ui| {
            ui.group(|ui| {
                theme.ui(ui);
                theme.clone().store_in_memory(ui.ctx());
            });
        });

      
      let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
        let mut layout_job =
            crate::syntax_highlighting::highlight(ui.ctx(), &theme, string, language);
        layout_job.wrap.max_width = wrap_width;
        ui.fonts().layout_job(layout_job)
      };
      
      egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add(
            egui::TextEdit::multiline(code)
                .font(egui::TextStyle::Monospace) // for cursor height
                .code_editor()
                .desired_rows(10)
                .lock_focus(true)
                .desired_width(f32::INFINITY)
                .layouter(&mut layouter),
        );
      });
      

    });
  }
}

fn custom_window_frame(
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    title: &str,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    use egui::*;
    let text_color = ctx.style().visuals.text_color();

    // Height of the title bar
    let height = 28.0;

    CentralPanel::default()
      .frame(Frame::none())
      .show(ctx, |ui| {
        let rect = ui.max_rect();
        let painter = ui.painter();

        // Paint the frame:
        painter.rect(
          rect.shrink(1.0),
          10.0,
          ctx.style().visuals.window_fill(),
          Stroke::new(1.0, text_color),
        );

        // Paint the title:
        painter.text(
          rect.center_top() + vec2(0.0, height / 2.0),
          Align2::CENTER_CENTER,
          title,
          FontId::proportional(height * 0.8),
          text_color,
        );

        // Paint the line under the title:
        painter.line_segment(
          [
            rect.left_top() + vec2(2.0, height),
            rect.right_top() + vec2(-2.0, height),
          ],
          Stroke::new(1.0, text_color),
        );

        // Interact with the title bar (drag to move window):
        let title_bar_rect = {
          let mut rect = rect;
          rect.max.y = rect.min.y + height;
          rect
        };
        let title_bar_response =
          ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());
        if title_bar_response.is_pointer_button_down_on() {
          frame.drag_window();
        }

        // Add the close button:
        let close_response = ui.put(
          Rect::from_min_size(rect.left_top(), Vec2::splat(height)),
          Button::new(RichText::new("❌").size(height - 4.0)).frame(false),
        );
        if close_response.clicked() {
          frame.close();
        }

        // Add the contents:
        let content_rect = {
          let mut rect = rect;
          rect.min.y = title_bar_rect.max.y;
          rect
        }
        .shrink(4.0);
        let mut content_ui = ui.child_ui(content_rect, *ui.layout());
        add_contents(&mut content_ui);
    });
}