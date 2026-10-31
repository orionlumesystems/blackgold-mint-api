use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum MintInstruction {
    //* Instruction to be executed: 0 - Initialiser
    Init = 0,

    //* Instruction to be executed: 1 - Mint BLKG Token
    MintToken = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Init {
    //* Empty struct
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct MintToken {
    //* No of tokens to mint in this request (max 2 allowed per request as per the architecture)
    pub amount: [u8; 8],
}

instruction!(MintInstruction, Init);
instruction!(MintInstruction, MintToken);
