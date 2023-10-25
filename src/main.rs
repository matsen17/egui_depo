use std::{io::empty, rc::Rc, sync::Arc};

use eframe::{
    egui::{
        CentralPanel, CtxRef, Hyperlink, Label, ScrollArea, Separator, TextStyle, TopBottomPanel, Window,
        Ui, Vec2, self, TextBuffer,
    },
    epi::App,
    run_native, NativeOptions,
};

mod book_depository;
mod persistence;
mod book;
mod reader;

use reader::Reader;
use book::{Book, DepositoryItem};
use book::Magazine;


use book_depository::{BookDepository, PADDING};

pub enum AddCommand {
    None,
    AddUser,
    AddBook,
    AddMagazine
}


impl App for BookDepository {
    fn setup(
        &mut self,
        ctx: &eframe::egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {

        self.render_top_panel(ctx);

        match self.add_command {
            AddCommand::None => {
            }
            AddCommand::AddUser => {
                Window::new("Add Reader")
                    .show(ctx, |ui| {
                        ui.vertical(|ui| {
                            ui.label("Reader's Name:");
                            ui.text_edit_singleline(&mut self.name_input);
                        });
                        
                        if ui.button("Add reader").clicked() {
                            self.readers.push(Reader::create(&self.name_input));
                            self.add_command = AddCommand::None;
                            self.clear_inputs();
                        }
                    });
            }
            AddCommand::AddBook => {
                Window::new("Add Book")
                    .show(ctx, |ui| {
                    
                        ui.vertical(|ui| {
                            ui.label("Book Name:");
                            ui.text_edit_singleline(&mut self.name_input);
                            ui.label("Book Year:");
                            ui.text_edit_singleline(&mut self.year_input);
                            ui.label("ISBN code:");
                            ui.text_edit_singleline(&mut self.isbn_input);
                        });

                        if ui.button("Add book").clicked() {

                            let book_year_int = match self.year_input.trim().parse::<i32>() {
                                Ok(num) => num,
                                Err(_) => {
                                    eprintln!("Invalid input!");
                                    return;
                                }
                            };

                            self.books.push(Book::create(self.isbn_input.clone(), self.name_input.clone(), book_year_int));
                            self.add_command = AddCommand::None;
                            self.clear_inputs();
                        }
                    });
            }
            AddCommand::AddMagazine => {
                Window::new("Add Magazine")
                    .show(ctx, |ui| {
                        ui.vertical(|ui| {
                            ui.label("Magazine Name:");
                            ui.text_edit_singleline(&mut self.name_input);
                            ui.label("Magazine Year:");
                            ui.text_edit_singleline(&mut self.year_input);
                            ui.label("Magazine Superhero");
                            ui.text_edit_singleline(&mut self.superhero_input);
                            ui.label("ISBN code:");
                            ui.text_edit_singleline(&mut self.isbn_input);
                        });

                        if ui.button("Add book").clicked() {

                            let book_year_int = match self.year_input.trim().parse::<i32>() {
                                Ok(num) => num,
                                Err(_) => {
                                    eprintln!("Invalid input!");
                                    return;
                                }
                            };

                            self.magazines.push(Magazine::create(self.isbn_input.clone(), self.name_input.clone(), book_year_int, self.superhero_input.clone()));
                            self.add_command = AddCommand::None;
                            self.clear_inputs();
                        }
                    });
            }
        }

        if let Some(selected_book) = &self.selected_book_for_borrow {
            // try Rc::new() 
            let book_for_borrow = selected_book.clone();
            Window::new("Borrow Book")
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.label("Borrower's Name:");
                        ui.text_edit_singleline(&mut self.name_input);
                    });

                    let selected_name = String::from(&self.name_input);
                
                    if ui.button("Confirm Borrow").clicked() {
                        borrow_item_for_reader(&mut self.readers, book_for_borrow, selected_name, &mut self.books);
                        self.selected_book_for_borrow = None;
                        self.clear_inputs();
                    }
                });
        }

        if let Some(selected_magazine)  = &self.selected_magazine_for_borrow {
            let magazine_for_borrow = selected_magazine.clone();
            Window::new("Borrow Magazine")
                .show(ctx, |ui| {

                    ui.horizontal(|ui| {
                        ui.label("Borrower's Name:");
                        ui.text_edit_singleline(&mut self.name_input);
                    });

                    let selected_name = String::from(&self.name_input);
                
                    if ui.button("Confirm Borrow").clicked() {
                        borrow_item_for_reader(&mut self.readers, magazine_for_borrow, selected_name, &mut self.magazines);
                        self.selected_magazine_for_borrow = None;
                        self.clear_inputs();
                    }
                });
        }

        CentralPanel::default().show(ctx, |ui| {
            render_header(ui);
            ScrollArea::auto_sized().show(ui, |ui| {
                self.render_depository_items(ui, ctx);
            });
        });
    }

    fn on_exit(&mut self) {

        match persistence::save_to_file("books_depo.json", &self.books, &self.magazines, &self.readers) {
            Ok(()) => println!("Data saved successfully."),
            Err(err) => eprintln!("Error saving data: {}", err),
        }

        println!("App is about to close!");
    }

    fn name(&self) -> &str {
        "Book Depository"
    }
}

fn render_header(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Book Depo");
    });
    ui.add_space(PADDING);
    let sep = Separator::default().spacing(20.);
    ui.add(sep);
}

fn main() {
    let depository_books: Vec<Book>;
    let depository_magazines: Vec<Magazine>;
    let depository_readers: Vec<Reader>;

    match persistence::load_from_file("books_depo.json") {
        Ok(read_result, ) => {
            depository_books = read_result.0;
            depository_magazines = read_result.1;
            depository_readers = read_result.2;  
        },
        Err(err) => {
            eprintln!("Error loading book depository data: {}", err);
            return;
        },
    }

    /* let number = 24;
    
    let number_rc = Rc::new(number);
    let number_arc = Arc::new(number);

    let number_rc_sum = *number_rc + 17;
    */

    let app = BookDepository {
        books: depository_books,
        magazines: depository_magazines,
        readers: depository_readers,
        selected_magazine_for_borrow: None,
        selected_book_for_borrow: None,

        name_input: String::from(""),
        year_input: String::from(""),
        superhero_input: String::from(""),
        isbn_input: String::from(""),

        add_command: AddCommand::None
    };
    let win_option = NativeOptions::default();
    
    run_native(Box::new(app), win_option);
}

fn borrow_item_for_reader<T: DepositoryItem>(readers: &mut Vec<Reader>, selected_item: T, selected_reader_name: String, items: &mut Vec<T>) {
    for i in 0..readers.len() {
        if readers[i].name.trim().to_lowercase() == selected_reader_name.to_lowercase() {
            remove_depository_item(items, selected_item.get_isbn_code());
            selected_item.borrow_for_reader(&mut readers[i]);
            return;
        }
    }
    println!("No reader was found");
}

fn remove_depository_item<T: DepositoryItem>(items: &mut Vec<T>, item_code: &String) {
    for i in (0..items.len()).rev() {
        if items[i].get_isbn_code() == item_code {
            items.swap_remove(i);
            println!("Found and removed item");
            break;
        }
    }
}

