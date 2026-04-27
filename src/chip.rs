use core::ptr::read_volatile;

use apmg32r430xx_pac::Peripherals;

use crate::board::{rcm_lock, rcm_unlock};

const FLASH_SIZE_REGISTER: *const u32 = 0x0802_0208 as *const u32;
const UID_BASE: *const u32 = 0x0802_020C as *const u32;
const TS_READY_TIMEOUT: u32 = 1_000_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TsError {
    Timeout,
}

pub fn read_uid_words() -> [u32; 3] {
    unsafe {
        [
            read_volatile(UID_BASE),
            read_volatile(UID_BASE.add(1)),
            read_volatile(UID_BASE.add(2)),
        ]
    }
}

pub fn read_flash_size_kb() -> u16 {
    unsafe { (read_volatile(FLASH_SIZE_REGISTER) & 0xffff) as u16 }
}

pub fn init_temperature_sensor(dp: &Peripherals) {
    rcm_unlock(dp);
    dp.rcm.apbcg().modify(|_, w| w.tscen().set_bit());
    rcm_lock(dp);

    dp.ts.cr().modify(|_, w| unsafe { w.osr_sel().bits(0) });
    dp.ts.cr().modify(|_, w| w.ovrmod().set_bit());
    dp.ts.cr().modify(|_, w| w.ts_en().set_bit());
}

pub fn read_temperature_c(dp: &Peripherals) -> Result<f32, TsError> {
    if dp.ts.sr().read().ovr().bit_is_set() {
        dp.ts.sr().modify(|_, w| w.ovr().set_bit());
    }

    let mut timeout = TS_READY_TIMEOUT;
    while dp.ts.sr().read().rdy().bit_is_clear() {
        if timeout == 0 {
            return Err(TsError::Timeout);
        }
        timeout -= 1;
    }

    let raw = dp.ts.dr().read().dr().bits();
    Ok(convert_raw_temperature(raw))
}

fn convert_raw_temperature(raw: u16) -> f32 {
    let sign = (raw >> 15) & 0x01;
    let int_part = ((raw >> 6) & 0x01ff) as f32;
    let frac_part = (raw & 0x003f) as f32 / 64.0;
    let value = int_part + frac_part;

    if sign != 0 { -value } else { value }
}
