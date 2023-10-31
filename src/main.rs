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


use book_depository::{BookDepository, PADDING, Item};
use serde::de;
use uuid::Uuid;

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

                            self.books.push(Rc::new(Book::new(self.isbn_input.clone(), self.name_input.clone(), book_year_int)));
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

                            self.magazines.push(Rc::new(Magazine::new(self.isbn_input.clone(), self.name_input.clone(), book_year_int, self.superhero_input.clone())));
                            self.add_command = AddCommand::None;
                            self.clear_inputs();
                        }
                    });
            }
        }

        if let Some(uuid) = self.selected_item_for_borrow_uuid {
            Window::new("Borrow item")
                .show(ctx, |ui| {

                    ui.horizontal(|ui| {
                        ui.label("Borrower's Name:");
                        ui.text_edit_singleline(&mut self.name_input);
                    });

                    let selected_name = String::from(&self.name_input);
                
                    let reader = self.readers.iter().find(|r| r.name.cmp(&selected_name).is_eq());
                    if ui.button("Confirm Borrow").clicked() {
                        if let Some(reader) = reader {
                            borrow_item_for_reader(&mut self.catalogue, &uuid,reader);
                            self.selected_item_for_borrow_uuid = None;
                        }
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

        match persistence::save_to_file("books_depo.json", &self.books, &self.magazines, &self.catalogue,&self.readers) {
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
    let depository_books: Vec<Rc<Book>>;
    let depository_magazines: Vec<Rc<Magazine>>;
    let depository_readers: Vec<Reader>;
    let depository_catalogue : Vec<Item>;

    match persistence::load_from_file("books_depo.json") {
        Ok(read_result, ) => {
            depository_books = read_result.0.into_iter().map(|b| {Rc::new(b)}).collect();
            depository_magazines = read_result.1.into_iter().map(|m| {Rc::new(m)}).collect();
            depository_readers = read_result.2;  
            let mut books_as_items: Vec<Item> = depository_books.iter().map(|b| {
                let reader = read_result.3.iter().find(|r| r.0.eq(b.get_uuid()));
                Item {
                    item: b.clone(),
                    reader: match reader {
                        Some(r) => {
                            Some(r.1.clone())
                        },
                        None => None,
                    }
                }
            }).collect();

            let magazines_as_items: Vec<Item> = depository_magazines.iter().map(|b| {
                let reader = read_result.3.iter().find(|r| r.0.eq(b.get_uuid()));
                Item {
                    item: b.clone(),
                    reader: match reader {
                        Some(r) => {
                            Some(r.1.clone())
                        },
                        None => None,
                    }
                }
            }).collect();
            println!("bi {}, mi {}", books_as_items.len(), magazines_as_items.len());
            books_as_items.extend(magazines_as_items.into_iter());

            depository_catalogue = books_as_items;
        },
        Err(err) => {
            eprintln!("Error loading book depository data: {}", err);
            return;
        },
    }

    let app = BookDepository {
        books: depository_books,
        magazines: depository_magazines,
        readers: depository_readers,
        catalogue: depository_catalogue,
        selected_item_for_borrow_uuid: None,
        name_input: String::from(""),
        year_input: String::from(""),
        superhero_input: String::from(""),
        isbn_input: String::from(""),

        add_command: AddCommand::None
    };
    let win_option = NativeOptions::default();
    
    run_native(Box::new(app), win_option);
}

fn borrow_item_for_reader(catalogue: &mut Vec<Item>, uuid: &Uuid, reader: &Reader) {
    println!("rrrr {} {}", uuid, reader.uuid);
    for item in catalogue.iter_mut() {
        if item.item.get_uuid().cmp(uuid).is_eq() {
            item.reader = Some(reader.uuid.clone());
        }
    }
}

fn remove_depository_item(depo: &mut BookDepository, uuid: Uuid) {
    if let Some(idx) = depo.catalogue.iter().position(|i| (*i.item.get_uuid()) == uuid) {
        depo.catalogue.swap_remove(idx);
    }

    if let Some(idx_books) = depo.books.iter().position(|b| (*b.get_uuid()) == uuid) {
        depo.books.swap_remove(idx_books); 
    }else if let Some(idx_magazines) = depo.magazines.iter().position(|m| (*m.get_uuid()) == uuid) {
        depo.magazines.swap_remove(idx_magazines);
    }else{
        println!("Didn't find item with uuid of: {}", uuid);
    }
}

