use crate::program::{combine_u16, combine_u8, Program, ProgramStatus};

impl Program {
    fn jmc(&mut self, location: u16) -> bool {
        self.log_operation(format!("Jump to {} (with carry)", location).as_str());
        if self.carry {
            self.pointer = location as usize;
            return true;
        }

        self.log_info("Failed condition, not jumping");
        return false;
    }

    fn jmp(&mut self, location: u16) {
        self.log_operation(format!("Jump to {}", location).as_str());
        self.pointer = location as usize;
    }
    fn nop(&self) {
        self.log_operation("No Operation");
    }
}

impl Program {
    pub fn execute(&mut self) {
        while self.status != ProgramStatus::Exited {
            let instruction = self.instructions[self.pointer];
            match instruction {
                [0, 0, 0, 0, 0, 0, 0, 0] => self.nop(),
                [1, 0, 0, 0, _, _, _, _] => {
                    self.jmp(combine_u16(
                        combine_u8(instruction[4], instruction[5]),
                        combine_u8(instruction[6], instruction[7]),
                    ));
                    continue;
                }
                [1, 1, 0, 0, _, _, _, _] => {
                    if self.jmc(combine_u16(
                        combine_u8(instruction[4], instruction[5]),
                        combine_u8(instruction[6], instruction[7]),
                    )) {
                        continue;
                    }
                }
                [2, 0, 0, _, _, _, _, _] => {}
                _ => self.pass_error(
                    format!(
                        "No instruction exists for operation {}",
                        format!("{:x?}", instruction)
                            .replace(", ", "")
                            .as_str()
                            .trim_start_matches("[")
                            .trim_end_matches("]")
                    )
                    .as_str(),
                ),
            }

            if self.status != ProgramStatus::Exited {
                self.pointer += 1;
                if self.pointer >= self.instructions.len() {
                    self.set_status(ProgramStatus::Exited)
                }
            }
        }
    }
}
