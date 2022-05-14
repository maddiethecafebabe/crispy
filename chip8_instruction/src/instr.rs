#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    SysJmp(u16),
    ClearScreen,
    Return,
    Jump(u16),
    Call(u16),

    /// Skip next instruction if equal immidiate
    /// (x, kk)
    /// 
    /// if V[x] == kk {
    ///     pc += 2;
    /// }
    SkipIfEqualImmidiate(u8, u8),


    /// Skip next instruction if not equal immidiate
    /// (x, kk)
    /// 
    /// if V[x] != kk {
    ///     pc += 2;instr!(0xff11 => <none>);
    SkipIfNotEqualImmidiate(u8, u8),

    /// Skip next instruction if equal register
    /// (x, y)
    /// 
    /// if V[x] == V[y] {
    ///     pc += 2;
    /// }
    SkipIfEqualRegister(u8, u8),

    LoadImmidiate(u8, u8),

    AddImmidiate(u8, u8),

    LoadRegister(u8, u8),

    OrRegister(u8, u8),
    AndRegister(u8, u8),
    XorRegister(u8, u8),
    AddRegister(u8, u8),
    SubRegister(u8, u8),
    ShrRegister(u8, u8),
    SubnRegister(u8, u8),
    ShlRegister(u8, u8),

    /// Skip next instruction if not equal register
    /// (x, y)
    /// 
    /// if V[x] != V[y] {
    ///     pc += 2;
    /// }
    SkipIfNotEqualRegister(u8, u8),

    LoadI(u16),

    JumpV0(u16),

    Random(u8, u8),

    DisplaySprite(u8, u8, u8),

    SkipIfPressed(u8),

    SkipIfNotPressed(u8),

    LoadDelayTimer(u8),

    ReadKey(u8),

    SetDelayTimer(u8),

    SetSoundTimer(u8),

    AddI(u8),

    LoadSpriteLocationI(u8),

    StoreDecimalI(u8),

    RegDumpI(u8),

    RegLoadI(u8)
}
