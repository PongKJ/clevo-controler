#[derive(Debug)]
enum ControlMethod {
    TableLook,
    TempRange,
    Pid,
}

#[derive(Debug)]
pub struct HwCtrl {
    expect_temp: f32,
}

impl HwCtrl {
    pub fn new(expect_temp: f32) -> Self {
        Self { expect_temp }
    }

    pub fn run(&mut self) {

    }
}
