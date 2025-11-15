use mb8_isa::registers::Register;

use crate::{mem::regions::MemoryRegion, vm::VirtualMachine};

impl VirtualMachine {
    const SCREEN_WIDTH: u16 = 64;
    const SCREEN_HEIGHT: u16 = 32;
    const BYTES_PER_ROW: u16 = Self::SCREEN_WIDTH / 8;

    pub fn draw(&mut self, x_reg: Register, y_reg: Register, height: u8) {
        let sprite_addr = self.registers.read(Register::I);
        let x0 = self.registers.read(x_reg);
        let y0 = self.registers.read(y_reg);

        // let mut collision = false;

        for row in 0..height as u16 {
            let py = (y0 + row) % Self::SCREEN_HEIGHT;

            let sprite_byte = {
                let general = self.mem.general();
                general.read(sprite_addr + row)
            };

            if sprite_byte == 0 {
                continue;
            }

            for bit in 0..8u16 {
                let mask_in_byte = 0x80u8 >> bit;

                if sprite_byte & mask_in_byte == 0 {
                    continue;
                }

                let px = (x0 + bit) % Self::SCREEN_WIDTH;

                let byte_index = py * Self::BYTES_PER_ROW + (px / 8);
                let bit_pos = px % 8;
                let pixel_mask = 0x80u8 >> bit_pos;

                let old = {
                    let gfx = self.mem.graphic_buffer();
                    gfx.read(byte_index)
                };

                let new = old ^ pixel_mask;

                if (old & pixel_mask) != 0 && (new & pixel_mask) == 0 {
                    // collision = true;
                }

                {
                    let mut gfx = self.mem.graphic_buffer();
                    gfx.write(byte_index, new);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use mb8_isa::opcodes::Opcode;
    use mb8_isa::registers::Register;

    use super::*;
    use crate::mem::regions::MemoryRegion;

    #[test]
    fn test_opcode_draw() {
        let mut vm = VirtualMachine::new();

        vm.mem.general().write(0x123, 0b0000_0000);
        vm.mem.general().write(0x124, 0b0010_0100);
        vm.mem.general().write(0x125, 0b0111_1110);
        vm.mem.general().write(0x126, 0b0011_1100);
        vm.mem.general().write(0x127, 0b0001_1000);
        vm.mem.general().write(0x128, 0b0000_0000);

        vm.registers.write(Register::I, 0x123);
        vm.registers.write(Register::R0, 4);
        vm.registers.write(Register::R1, 4);

        vm.execute(&Opcode::Draw {
            x: Register::R0,
            y: Register::R1,
            height: 6,
        });

        assert_eq!(vm.mem.graphic_buffer().read(0x20), 0b0000_0000);
        assert_eq!(vm.mem.graphic_buffer().read(0x21), 0b0000_0000);

        assert_eq!(vm.mem.graphic_buffer().read(0x28), 0b0000_0010);
        assert_eq!(vm.mem.graphic_buffer().read(0x29), 0b0100_0000);

        assert_eq!(vm.mem.graphic_buffer().read(0x30), 0b0000_0111);
        assert_eq!(vm.mem.graphic_buffer().read(0x31), 0b1110_0000);
    }
}
