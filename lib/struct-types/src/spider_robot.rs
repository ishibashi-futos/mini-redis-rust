use std::cell::{Cell, RefCell};

pub struct SpiderRobot<'a> {
    hardware_error_count: Cell<u32>,
    log_file: RefCell<Vec<&'a str>>,
}

impl<'a> SpiderRobot<'a> {
    pub fn new() -> SpiderRobot<'a> {
        SpiderRobot {
            hardware_error_count: Cell::new(0),
            log_file: RefCell::new(Vec::new())
        }
    }

    pub fn log(&self, message: &'a str) {
        let mut file = self.log_file.borrow_mut();
        file.push(message);
    }
}

// ライフタイム・型パラメータに関係のない関数はimplを分けることができる
impl SpiderRobot<'_> {
    pub fn add_hardware_error(&self) {
        let n = self.hardware_error_count.get();
        self.hardware_error_count.set(n + 1);
    }

    pub fn has_hardware_error(&self) -> bool {
        self.hardware_error_count.get() > 0
    }

    pub fn get_hardware_error(&self) -> u32 {
        self.hardware_error_count.get()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_hardware_error() {
        let robot = SpiderRobot::new();

        robot.add_hardware_error();
        let error_count = robot.get_hardware_error();

        assert_eq!(1, error_count);
    }

    #[test]
    fn has_hardware_error() {
        let robot = SpiderRobot::new();

        robot.add_hardware_error();
        let has_errors = robot.has_hardware_error();

        assert!(has_errors);
    }

    #[test]
    fn no_errors_have() {
        let robot = SpiderRobot::new();

        let has_errors = robot.has_hardware_error();

        assert!(!has_errors);
    }

    #[test]
    fn ref_cell_test() {
        use std::cell::RefCell;

        let ref_cell: RefCell<String> = RefCell::new("Hello".to_string());

        {
            let r = ref_cell.borrow();
            let count = r.len();

            assert_eq!(5, count);
        }

        {
            let mut w = ref_cell.borrow_mut();
            w.push_str(" world!");
        }

        assert_eq!("Hello world!".len(), ref_cell.borrow().len());
    }

}

#[test]
fn write_log() {
    let robot = SpiderRobot::new();

    robot.log("Hello");
    robot.log("World!");

    assert_eq!("Hello World!", robot.log_file.borrow_mut().join(" "));
}
