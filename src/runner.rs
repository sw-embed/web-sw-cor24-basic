use std::collections::VecDeque;

use cor24_emulator::{EmulatorCore, StopReason};

use crate::config;

const BATCH_SIZE: u64 = 200_000;
const P24_LOAD_ADDR: u32 = 0x010000;

mod vm_offsets {
    pub const PC: u32 = 0;
    pub const ESP: u32 = 3;
    pub const CSP: u32 = 6;
    pub const FP_VM: u32 = 9;
    pub const GP: u32 = 12;
    pub const HP: u32 = 15;
    pub const CODE: u32 = 18;
    pub const STATUS: u32 = 21;
    pub const TRAP_CODE: u32 = 24;
}

fn pcode_instr_size(op: u8) -> u32 {
    match op {
        0x01 | 0x30 | 0x31 | 0x32 | 0x33 | 0x54 | 0x55 | 0x56 => 4,
        0x02 | 0x36 | 0x40 | 0x42 | 0x43 | 0x44 | 0x45 | 0x57 | 0x60 | 0x34 | 0x35 => 2,
        0x58 | 0x59 => 3,
        0x5A => 5,
        _ => 1,
    }
}

pub struct Session {
    emu: EmulatorCore,
    pub instructions: u64,
    pub done: bool,
    pub stop_reason: String,
    pub halted: bool,
    uart_rx_queue: VecDeque<u8>,
    vm_state_addr: u32,
    code_seg_addr: u32,
    vm_loop_addr: u32,
    accumulated_output: String,
    uart_seen: usize,
}

pub struct TickResult {
    pub done: bool,
}

impl Session {
    pub fn new(basic_source: &str) -> Self {
        let mut emu = EmulatorCore::new();
        emu.set_uart_tx_busy_cycles(0);

        let binary = config::PVM_BINARY;
        let vm_state_addr = config::label_addr("vm_state");
        let code_seg_addr = config::label_addr("code_seg");
        let vm_loop_addr = config::label_addr("vm_loop");

        emu.hard_reset();
        emu.load_program(0, binary);
        emu.load_program_extent(binary.len() as u32);
        emu.set_pc(0);

        let mut session = Self {
            emu,
            instructions: 0,
            done: false,
            stop_reason: String::new(),
            halted: false,
            uart_rx_queue: VecDeque::new(),
            vm_state_addr,
            code_seg_addr,
            vm_loop_addr,
            accumulated_output: String::new(),
            uart_seen: 0,
        };

        session.load_basic_p24();
        if !session.done {
            session.init_vm();
        }
        session.queue_input(basic_source);

        session
    }

    fn load_basic_p24(&mut self) {
        let basic_p24 = config::BASIC_P24;
        match pa24r::load_p24(basic_p24) {
            Ok(image) => {
                let load_addr = P24_LOAD_ADDR;
                let code_size = image.code.len() as u32;
                let total = code_size + image.data.len() as u32;

                for (i, &b) in image.code.iter().chain(image.data.iter()).enumerate() {
                    self.emu.write_byte(load_addr + i as u32, b);
                }

                let mut i: u32 = 0;
                while i < code_size {
                    let op = self.emu.read_byte(load_addr + i);
                    let size = pcode_instr_size(op);
                    if op == 0x01 && i + 4 <= code_size {
                        let lo = self.emu.read_byte(load_addr + i + 1) as u32;
                        let mid = self.emu.read_byte(load_addr + i + 2) as u32;
                        let hi = self.emu.read_byte(load_addr + i + 3) as u32;
                        let val = lo | (mid << 8) | (hi << 16);
                        if val >= code_size && val < total {
                            let abs = val + load_addr;
                            self.emu.write_byte(load_addr + i + 1, abs as u8);
                            self.emu.write_byte(load_addr + i + 2, (abs >> 8) as u8);
                            self.emu.write_byte(load_addr + i + 3, (abs >> 16) as u8);
                        }
                    }
                    i += size;
                }
            }
            Err(e) => {
                self.stop_reason = format!("p24 load error: {e}");
                self.done = true;
            }
        }
    }

