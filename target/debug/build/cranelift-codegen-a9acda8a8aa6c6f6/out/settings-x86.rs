#[derive(Clone)]
/// Flags group `x86`.
pub struct Flags {
    bytes: [u8; 3],
}
impl Flags {
    /// Create flags x86 settings group.
    #[allow(unused_variables)]
    pub fn new(shared: &settings::Flags, builder: Builder) -> Self {
        let bvec = builder.state_for("x86");
        let mut x86 = Self { bytes: [0; 3] };
        debug_assert_eq!(bvec.len(), 2);
        x86.bytes[0..2].copy_from_slice(&bvec);
        // Precompute #9.
        if shared.allones_funcaddrs() && !(shared.is_pic()) {
            x86.bytes[1] |= 1 << 1;
        }
        // Precompute #10.
        if shared.is_pic() {
            x86.bytes[1] |= 1 << 2;
        }
        // Precompute #11.
        if !(shared.allones_funcaddrs()) && !(shared.is_pic()) {
            x86.bytes[1] |= 1 << 3;
        }
        // Precompute #12.
        if !(shared.is_pic()) {
            x86.bytes[1] |= 1 << 4;
        }
        // Precompute #13.
        if x86.has_bmi1() {
            x86.bytes[1] |= 1 << 5;
        }
        // Precompute #14.
        if x86.has_lzcnt() {
            x86.bytes[1] |= 1 << 6;
        }
        // Precompute #15.
        if x86.has_popcnt() && x86.has_sse42() {
            x86.bytes[1] |= 1 << 7;
        }
        // Precompute #16.
        if x86.has_sse41() {
            x86.bytes[2] |= 1 << 0;
        }
        // Precompute #17.
        if shared.enable_simd() && x86.has_sse41() {
            x86.bytes[2] |= 1 << 1;
        }
        // Precompute #18.
        if x86.has_sse41() && x86.has_sse42() {
            x86.bytes[2] |= 1 << 2;
        }
        // Precompute #19.
        if shared.enable_simd() && x86.has_sse41() && x86.has_sse42() {
            x86.bytes[2] |= 1 << 3;
        }
        // Precompute #20.
        if x86.has_ssse3() {
            x86.bytes[2] |= 1 << 4;
        }
        // Precompute #21.
        if shared.enable_simd() && x86.has_ssse3() {
            x86.bytes[2] |= 1 << 5;
        }
        x86
    }
}
/// User-defined settings.
#[allow(dead_code)]
impl Flags {
    /// Get a view of the boolean predicates.
    pub fn predicate_view(&self) -> crate::settings::PredicateView {
        crate::settings::PredicateView::new(&self.bytes[0..])
    }
    /// Dynamic numbered predicate getter.
    fn numbered_predicate(&self, p: usize) -> bool {
        self.bytes[0 + p / 8] & (1 << (p % 8)) != 0
    }
    /// SSE3: CPUID.01H:ECX.SSE3[bit 0]
    pub fn has_sse3(&self) -> bool {
        self.numbered_predicate(0)
    }
    /// SSSE3: CPUID.01H:ECX.SSSE3[bit 9]
    pub fn has_ssse3(&self) -> bool {
        self.numbered_predicate(1)
    }
    /// SSE4.1: CPUID.01H:ECX.SSE4_1[bit 19]
    pub fn has_sse41(&self) -> bool {
        self.numbered_predicate(2)
    }
    /// SSE4.2: CPUID.01H:ECX.SSE4_2[bit 20]
    pub fn has_sse42(&self) -> bool {
        self.numbered_predicate(3)
    }
    /// POPCNT: CPUID.01H:ECX.POPCNT[bit 23]
    pub fn has_popcnt(&self) -> bool {
        self.numbered_predicate(4)
    }
    /// AVX: CPUID.01H:ECX.AVX[bit 28]
    pub fn has_avx(&self) -> bool {
        self.numbered_predicate(5)
    }
    /// BMI1: CPUID.(EAX=07H, ECX=0H):EBX.BMI1[bit 3]
    pub fn has_bmi1(&self) -> bool {
        self.numbered_predicate(6)
    }
    /// BMI2: CPUID.(EAX=07H, ECX=0H):EBX.BMI2[bit 8]
    pub fn has_bmi2(&self) -> bool {
        self.numbered_predicate(7)
    }
    /// LZCNT: CPUID.EAX=80000001H:ECX.LZCNT[bit 5]
    pub fn has_lzcnt(&self) -> bool {
        self.numbered_predicate(8)
    }
    /// Computed predicate `shared.allones_funcaddrs() && !(shared.is_pic())`.
    pub fn all_ones_funcaddrs_and_not_is_pic(&self) -> bool {
        self.numbered_predicate(9)
    }
    /// Computed predicate `shared.is_pic()`.
    pub fn is_pic(&self) -> bool {
        self.numbered_predicate(10)
    }
    /// Computed predicate `!(shared.allones_funcaddrs()) && !(shared.is_pic())`.
    pub fn not_all_ones_funcaddrs_and_not_is_pic(&self) -> bool {
        self.numbered_predicate(11)
    }
    /// Computed predicate `!(shared.is_pic())`.
    pub fn not_is_pic(&self) -> bool {
        self.numbered_predicate(12)
    }
    /// Computed predicate `x86.has_bmi1()`.
    pub fn use_bmi1(&self) -> bool {
        self.numbered_predicate(13)
    }
    /// Computed predicate `x86.has_lzcnt()`.
    pub fn use_lzcnt(&self) -> bool {
        self.numbered_predicate(14)
    }
    /// Computed predicate `x86.has_popcnt() && x86.has_sse42()`.
    pub fn use_popcnt(&self) -> bool {
        self.numbered_predicate(15)
    }
    /// Computed predicate `x86.has_sse41()`.
    pub fn use_sse41(&self) -> bool {
        self.numbered_predicate(16)
    }
    /// Computed predicate `shared.enable_simd() && x86.has_sse41()`.
    pub fn use_sse41_simd(&self) -> bool {
        self.numbered_predicate(17)
    }
    /// Computed predicate `x86.has_sse41() && x86.has_sse42()`.
    pub fn use_sse42(&self) -> bool {
        self.numbered_predicate(18)
    }
    /// Computed predicate `shared.enable_simd() && x86.has_sse41() && x86.has_sse42()`.
    pub fn use_sse42_simd(&self) -> bool {
        self.numbered_predicate(19)
    }
    /// Computed predicate `x86.has_ssse3()`.
    pub fn use_ssse3(&self) -> bool {
        self.numbered_predicate(20)
    }
    /// Computed predicate `shared.enable_simd() && x86.has_ssse3()`.
    pub fn use_ssse3_simd(&self) -> bool {
        self.numbered_predicate(21)
    }
}
static DESCRIPTORS: [detail::Descriptor; 17] = [
    detail::Descriptor {
        name: "has_sse3",
        offset: 0,
        detail: detail::Detail::Bool { bit: 0 },
    },
    detail::Descriptor {
        name: "has_ssse3",
        offset: 0,
        detail: detail::Detail::Bool { bit: 1 },
    },
    detail::Descriptor {
        name: "has_sse41",
        offset: 0,
        detail: detail::Detail::Bool { bit: 2 },
    },
    detail::Descriptor {
        name: "has_sse42",
        offset: 0,
        detail: detail::Detail::Bool { bit: 3 },
    },
    detail::Descriptor {
        name: "has_popcnt",
        offset: 0,
        detail: detail::Detail::Bool { bit: 4 },
    },
    detail::Descriptor {
        name: "has_avx",
        offset: 0,
        detail: detail::Detail::Bool { bit: 5 },
    },
    detail::Descriptor {
        name: "has_bmi1",
        offset: 0,
        detail: detail::Detail::Bool { bit: 6 },
    },
    detail::Descriptor {
        name: "has_bmi2",
        offset: 0,
        detail: detail::Detail::Bool { bit: 7 },
    },
    detail::Descriptor {
        name: "has_lzcnt",
        offset: 1,
        detail: detail::Detail::Bool { bit: 0 },
    },
    detail::Descriptor {
        name: "baseline",
        offset: 0,
        detail: detail::Detail::Preset,
    },
    detail::Descriptor {
        name: "nehalem",
        offset: 2,
        detail: detail::Detail::Preset,
    },
    detail::Descriptor {
        name: "haswell",
        offset: 4,
        detail: detail::Detail::Preset,
    },
    detail::Descriptor {
        name: "broadwell",
        offset: 6,
        detail: detail::Detail::Preset,
    },
    detail::Descriptor {
        name: "skylake",
        offset: 8,
        detail: detail::Detail::Preset,
    },
    detail::Descriptor {
        name: "cannonlake",
        offset: 10,
        detail: detail::Detail::Preset,
    },
    detail::Descriptor {
        name: "icelake",
        offset: 12,
        detail: detail::Detail::Preset,
    },
    detail::Descriptor {
        name: "znver1",
        offset: 14,
        detail: detail::Detail::Preset,
    },
];
static ENUMERATORS: [&str; 0] = [
];
static HASH_TABLE: [u16; 32] = [
    0xffff,
    0xffff,
    0xffff,
    0,
    16,
    0xffff,
    12,
    0xffff,
    0xffff,
    0xffff,
    10,
    0xffff,
    0xffff,
    3,
    0xffff,
    7,
    2,
    6,
    1,
    11,
    0xffff,
    0xffff,
    5,
    4,
    14,
    0xffff,
    15,
    13,
    0xffff,
    0xffff,
    8,
    9,
];
static PRESETS: [(u8, u8); 16] = [
    // baseline
    (0b00000000, 0b00000000),
    (0b00000000, 0b00000000),
    // nehalem
    (0b00011111, 0b00011111),
    (0b00000000, 0b00000000),
    // haswell
    (0b11011111, 0b11011111),
    (0b00000001, 0b00000001),
    // broadwell
    (0b11011111, 0b11011111),
    (0b00000001, 0b00000001),
    // skylake
    (0b11011111, 0b11011111),
    (0b00000001, 0b00000001),
    // cannonlake
    (0b11011111, 0b11011111),
    (0b00000001, 0b00000001),
    // icelake
    (0b11011111, 0b11011111),
    (0b00000001, 0b00000001),
    // znver1
    (0b11011111, 0b11011111),
    (0b00000001, 0b00000001),
];
static TEMPLATE: detail::Template = detail::Template {
    name: "x86",
    descriptors: &DESCRIPTORS,
    enumerators: &ENUMERATORS,
    hash_table: &HASH_TABLE,
    defaults: &[0x00, 0x00],
    presets: &PRESETS,
};
/// Create a `settings::Builder` for the x86 settings group.
pub fn builder() -> Builder {
    Builder::new(&TEMPLATE)
}
impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[x86]")?;
        for d in &DESCRIPTORS {
            if !d.detail.is_preset() {
                write!(f, "{} = ", d.name)?;
                TEMPLATE.format_toml_value(d.detail, self.bytes[d.offset as usize], f)?;
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
