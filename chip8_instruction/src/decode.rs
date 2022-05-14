use crate::Instruction;

struct RawParts {
    pub low: u8,
    pub nib0: u8,
    pub nib1: u8,
    pub nib2: u8,
    pub nib3: u8,
    pub nnn: u16, // last 12 bits
}

pub fn decode(raw: u16) -> Option<Instruction> {
    let parts = {
        let lo = (raw & 0xff) as u8;
        let hi = ((raw >> 8) & 0x00ff) as u8;
        
        RawParts {
            low: lo,
            nib0: (hi & 0xf0) >> 4,
            nib1: (hi & 0x0f),
            nib2: (lo & 0xf0) >> 4,
            nib3: (lo & 0x0f),
            nnn: raw & 0x0fff,
        }
    };

    Some(match parts.nib0 {
        0 => {
            match parts.nib1 {
                0x0 => match parts.low {
                    0xe0 => Instruction::ClearScreen,
                    0xee => Instruction::Return,
                    _ => return None,
                },
                _ => Instruction::SysJmp(parts.nnn),
            }
        },
        1 => Instruction::Jump(parts.nnn),
        2 => Instruction::Call(parts.nnn),
        3 => Instruction::SkipIfEqualImmidiate(parts.nib1, parts.low),
        4 => Instruction::SkipIfNotEqualImmidiate(parts.nib1, parts.low),
        5 => match parts.nib3 {
            0 => Instruction::SkipIfEqualRegister(parts.nib1, parts.nib2),
            _ => return None,
        },
        6 => Instruction::LoadImmidiate(parts.nib1, parts.low),
        7 => Instruction::AddImmidiate(parts.nib1, parts.low),
        8 => match parts.nib3 {
            0 => Instruction::LoadRegister(parts.nib1, parts.nib2),
            1 => Instruction::OrRegister(parts.nib1, parts.nib2),
            2 => Instruction::AndRegister(parts.nib1, parts.nib2),
            3 => Instruction::XorRegister(parts.nib1, parts.nib2),
            4 => Instruction::AddRegister(parts.nib1, parts.nib2),
            5 => Instruction::SubRegister(parts.nib1, parts.nib2),
            6 => Instruction::ShrRegister(parts.nib1, parts.nib2),
            7 => Instruction::SubnRegister(parts.nib1, parts.nib2),
            0xe => Instruction::ShlRegister(parts.nib1, parts.nib2),
            _ => return None,
        },
        9 => match parts.nib3 {
            0 => Instruction::SkipIfNotEqualRegister(parts.nib1, parts.nib2),
            _ => return None,
        },
        0xa => Instruction::LoadI(parts.nnn),
        0xb => Instruction::JumpV0(parts.nnn),
        0xc => Instruction::Random(parts.nib1, parts.low),
        0xd => Instruction::DisplaySprite(parts.nib1, parts.nib2, parts.nib3),
        0xe => match parts.low {
            0x9e => Instruction::SkipIfPressed(parts.nib1),
            0xa1 => Instruction::SkipIfNotPressed(parts.nib1),
            _ => return None,
        }
        0xf => match parts.low {
            0x07 => Instruction::LoadDelayTimer(parts.nib1),
            0x0a => Instruction::ReadKey(parts.nib1),
            0x15 => Instruction::SetDelayTimer(parts.nib1),
            0x18 => Instruction::SetSoundTimer(parts.nib1),
            0x1e => Instruction::AddI(parts.nib1),
            0x29 => Instruction::LoadSpriteLocationI(parts.nib1),
            0x33 => Instruction::StoreDecimalI(parts.nib1),
            0x55 => Instruction::RegDumpI(parts.nib1),
            0x65 => Instruction::RegLoadI(parts.nib1),
            _ => return None,
        },
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! instr {
        ($raw:literal => <none>) => {
            assert_eq!(decode($raw), Option::None);
        };
        ($raw:literal => $res:expr) => {
            assert_eq!(decode($raw), Some($res));
        };
    }

    #[test]
    fn decoding_works() {
        use Instruction::*;
        
        instr!(0x00e0 => ClearScreen);
        
        instr!(0x00ee => Return);
        
        instr!(0x000e => <none>);
        
        instr!(0x1345 => Jump(0x345));
        instr!(0x1432 => Jump(0x432));
        
        instr!(0x2543 => Call(0x543));
        instr!(0x2123 => Call(0x123));
        
        instr!(0x3256 => SkipIfEqualImmidiate(0x2, 0x56));
        instr!(0x3333 => SkipIfEqualImmidiate(0x3, 0x33));

        instr!(0x4987 => SkipIfNotEqualImmidiate(0x9, 0x87));
        instr!(0x4312 => SkipIfNotEqualImmidiate(0x3, 0x12));
        
        instr!(0x5120 => SkipIfEqualRegister(0x1, 0x2));
        instr!(0x5320 => SkipIfEqualRegister(0x3, 0x2));
        instr!(0x5121 => <none>);

        instr!(0x6123 => LoadImmidiate(0x1, 0x23));
        instr!(0x6ff3 => LoadImmidiate(0xf, 0xf3));
        
        instr!(0x7123 => AddImmidiate(0x1, 0x23));
        instr!(0x7ff3 => AddImmidiate(0xf, 0xf3));
        
        instr!(0x8120 => LoadRegister(0x1, 0x2));
        instr!(0x8830 => LoadRegister(0x8, 0x3));

        instr!(0x8121 => OrRegister(0x1, 0x2));
        instr!(0x8221 => OrRegister(0x2, 0x2));

        instr!(0x8122 => AndRegister(0x1, 0x2));
        instr!(0x8ff2 => AndRegister(0xf, 0xf));

        instr!(0x8123 => XorRegister(0x1, 0x2));
        instr!(0x8163 => XorRegister(0x1, 0x6));

        instr!(0x8124 => AddRegister(0x1, 0x2));
        instr!(0x8ed4 => AddRegister(0xe, 0xd));

        instr!(0x8125 => SubRegister(0x1, 0x2));
        instr!(0x8aa5 => SubRegister(0xa, 0xa));

        instr!(0x8126 => ShrRegister(0x1, 0x2));
        instr!(0x8676 => ShrRegister(0x6, 0x7));

        instr!(0x8127 => SubnRegister(0x1, 0x2));
        instr!(0x8bb7 => SubnRegister(0xb, 0xb));

        instr!(0x812e => ShlRegister(0x1, 0x2));
        instr!(0x8eee => ShlRegister(0xe, 0xe));

        instr!(0x8aa9 => <none>);
        instr!(0x8aaa => <none>);
        instr!(0x8aab => <none>);
        instr!(0x8aac => <none>);
        instr!(0x8aad => <none>);
        instr!(0x8aaf => <none>);
     
        instr!(0x9ab0 => SkipIfNotEqualRegister(0xa, 0xb));
        instr!(0x94d0 => SkipIfNotEqualRegister(0x4, 0xd));
        instr!(0x9ab1 => <none>);

        instr!(0xaaaa => LoadI(0xaaa));
        instr!(0xa123 => LoadI(0x123));
        
        instr!(0xbaaa => JumpV0(0xaaa));
        instr!(0xbabe => JumpV0(0xabe));
        
        instr!(0xc123 => Random(0x1, 0x23));
        instr!(0xcafe => Random(0xa, 0xfe));
        
        instr!(0xd123 => DisplaySprite(0x1, 0x2, 0x3));
        instr!(0xdead => DisplaySprite(0xe, 0xa, 0xd));
        
        instr!(0xe19e => SkipIfPressed(0x1));
        instr!(0xe39e => SkipIfPressed(0x3));
        
        instr!(0xe1a1 => SkipIfNotPressed(0x1));
        instr!(0xe2a1 => SkipIfNotPressed(0x2));
        instr!(0xe1ab => <none>);

        instr!(0xf107 => LoadDelayTimer(0x1));
        instr!(0xff07 => LoadDelayTimer(0xf));
        
        instr!(0xf10a => ReadKey(0x1));
        instr!(0xf30a => ReadKey(0x3));

        instr!(0xf115 => SetDelayTimer(0x1));
        instr!(0xfa15 => SetDelayTimer(0xa));
        
        instr!(0xf118 => SetSoundTimer(0x1));
        instr!(0xfd18 => SetSoundTimer(0xd));
        
        instr!(0xf11e => AddI(0x1));
        instr!(0xf71e => AddI(0x7));
        
        instr!(0xf129 => LoadSpriteLocationI(0x1));
        instr!(0xfc29 => LoadSpriteLocationI(0xc));
        
        instr!(0xf133 => StoreDecimalI(0x1));
        instr!(0xf633 => StoreDecimalI(0x6));
        
        instr!(0xf155 => RegDumpI(0x1));
        instr!(0xf955 => RegDumpI(0x9));
        
        instr!(0xf165 => RegLoadI(0x1));
        instr!(0xff65 => RegLoadI(0xf));

        instr!(0xff11 => <none>);
        instr!(0xff19 => <none>);
        instr!(0xff31 => <none>);
        instr!(0xffff => <none>);
        instr!(0xff67 => <none>);
    }
}
