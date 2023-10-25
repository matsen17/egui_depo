use std::{borrow::Cow, iter::FromIterator};
use eframe::egui::{self, Button, Color32, CtxRef, FontDefinitions, FontFamily, Hyperlink, Label, Layout, Separator, TopBottomPanel, Window, epaint::text};

pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

use crate::{book::{Book, Magazine, DepositoryItem}, reader::{self, Reader}, AddCommand};

pub struct BookDepository {
    pub books: Vec<Book>,
    pub magazines: Vec<Magazine>,
    pub readers: Vec<Reader>,
    
    pub selected_book_for_borrow: Option<Book>,
    pub selected_magazine_for_borrow: Option<Magazine>,
    
    // inputs
    pub name_input: String,
    pub year_input: String,
    pub superhero_input: String,
    pub isbn_input: String,

    //commands
    pub add_command: AddCommand
}

impl BookDepository {

    pub fn clear_inputs(&mut self)
    {
        self.name_input.clear();
        self.year_input.clear();
        self.superhero_input.clear();
        self.isbn_input.clear();
    }

    pub fn configure_fonts(&self, ctx: &CtxRef) {
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "MesloLGS".to_string(),
            Cow::Borrowed(include_bytes!("../../MesloLGS_NF_Regular.ttf")),
        );
        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Heading,
            (FontFamily::Proportional, 35.),
        );
        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Body,
            (FontFamily::Proportional, 20.),
        );
        font_def
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "MesloLGS".to_string());
        ctx.set_fonts(font_def);
    }

    pub fn render_depository_items(&mut self, ui: &mut eframe::egui::Ui, ctx: &CtxRef) {
        for book in &self.books {
            ui.add_space(PADDING);
            // render title
            let title = format!("▶ {}", book.title);
            ui.colored_label(WHITE, title);
            // render desc
            ui.add_space(PADDING);
            let desc = Label::new(book.year).text_style(eframe::egui::TextStyle::Button);
            ui.add(desc);

            // render hyperlinks
            ui.style_mut().visuals.hyperlink_color = CYAN;
            ui.add_space(PADDING);
            ui.with_layout(Layout::right_to_left(), |ui| {
                let borrow_button = ui.add(Button::new("Borrow").text_style(egui::TextStyle::Body));
                let remove_btn = ui.add(Button::new("❌").text_style(egui::TextStyle::Body));

                if borrow_button.clicked() {
                    self.selected_book_for_borrow = Some(book.clone()); 
                }
            
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
        for magazine in &self.magazines {
            ui.add_space(PADDING);
            // render title
            let title = format!("▶ {}", magazine.title);
            ui.colored_label(WHITE, title);
            // render desc
            ui.add_space(PADDING);
            let desc = Label::new(magazine.year).text_style(eframe::egui::TextStyle::Button);
            ui.add(desc);

            // render hyperlinks
            ui.style_mut().visuals.hyperlink_color = CYAN;
            ui.add_space(PADDING);
            ui.with_layout(Layout::right_to_left(), |ui| {
                let borrow_button = ui.add(Button::new("Borrow").text_style(egui::TextStyle::Body));
                let remove_btn = ui.add(Button::new("❌").text_style(egui::TextStyle::Body));

                if borrow_button.clicked() {
                    self.selected_magazine_for_borrow = Some(magazine.clone()); 
                }
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }

    pub(crate) fn render_top_panel(&mut self, ctx: &CtxRef) {
        // define a TopBottomPanel widget
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(10.);
            egui::menu::bar(ui, |ui| {
                // logo
                ui.with_layout(Layout::left_to_right(), |ui| {
                    ui.add(Label::new("📓").text_style(egui::TextStyle::Heading));
                });
                // controls
                ui.with_layout(Layout::right_to_left(), |ui| {
                    let add_book_btn = ui.add(Button::new("Add Book").text_style(egui::TextStyle::Body));
                    let add_magazine_btn = ui.add(Button::new("Add Magazine").text_style(egui::TextStyle::Body));
                    let add_user_btn = ui.add(Button::new("Add User").text_style(egui::TextStyle::Body));

                    if add_user_btn.clicked() {
                        self.add_command = AddCommand::AddUser
                    }
                    if add_book_btn.clicked() {
                        self.add_command = AddCommand::AddBook
                    }
                    if add_magazine_btn.clicked() {
                        self.add_command = AddCommand::AddMagazine
                    }
                });
            });
            ui.add_space(10.);
        });
    }


}