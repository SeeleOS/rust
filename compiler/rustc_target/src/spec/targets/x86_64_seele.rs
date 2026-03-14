use crate::spec::{
    Arch, Cc, CodeModel, LinkerFlavor, Lld, Os, PanicStrategy, RelroLevel, RustcAbi, SanitizerSet,
    StackProbeType, Target, TargetMetadata, TargetOptions,
};

pub(crate) fn target() -> Target {
    let opts = TargetOptions {
        os: Os::Other("seele".into()),
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        linker: Some("rust-lld".into()),
        plt_by_default: false,
        max_atomic_width: Some(64),
        stack_probes: StackProbeType::Inline,
        position_independent_executables: true,
        static_position_independent_executables: true,
        relro_level: RelroLevel::Full,
        rustc_abi: Some(RustcAbi::Softfloat),
        features: "-mmx,-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-avx,-avx2,+soft-float".into(),
        supported_sanitizers: SanitizerSet::KCFI | SanitizerSet::KERNELADDRESS,
        disable_redzone: true,
        panic_strategy: PanicStrategy::Abort,
        code_model: Some(CodeModel::Kernel),
        ..Default::default()
    };

    Target {
        llvm_target: "x86_64-unknown-none-elf".into(),
        metadata: TargetMetadata {
            description: Some("Freestanding/bare-metal x86_64 softfloat".into()),
            tier: Some(2),
            host_tools: Some(false),
            std: Some(false),
        },
        pointer_width: 64,
        data_layout:
            "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128".into(),
        arch: Arch::X86_64,
        options: opts,
    }
}
