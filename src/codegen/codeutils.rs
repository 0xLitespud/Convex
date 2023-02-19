use crate::utils::errors::error_and_exit;

pub struct Code {
    pub code: String, 
    tab: usize // 当前的缩进
}

impl Code {
    pub fn new() -> Self{
        Code {
            code: String::from(""), 
            tab: 0
        }
    }

    fn change_tab(&mut self, change: i32) {
        // 这种情况几乎不会发生，因为在语法分析阶段就已经排除了
        if self.tab == 0 && change < 0 {
            error_and_exit("Indent processing exception");
        }

        if change > 0 {
            self.tab += change as usize;
        } else {
            self.tab -= -(change) as usize;
        }
    }

    // 添加代码
    pub fn dump(&mut self, content: String) {
        self.code.push_str("\t".repeat(self.tab).as_str());
        self.code.push_str(content.as_str());
        self.code.push('\n');
    }

    // 添加代码并缩进
    pub fn dump_in(&mut self, content: String) {
        self.dump(content);
        self.change_tab(1);
    }

    // 取消缩进并添加代码
    pub fn dump_out(&mut self, content: String) {
        self.change_tab(-1);
        self.dump(content);
    }
}