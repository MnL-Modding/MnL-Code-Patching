//! Module that provides the addressess of various hardware registers.

// Taken from IronDS, which is licensed under the Zlib license:
// Copyright (C) 2022 Quinn Painter
//
// This software is provided 'as-is', without any express or implied
// warranty.  In no event will the authors be held liable for any damages
// arising from the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented; you must not
//    claim that you wrote the original software. If you use this software
//    in a product, an acknowledgment in the product documentation would be
//    appreciated but is not required.
// 2. Altered source versions must be plainly marked as such, and must not be
//    misrepresented as being the original software.
// 3. This notice may not be removed or altered from any source distribution.

#![allow(missing_docs)] // can remove this when all are voladdresses

use voladdress::{Safe, VolAddress, VolBlock};

// thanks rust-console/gba, this is a good idea!
macro_rules! def_mmio {
    ($addr:literal = $name:ident : $t:ty; [ $($cpu:expr),+ ] $(; $comment:expr )?) => {
        // redirect a call **without** an alias list to just pass an empty alias list
        def_mmio!($addr = $name/[]: $t; [$($cpu),+] $(; $comment)? );
    };
    ($addr:literal = $name:ident / [ $( $alias:literal ),* ]: $t:ty ; [ $($cpu:expr),+ ] $(; $comment:expr )?) => {
        $(#[doc = $comment])?
        // this could be formatted better, like:
        // "Accessible to arm9 and arm7"
        // or "Accessible to arm9 only"
        // but can't figure out a good way to do so
        #[doc = concat!("\n", $("\nAccessible to ", $cpu, "  "),+)]
        $(#[doc(alias = $alias)])*
        #[allow(missing_docs)]
        // #[cfg(any($(feature = $cpu),*))]
        pub const $name: $t = unsafe { <$t>::new($addr) };
    };
}

// https://www.problemkaputt.de/gbatek.htm#dsmemorycontrolvram
def_mmio!(0x0400_0240 = VRAMSTAT: VolAddress<u8, Safe, ()>; ["arm7"]; "VRAM Bank Status");
def_mmio!(0x0400_0240 = VRAMCNT_A: VolAddress<u8, (), Safe>; ["arm9"]; "VRAM-A Bank Control");
def_mmio!(0x0400_0241 = VRAMCNT_B: VolAddress<u8, (), Safe>; ["arm9"]; "VRAM-B Bank Control");
def_mmio!(0x0400_0242 = VRAMCNT_C: VolAddress<u8, (), Safe>; ["arm9"]; "VRAM-C Bank Control");
def_mmio!(0x0400_0243 = VRAMCNT_D: VolAddress<u8, (), Safe>; ["arm9"]; "VRAM-D Bank Control");
def_mmio!(0x0400_0244 = VRAMCNT_E: VolAddress<u8, (), Safe>; ["arm9"]; "VRAM-E Bank Control");
def_mmio!(0x0400_0245 = VRAMCNT_F: VolAddress<u8, (), Safe>; ["arm9"]; "VRAM-F Bank Control");
def_mmio!(0x0400_0246 = VRAMCNT_G: VolAddress<u8, (), Safe>; ["arm9"]; "VRAM-G Bank Control");
def_mmio!(0x0400_0248 = VRAMCNT_H: VolAddress<u8, (), Safe>; ["arm9"]; "VRAM-H Bank Control");
def_mmio!(0x0400_0249 = VRAMCNT_I: VolAddress<u8, (), Safe>; ["arm9"]; "VRAM-I Bank Control");

// https://www.problemkaputt.de/gbatek.htm#dsvideostuff
// #[cfg(feature = "arm9")]
pub const MASTER_BRIGHT_MAIN: usize = 0x0400006C;
// #[cfg(feature = "arm9")]
pub const MASTER_BRIGHT_SUB: usize = 0x0400106C;
// #[cfg(feature = "arm9")]
pub const DISPCNT_MAIN: usize = 0x04000000;
// #[cfg(feature = "arm9")]
pub const DISPCNT_SUB: usize = 0x04001000;
// #[cfg(feature = "arm9")]
pub const BG0CNT_MAIN: usize = 0x04000008;
// #[cfg(feature = "arm9")]
pub const BG0CNT_SUB: usize = 0x04001008;
// #[cfg(feature = "arm9")]
pub const BG1CNT_MAIN: usize = 0x0400000A;
// #[cfg(feature = "arm9")]
pub const BG1CNT_SUB: usize = 0x0400100A;
// #[cfg(feature = "arm9")]
pub const BG2CNT_MAIN: usize = 0x0400000C;
// #[cfg(feature = "arm9")]
pub const BG2CNT_SUB: usize = 0x0400100C;
// #[cfg(feature = "arm9")]
pub const BG3CNT_MAIN: usize = 0x0400000E;
// #[cfg(feature = "arm9")]
pub const BG3CNT_SUB: usize = 0x0400100E;

def_mmio!(0x0400_0010 = BG0XOFS_MAIN: VolAddress<u16, (), Safe>; ["arm9"]; "Main Background 0 X Offset");
def_mmio!(0x0400_1010 = BG0XOFS_SUB: VolAddress<u16, (), Safe>; ["arm9"]; "Sub Background 0 X Offset");
def_mmio!(0x0400_0012 = BG0YOFS_MAIN: VolAddress<u16, (), Safe>; ["arm9"]; "Main Background 0 Y Offset");
def_mmio!(0x0400_1012 = BG0YOFS_SUB: VolAddress<u16, (), Safe>; ["arm9"]; "Sub Background 0 Y Offset");
def_mmio!(0x0400_0014 = BG1XOFS_MAIN: VolAddress<u16, (), Safe>; ["arm9"]; "Main Background 1 X Offset");
def_mmio!(0x0400_1014 = BG1XOFS_SUB: VolAddress<u16, (), Safe>; ["arm9"]; "Sub Background 1 X Offset");
def_mmio!(0x0400_0016 = BG1YOFS_MAIN: VolAddress<u16, (), Safe>; ["arm9"]; "Main Background 1 Y Offset");
def_mmio!(0x0400_1016 = BG2YOFS_SUB: VolAddress<u16, (), Safe>; ["arm9"]; "Sub Background 1 Y Offset");
def_mmio!(0x0400_0018 = BG2XOFS_MAIN: VolAddress<u16, (), Safe>; ["arm9"]; "Main Background 2 X Offset");
def_mmio!(0x0400_1018 = BG2XOFS_SUB: VolAddress<u16, (), Safe>; ["arm9"]; "Sub Background 2 X Offset");
def_mmio!(0x0400_001A = BG20YOFS_MAIN: VolAddress<u16, (), Safe>; ["arm9"]; "Main Background 2 Y Offset");
def_mmio!(0x0400_101A = BG20YOFS_SUB: VolAddress<u16, (), Safe>; ["arm9"]; "Sub Background 2 Y Offset");
def_mmio!(0x0400_001C = BG30XOFS_MAIN: VolAddress<u16, (), Safe>; ["arm9"]; "Main Background 3 X Offset");
def_mmio!(0x0400_101C = BG30XOFS_SUB: VolAddress<u16, (), Safe>; ["arm9"]; "Sub Background 3 X Offset");
def_mmio!(0x0400_001E = BG30YOFS_MAIN: VolAddress<u16, (), Safe>; ["arm9"]; "Main Background 3 Y Offset");
def_mmio!(0x0400_101E = BG30YOFS_SUB: VolAddress<u16, (), Safe>; ["arm9"]; "Sub Background 3 Y Offset");

// arm9 and arm7 have their own separate DISPSTATS
def_mmio!(0x0400_0004 = DISPSTAT: VolAddress<u16, Safe, Safe>; ["arm9", "arm7"]; "Display Status");
def_mmio!(0x0400_0006 = VCOUNT: VolAddress<u16, Safe, Safe>; ["arm9", "arm7"]; "Vertical Counter");

// https://www.problemkaputt.de/gbatek.htm#dsvideocaptureandmainmemorydisplaymode
// #[cfg(feature = "arm9")]
pub const DISPCAPCNT: usize = 0x04000064;
// #[cfg(feature = "arm9")]
pub const DISP_MMEM_FIFO: usize = 0x04000068;

pub const BG_PALETTE_RAM_BASE_MAIN: usize = 0x05000000;
pub const OBJ_PALETTE_RAM_BASE_MAIN: usize = 0x05000200;
pub const BG_PALETTE_RAM_BASE_SUB: usize = 0x05000400;
pub const OBJ_PALETTE_RAM_BASE_SUB: usize = 0x05000600;
pub const BG_RAM_BASE_MAIN: usize = 0x06000000;
pub const BG_RAM_BASE_SUB: usize = 0x06200000;
pub const OBJ_RAM_BASE_MAIN: usize = 0x06400000;
pub const OBJ_RAM_BASE_SUB: usize = 0x06600000;
pub const OAM_BASE_MAIN: usize = 0x07000000;
pub const OAM_BASE_SUB: usize = 0x07000400;

// https://www.problemkaputt.de/gbatek.htm#dsdmatransfers
pub const DMA0SAD: usize = 0x040000B0;
pub const DMA1SAD: usize = 0x040000BC;
pub const DMA2SAD: usize = 0x040000C8;
pub const DMA3SAD: usize = 0x040000D4;
pub const DMA0DAD: usize = 0x040000B4;
pub const DMA1DAD: usize = 0x040000C0;
pub const DMA2DAD: usize = 0x040000CC;
pub const DMA3DAD: usize = 0x040000D8;
pub const DMA0CNT_L: usize = 0x040000B8;
pub const DMA1CNT_L: usize = 0x040000C4;
pub const DMA2CNT_L: usize = 0x040000D0;
pub const DMA3CNT_L: usize = 0x040000DC;
pub const DMA0CNT_H: usize = 0x040000BA;
pub const DMA1CNT_H: usize = 0x040000C6;
pub const DMA2CNT_H: usize = 0x040000D2;
pub const DMA3CNT_H: usize = 0x040000DE;
// #[cfg(feature = "arm9")]
pub const DMA0FILL: usize = 0x040000E0;
// #[cfg(feature = "arm9")]
pub const DMA1FILL: usize = 0x040000E4;
// #[cfg(feature = "arm9")]
pub const DMA2FILL: usize = 0x040000E8;
// #[cfg(feature = "arm9")]
pub const DMA3FILL: usize = 0x040000EC;

// https://www.problemkaputt.de/gbatek.htm#dstimers
pub const TM0CNT_L: usize = 0x04000100;
pub const TM1CNT_L: usize = 0x04000104;
pub const TM2CNT_L: usize = 0x04000108;
pub const TM3CNT_L: usize = 0x0400010C;
pub const TM0CNT_H: usize = 0x04000102;
pub const TM1CNT_H: usize = 0x04000106;
pub const TM2CNT_H: usize = 0x0400010A;
pub const TM3CNT_H: usize = 0x0400010E;

// https://www.problemkaputt.de/gbatek.htm#dsinterrupts
pub const IME: usize = 0x04000208;
pub const IE: usize = 0x04000210;
pub const IF: usize = 0x04000214;
//pub const IE2: usize = 0x04000218; // DSi7 only
//pub const IF2: usize = 0x0400021C;
// todo: other interrupt addresses (some are DTCM relative)

// https://www.problemkaputt.de/gbatek.htm#dsmaths
// #[cfg(feature = "arm9")]
pub const DIVCNT: usize = 0x04000280;
// #[cfg(feature = "arm9")]
pub const DIV_NUMER: usize = 0x04000290;
// #[cfg(feature = "arm9")]
pub const DIV_DENOM: usize = 0x04000298;
// #[cfg(feature = "arm9")]
pub const DIV_RESULT: usize = 0x040002A0;
// #[cfg(feature = "arm9")]
pub const DIVREM_RESULT: usize = 0x040002A8;
// #[cfg(feature = "arm9")]
pub const SQRTCNT: usize = 0x040002B0;
// #[cfg(feature = "arm9")]
pub const SQRT_RESULT: usize = 0x040002B4;
// #[cfg(feature = "arm9")]
pub const SQRT_PARAM: usize = 0x040002B8;

// https://www.problemkaputt.de/gbatek.htm#dsinterprocesscommunicationipc
pub const IPCSYNC: usize = 0x04000180;
pub const IPCFIFOCNT: usize = 0x04000184;
pub const IPCFIFOSEND: usize = 0x04000188;
pub const IPCFIFORECV: usize = 0x04100000;

// https://www.problemkaputt.de/gbatek.htm#dskeypad
def_mmio!(0x0400_0130 = KEYINPUT: VolAddress<u16, Safe, ()>; ["arm9", "arm7"]; "Key Input");
def_mmio!(0x0400_0132 = KEYCNT: VolAddress<u16, Safe, Safe>; ["arm9", "arm7"]; "Key Interrupt Control");
def_mmio!(0x0400_0136 = EXTKEYIN: VolAddress<u16, Safe, ()>; ["arm7"]; "Extra Key Input");

// https://www.problemkaputt.de/gbatek.htm#dsserialperipheralinterfacebusspi
// #[cfg(feature = "arm7")]
// pub const SPICNT: usize = 0x040001C0;
// #[cfg(feature = "arm7")]
// pub const SPIDATA: usize = 0x040001C2;

// https://www.problemkaputt.de/gbatek.htm#dspowercontrol
// #[cfg(feature = "arm9")]
pub const POWCNT1: usize = 0x04000304;
// #[cfg(feature = "arm7")]
// pub const POWCNT2: usize = 0x04000304;
// #[cfg(feature = "arm7")]
// pub const WIFIWAITCNT: usize = 0x04000206;
// #[cfg(feature = "arm7")]
// pub const HALTCNT: usize = 0x04000301;
pub const POSTFLG: usize = 0x04000300;

// https://www.problemkaputt.de/gbatek.htm#dsdebugregistersemulatordevkits
def_mmio!(0x04FF_FA00 = NOCASH_EMUID: VolBlock<u8, Safe, (), 16>; ["arm9", "arm7"]; "Nocash Emulator ID");
def_mmio!(0x04FF_FA10 = NOCASH_STROUT_RAW: VolAddress<u32, (), Safe>; ["arm9", "arm7"]; "Nocash String Out (raw)");
def_mmio!(0x04FF_FA14 = NOCASH_STROUT_PARAM: VolAddress<u32, (), Safe>; ["arm9", "arm7"]; "Nocash String Out (with %params)");
def_mmio!(0x04FF_FA18 = NOCASH_STROUT_PARAM_LF: VolAddress<u32, (), Safe>; ["arm9", "arm7"]; "Nocash String Out (with %params and linefeed)");
// this reg is really 8 bit in no$gba, but melonds won't accept it unless it's treated as 32 bit
def_mmio!(0x04FF_FA1C = NOCASH_CHAROUT: VolAddress<u32, (), Safe>; ["arm9", "arm7"]; "Nocash Character Out");
def_mmio!(0x04FF_FA20 = NOCASH_CLOCKS: VolBlock<u32, Safe, (), 2>; ["arm9", "arm7"]; "Nocash Clock Cycles");
