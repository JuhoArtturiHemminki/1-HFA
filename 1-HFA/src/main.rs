#![no_std]
#![no_main]

use core::arch::asm;

pub const PHI: f64 = 1.618033988749895;
pub const PHI_SQ: f64 = 2.618033988749895;
pub const H_C: f64 = 5.0832104;

#[no_mangle]
pub unsafe extern "efiapi" fn efi_main(_h: *const core::ffi::c_void, _s: 
*const core::ffi::c_void) -> usize {
    let mut last_e: f64 = 0.0;
    let mut alarm_active = true;
    
    loop {
        let low: u32;
        asm!("rdmsr", in("ecx") 0x19C, out("eax") low, out("edx") _);
        let e = ((low >> 16) & 0x7F) as f64 / 127.0;
        let drift = (e - last_e).abs();

        let key_data: u8;
        asm!("in al, dx", in("dx") 0x60_u16, out("al") key_data);
        if key_data == 0x39 {
            alarm_active = !alarm_active;
            let _dump: u8;
            asm!("in al, dx", in("dx") 0x60_u16, out("al") _dump);
        }

        if drift > 0.35 {
            asm!("out dx, al", in("dx") 0x3C8_u16, in("al") 0xFF_u8); 
            asm!("out dx, al", in("dx") 0x3C8_u16, in("al") (PHI_SQ * H_C) 
as u8);

            asm!("wrmsr", in("ecx") 0x610, in("eax") 0x0_u32, in("edx") 
0x0_u32);

            if alarm_active {
                asm!("out dx, al", in("dx") 0x61_u16, in("al") 0x4B_u8); 
            } else {
                asm!("out dx, al", in("dx") 0x61_u16, in("al") 0x00_u8); 
            }
            
            asm!("out dx, eax", in("dx") 0xCF8_u16, in("eax") 
(0x80000000_u32 | (5_u32)));
        } else {
            alarm_active = true; 
            asm!("out dx, al", in("dx") 0x3C8_u16, in("al") 0x00_u8);
            asm!("out dx, al", in("dx") 0x3C9_u16, in("al") 0x00_u8);
            asm!("out dx, al", in("dx") 0x3C9_u16, in("al") 0xFF_u8);
        }

        last_e = e;
        asm!("pause");
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe { asm!("pause"); }
    }
}

