pub trait Processable {
    fn key(&self) -> String; // define what "sorting key" mean for each type
}