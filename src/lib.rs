anchor_gen::generate_cpi_crate!("idl.json");

anchor_lang::declare_id!("JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB");

pub mod jupiter_override {
    use super::Side;
    use super::SplitLeg;
    use anchor_lang::prelude::*;
    use anchor_lang::Discriminator;
    use anchor_lang::{AnchorSerialize, InstructionData};
    use std::io::Write;

    #[derive(AnchorSerialize)]
    pub enum Swap {
        Saber,
        SaberAddDecimalsDeposit,
        SaberAddDecimalsWithdraw,
        TokenSwap,
        Sencha,
        Step,
        Cropper,
        Raydium,
        Crema {
            a_to_b: bool,
        },
        Lifinity,
        Mercurial,
        Cykura,
        Serum {
            side: Side,
        },
        MarinadeDeposit,
        MarinadeUnstake,
        Aldrin {
            side: Side,
        },
        AldrinV2 {
            side: Side,
        },
        Whirlpool {
            a_to_b: bool,
        },
        Invariant {
            x_to_y: bool,
        },
        Meteora,
        GooseFX,
        DeltaFi {
            stable: bool,
        },
        Balansol,
        MarcoPolo {
            x_to_y: bool,
        },
        Dradex {
            side: Side,
        },
        LifinityV2,
        RaydiumClmm,
        Openbook {
            side: Side,
        },
        Phoenix {
            side: Side,
        },
        Symmetry {
            from_token_id: u64,
            to_token_id: u64,
        },
    }

    pub enum SwapLeg {
        Chain { swap_legs: Vec<SwapLeg> },
        Split { split_legs: Vec<SplitLeg> },
        Swap { swap: Swap },
    }

    impl AnchorSerialize for SwapLeg {
        #[inline]
        fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
            match self {
                SwapLeg::Chain { swap_legs } => {
                    0u8.serialize(writer)?;
                    swap_legs.serialize(writer)
                }
                SwapLeg::Split { split_legs } => {
                    1u8.serialize(writer)?;
                    split_legs.serialize(writer)
                }
                SwapLeg::Swap { swap } => {
                    2u8.serialize(writer)?;
                    swap.serialize(writer)
                }
            }
        }
    }

    #[derive(AnchorSerialize)]
    pub struct Route {
        pub swap_leg: SwapLeg,
        pub in_amount: u64,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }

    impl Discriminator for Route {
        const DISCRIMINATOR: [u8; 8] = [229, 23, 203, 151, 122, 227, 173, 42];
    }

    impl InstructionData for Route {}
}
