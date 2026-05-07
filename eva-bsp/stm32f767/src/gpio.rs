#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputType {
    PushPull,
    OpenDrain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Input,
    Output(OutputType),
    Alternate(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pull {
    None,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Speed {
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Port {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trigger {
    RisingEdge,
    FallingEdge,
    BothEdge
}

fn port_to_bank(port: Port) -> eva_pac::gpio::Gpio {
    match port {
        Port::A => eva_pac::GPIOA,
        Port::B => eva_pac::GPIOB,
        Port::C => eva_pac::GPIOC,
        Port::D => eva_pac::GPIOD,
        Port::E => eva_pac::GPIOE,
        Port::F => eva_pac::GPIOF,
        Port::G => eva_pac::GPIOG,
        Port::H => eva_pac::GPIOH,
        Port::I => eva_pac::GPIOI,
        Port::J => eva_pac::GPIOJ,
        Port::K => eva_pac::GPIOK,
    }
}

fn port_to_exti_pin(port: Port) -> u8 {
    match port {
        Port::A => 0b0000,
        Port::B => 0b0001,
        Port::C => 0b0010,
        Port::D => 0b0011,
        Port::E => 0b0100,
        Port::F => 0b0101,
        Port::G => 0b0110,
        Port::H => 0b0111,
        Port::I => 0b1000,
        Port::J => 0b1001,
        Port::K => 0b1010,
    }
}

fn get_exti_num(num: usize) -> usize {
    match num {
        0 => 6,
        1 => 7,
        2 => 8,
        3 => 9,
        4 => 10,
        5..=9 => 23,
        10..=15 => 40,
        _ => unreachable!()
    }
}

pub fn set_mode(port: Port, num: usize, mode: Mode, pull: Pull, speed: Speed) {
    let bank = port_to_bank(port);
    unsafe {
        use eva_pac::gpio::*;

        bank.moder().update(|reg| {
            reg.set_moder(
                num,
                match mode {
                    Mode::Input => ModerVal::Input,
                    Mode::Output(_) => ModerVal::Output,
                    Mode::Alternate(_) => ModerVal::Alternate,
                },
            )
        });

        if let Mode::Output(mode) = mode {
            bank.otyper().update(|reg| {
                reg.set_ot(
                    num,
                    match mode {
                        OutputType::OpenDrain => OtVal::OpenDrain,
                        OutputType::PushPull => OtVal::PushPull,
                    },
                )
            })
        }

        if let Mode::Alternate(mode) = mode {
            bank.afr(num / 8)
                .update(|reg| reg.set_afr(num % 8, mode as _));
        }

        bank.ospeedr().update(|reg| {
            reg.set_ospeedr(
                num,
                match speed {
                    Speed::Low => OspeedrVal::LowSpeed,
                    Speed::Medium => OspeedrVal::MediumSpeed,
                    Speed::High => OspeedrVal::HighSpeed,
                    Speed::VeryHigh => OspeedrVal::VeryHighSpeed,
                },
            )
        });

        bank.pupdr().update(|reg| {
            reg.set_pupdr(
                num,
                match pull {
                    Pull::None => PupdrVal::Floating,
                    Pull::Up => PupdrVal::PullUp,
                    Pull::Down => PupdrVal::PullDown,
                },
            )
        });
    }
}

pub fn high(port: Port, num: usize) {
    let bank = port_to_bank(port);
    unsafe {
        use eva_pac::gpio::*;
        bank.odr().update(|reg| {
            reg.set_odr(num, OdrVal::High)
        });
    }
}

pub fn low(port: Port, num: usize) {
    let bank = port_to_bank(port);
    unsafe {
        use eva_pac::gpio::*;
        bank.odr().update(|reg| {
            reg.set_odr(num, OdrVal::Low)
        });
    }
}

pub fn enable_mco2_out(div: eva_pac::rcc::McopreVal) {
    // Output MCO2 on PC9
    
    unsafe {
        // First enable MCO2 output
        eva_pac::RCC.cfgr().update(|reg| {
            use eva_pac::rcc::*;
            reg.set_mco2sel(Mco2selVal::Sys)
                .set_mco2pre(div)
        });
    }
    
    set_mode(Port::C, 9, Mode::Alternate(0), Pull::None, Speed::VeryHigh);
}

pub fn enable_irq(port: Port, num: usize, trigger: Trigger, prio: u8) {
    unsafe {
        eva_pac::SYSCFG.exticr(num / 4).update(|reg| {
            reg.set_exti(num % 4, port_to_exti_pin(port))
        });
        
        eva_pac::EXTI.imr(0).update(|reg| {
            reg.set_line(num, true)
        });
        
        if trigger == Trigger::BothEdge || trigger == Trigger::RisingEdge {
            eva_pac::EXTI.rtsr(0).update(|reg| {
                reg.set_line(num, true)
            })
        }
        
        if trigger == Trigger::BothEdge || trigger == Trigger::FallingEdge {
            eva_pac::EXTI.ftsr(0).update(|reg| {
                reg.set_line(num, true)
            })
        }
        
        let irqn = get_exti_num(num);
        
        eva_pac::NVIC.iser(irqn >> 5).write({
            eva_pac::nvic::IserBits::default()
                .set_setena(irqn & 0x1f, true)
        });
        
        eva_pac::NVIC.ip(irqn).write(prio);
    }
}

pub fn reset_irq(num: usize) {
    unsafe {
        eva_pac::EXTI.pr(0).write({
            use eva_pac::exti::*;
            LinesBits::default().set_line(num, true)
        });
    }
}