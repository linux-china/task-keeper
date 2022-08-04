#[macro_export]
macro_rules! task {
    ($name:expr, $runner:expr) => {
       Task { name: $name.to_owned(), runner: $runner.to_owned(), description: "".to_owned()}
    };
    ($name:expr, $runner:expr, $description:expr) => {
       Task { name: $name.to_owned(), runner: $runner.to_owned(), description: $description.to_owned()}
    };
}

#[derive(Debug)]
pub struct Task {
    pub name: String,
    pub runner: String,
    pub description: String,
}
