use requests::test;

slint::include_modules!();
mod requests;


fn main() {
    let _ui = MainWindow::new().unwrap().run().unwrap();
    
    test(); 
}
    