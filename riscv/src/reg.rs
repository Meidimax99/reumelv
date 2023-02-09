/// A individual bit in a register, `RegisterEntry` contains the position and the values of the bit
/// # Example
///
/// ```
/// let register entry = (0, true);
/// ```
///
pub type RegisterEntry = (usize, bool);
type RegEnt = RegisterEntry;

/// mstatus: machine status
///
///     `mpp`: sets Previous Privilege Mode to User-mode so modules run only in U-mode after setup
pub const MSTATUS_MPP_U: (RegEnt, RegEnt) = ((11, false), (12, false));
///     `mie`: machine-mode interrupt enable
pub const MSTATUS_MIE: RegEnt = (3, true);

//  mie
///     `meie`: external machine-mode interrupt enable
pub const MIE_MEIE: RegEnt = (11, true);
///     `mtie`: timer machine-mode interrupt enable
pub const MIE_MTIE: RegEnt = (7, true);
///     `msie`: software machine-mode interrupt enable
pub const MIE_MSIE: RegEnt = (3, true);

//  sie
///     `seie`: external supervisor-mode interrupt enable
pub const SIE_SEIE: RegEnt = (9, true);
///     `stie`: timer supervisor-mode interrupt enable
pub const SIE_STIE: RegEnt = (5, true);
///     `ssie`: software supervisor-mode interrupt enable
pub const SIE_SSIE: RegEnt = (1, true);

//  clint - Core Local Interrupt
///     `mtimecmp_addr`: Address of the Compare Value for the Core Local Interrupt (clint), triggers timer interrupt **!! In QEMU at 0x0200 on real hardware at 0x2000**
pub const MTIMECMP_ADDR: usize = 0x0200_4000;

///     `mtime`: 64bit register of the timer incremented every clock-cycle e.g. 10.000.000 times on QEMU with 10Mhz
pub const MTIME_ADDR: usize = 0x0200_BFF8;

//  plic - Platform-Level Interrupt Controller

///base-address for QEMU(??for other systems as well??)
///
/// [More Info](https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc#memory-map)
pub const PLIC_MEMORY_MAP_BASE: usize = 0x0c00_0000;

///base address for the interrupt priorities, starts at `PLIC_MEMORY_MAP_BASE + 0x0000_0000`, consisting of 32-bit registers.
/// Priorities are unsigned u32, 0 means "never interrupt", max priority is platform specific. Note that *0x0000_0000* does not have an interrupt source since interrupt 0 does not exist
///
/// [More Info](https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc#interrupt-priorities)
pub const PRIORITY_BASE_ADDR: usize = 0x0c00_0000;

///base address for enabling interrupt sources, starts at `PLIC_MEMORY_MAP_BASE + 0x0000_2000`. 1-bit for enable of interrupt source with ID = bit position. Continuos block for 0-1023 sources for
/// 15872 contexts
///
/// [More Info](https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc#interrupt-enables)
pub const ENABLE_ADDR: usize = 0x0c00_2000;

///base address for setting of a interrupt priority threshold, starts at `PLIC_MEMORY_MAP_BASE + 0x0020_0000`, incremented by 0x1000 for each context. PLIC ignorers all interrupts with priority less than or equal to
/// the given threshold, set individually for all 15872 contexts.
///
/// [More Info](https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc#priority-thresholds)
pub const THRESHOLD_ADDR_C0: usize = 0x0c20_0000;

///base address for the interrupt claim and completion registers, starts at `PLIC_MEMORY_MAP_BASE + 0x0020_0004`, incremented by 0x1000 for each context. If interrupt is handled by service after
/// receiving an interrupt notification, the Interrupt must be claimed from the PLIC. PLIC returns Interrupt ID to Service, if no interrupt is pending returns 0.
///
/// [More Info](https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc#interrupt-claim-process)
pub const CLAIM_COMP_ADDR_C0: usize = 0x0c20_0004;

// MStatus, // Machine Status
// MEPC,    // 'machine exception program counter' holds the 'return from exception' address.
// SATP,    // 'supervisor address translation and protection' holds the 'page table' address.
// MIE,     // 'machine interrupt enable'
// SIE,     // 'supervisor interrupt enable'
// MTVec,   // 'machine-mode interrupt vector'
// PmpCfg0,
// PmpAddr0,
