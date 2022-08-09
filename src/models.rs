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

#[derive(Debug)]
pub struct TaskContext<'a> {
    pub names: Vec<&'a str>,
    pub task_options: Vec<&'a str>,
    pub global_options: Vec<&'a str>,
}

impl TaskContext<'_> {
    pub fn new(args: Vec<&str>) -> TaskContext {
        let len = args.len();
        let mut task_options: Vec<&str> = Vec::new();
        let mut global_options: Vec<&str> = Vec::new();
        let task_global_options_index = args.iter().position(|&arg| arg == "--").unwrap_or(0);
        let mut task_options_index = args.iter().position(|&arg| arg.starts_with("-") && arg != "--").unwrap_or(0);
        let first_option_index = args.iter().position(|&arg| arg.starts_with("-")).unwrap_or(0);
        if task_global_options_index > 0 && task_options_index > task_global_options_index { // no task options found
            task_options_index = 0;
        }
        //slice global options
        if task_global_options_index > 0 {
            global_options = args[task_global_options_index + 1..len].to_vec();
        }
        //slice task options
        if task_options_index > 0 {
            if task_global_options_index > 0 {
                task_options = args[task_options_index..task_global_options_index].to_vec();
            } else {
                task_options = args[task_options_index..len].to_vec();
            }
        }
        //slice task names
        let names = if first_option_index > 0 {
            args[0..first_option_index].to_vec()
        } else {
            args[0..len].to_vec()
        };
        TaskContext {
            names,
            task_options,
            global_options,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_task_context() {
        let args = vec!["helo", "second", "-", "linux_china"];
        println!("{:?}", TaskContext::new(args));
        let args = vec!["helo", "second", "-n", "linux_china", "--", "--verbose", "--debug"];
        println!("{:?}", TaskContext::new(args));
        let args = vec!["helo", "second", "-n", "linux_china", "--"];
        println!("{:?}", TaskContext::new(args));
    }
}
