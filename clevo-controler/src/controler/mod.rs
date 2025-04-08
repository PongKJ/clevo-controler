use crate::component::Visitor;

pub mod pid;
pub mod range;
pub mod table;

#[derive(Debug)]
enum ControlMethod {
    TableLook,
    TempRange,
    Pid,
}

#[derive(Debug)]
pub struct Controler {
    expect_temp: f32,
}

impl Controler {
    pub fn new(expect_temp: f32) -> Self {
        Self { expect_temp }
    }

    pub fn run(&mut self) {}
}

impl Visitor for Controler {
    fn visit_cpu(&mut self, cpu: &crate::component::cpu::Cpu) {
        // 访问 CPU 组件
        println!("Visiting CPU: {:#?}", cpu);
        // 在这里可以执行一些操作，例如获取 CPU 的频率、温度等信息
        // self.expect_temp = cpu.get_temp();
    }
}
