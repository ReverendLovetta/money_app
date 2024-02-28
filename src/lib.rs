mod graph;
mod income;
mod storage;

use graph::Graph;
use income::Income;
use storage::Storage;

pub struct App {}

impl App {
    pub fn new() -> App {
        App {}
    }

    pub fn exit(&self) {}

    pub fn run(&self) {
        let money: f64 = 100.0;
        let mut income = Income::new(money);
        income.split();
        Graph::show_splitting(&income);
        Storage::save_into_storage(income);
    }
}