    fn init_vm(&mut self) {
        let code_seg = self.code_seg_addr;
        let load_addr = P24_LOAD_ADDR;

        self.emu.write_byte(code_seg, 0x60);
        self.emu.write_byte(code_seg + 1, 0x00);

        self.emu.resume();
        self.emu.run_batch(10_000);

        self.emu.clear_uart_output();

        self.emu.reset();
        self.emu.set_uart_tx_busy_cycles(0);
        self.emu.set_stack_bounds(0, 0);

        self.emu.set_pc(self.vm_loop_addr);
        self.emu.set_reg(3, self.vm_state_addr);

        let base = self.vm_state_addr;
        self.emu.write_byte(base + 18, load_addr as u8);
        self.emu.write_byte(base + 19, (load_addr >> 8) as u8);
        self.emu.write_byte(base + 20, (load_addr >> 16) as u8);
        self.emu.write_byte(base, 0);
        self.emu.write_byte(base + 1, 0);
        self.emu.write_byte(base + 2, 0);
        self.emu.write_byte(base + 21, 0);
        self.emu.write_byte(base + 22, 0);
        self.emu.write_byte(base + 23, 0);
    }

    fn queue_input(&mut self, source: &str) {
        for b in source.bytes() {
            self.uart_rx_queue.push_back(b);
        }
    }

    fn feed_uart_bytes(&mut self) {
        let mut feed_budget: u32 = 10_000;
        while !self.uart_rx_queue.is_empty() && feed_budget > 0 {
            let status = self.emu.read_byte(0xFF0101);
            if status & 0x01 != 0 {
                self.emu.run_batch(50);
                feed_budget = feed_budget.saturating_sub(50);
                continue;
            }
            let byte = self.uart_rx_queue.pop_front().unwrap();
            self.emu.send_uart_byte(byte);
            self.emu.run_batch(50);
            feed_budget = feed_budget.saturating_sub(50);
        }
    }

    fn collect_output(&mut self) {
        let uart = self.emu.get_uart_output();
        if uart.len() > self.uart_seen {
            self.accumulated_output.push_str(&uart[self.uart_seen..]);
            self.uart_seen = uart.len();
        }
    }

    pub fn tick(&mut self) -> TickResult {
        if self.done {
            return TickResult { done: true };
        }

        self.feed_uart_bytes();

        let result = self.emu.run_batch(BATCH_SIZE);
        self.instructions += result.instructions_run as u64;

        self.collect_output();

        let (_pc, _esp, _csp, _fp, _gp, _hp, _code, status, trap_code) = self.read_pcode_state();
        if status == 1 {
            self.collect_output();
            self.done = true;
            self.halted = true;
            self.stop_reason = "halted".into();
        } else if status == 2 {
            self.done = true;
            self.stop_reason = format!("trap {}", trap_code);
        }

        match result.reason {
            StopReason::Halted => {
                self.collect_output();
                self.done = true;
                self.halted = true;
                self.stop_reason = "emulator halted".into();
            }
            StopReason::InvalidInstruction(byte) => {
                self.done = true;
                self.stop_reason = format!(
                    "invalid instruction 0x{:02X} at PC=0x{:06X}",
                    byte,
                    self.emu.pc()
                );
            }
            StopReason::CycleLimit => {
                if result.instructions_run == 0 && !self.done {
                    self.done = true;
                    self.stop_reason = "stalled".into();
                }
            }
            _ => {}
        }

        TickResult { done: self.done }
    }

    fn read_pcode_state(&self) -> (u32, u32, u32, u32, u32, u32, u32, u32, u32) {
        let base = self.vm_state_addr;
        (
            self.emu.read_word(base + vm_offsets::PC),
            self.emu.read_word(base + vm_offsets::ESP),
            self.emu.read_word(base + vm_offsets::CSP),
            self.emu.read_word(base + vm_offsets::FP_VM),
            self.emu.read_word(base + vm_offsets::GP),
            self.emu.read_word(base + vm_offsets::HP),
            self.emu.read_word(base + vm_offsets::CODE),
            self.emu.read_word(base + vm_offsets::STATUS),
            self.emu.read_word(base + vm_offsets::TRAP_CODE),
        )
    }

    pub fn output(&self) -> String {
        let cleaned: String = self
            .accumulated_output
            .chars()
            .filter(|&c| c != '>')
            .collect();
        cleaned
    }
}
