use crate::hal::pac;

macro_rules! native_wrapper {
    ($name:ident, $pac_ty:ty, $rb:ty) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $name;

        impl $name {
            pub(crate) fn new() -> Self {
                Self
            }

            pub fn regs(&self) -> &'static $rb {
                unsafe { &*<$pac_ty>::ptr() }
            }
        }
    };
}

native_wrapper!(Dma, pac::Dma, pac::dma::RegisterBlock);
native_wrapper!(Rcm, pac::Rcm, pac::rcm::RegisterBlock);
native_wrapper!(Fmc, pac::Fmc, pac::fmc::RegisterBlock);
native_wrapper!(Adc1, pac::Adc1, pac::adc1::RegisterBlock);
native_wrapper!(Adc2, pac::Adc2, pac::adc2::RegisterBlock);
native_wrapper!(Adc3, pac::Adc3, pac::adc3::RegisterBlock);
native_wrapper!(Tmr1, pac::Tmr1, pac::tmr1::RegisterBlock);
native_wrapper!(Tmr2, pac::Tmr2, pac::tmr2::RegisterBlock);
native_wrapper!(Tmr3, pac::Tmr3, pac::tmr3::RegisterBlock);
native_wrapper!(Tmr4, pac::Tmr4, pac::tmr4::RegisterBlock);
native_wrapper!(Wwdt, pac::Wwdt, pac::wwdt::RegisterBlock);
native_wrapper!(Iwdt, pac::Iwdt, pac::iwdt::RegisterBlock);
native_wrapper!(Eint, pac::Eint, pac::eint::RegisterBlock);
native_wrapper!(Dac1, pac::Dac1, pac::dac1::RegisterBlock);
native_wrapper!(Dac2, pac::Dac2, pac::dac2::RegisterBlock);
native_wrapper!(Comp, pac::Comp, pac::comp::RegisterBlock);
native_wrapper!(Pmu, pac::Pmu, pac::pmu::RegisterBlock);
native_wrapper!(Bakpr, pac::Bakpr, pac::bakpr::RegisterBlock);
native_wrapper!(Rtc, pac::Rtc, pac::rtc::RegisterBlock);
native_wrapper!(Lptmr, pac::Lptmr, pac::lptmr::RegisterBlock);
native_wrapper!(Ts, pac::Ts, pac::ts::RegisterBlock);
native_wrapper!(Dbgmcu, pac::Dbgmcu, pac::dbgmcu::RegisterBlock);
