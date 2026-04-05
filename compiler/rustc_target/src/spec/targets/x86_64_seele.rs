use crate::spec::{
    Arch, Cc, CodeModel, Env, LinkerFlavor, Lld, Os, PanicStrategy, RelroLevel, SanitizerSet,
    StackProbeType, Target, TargetMetadata, TargetOptions,
};

pub(crate) fn target() -> Target {
    let mut opts = TargetOptions {
        os: Os::Other("seele".into()),
        env: Env::Relibc,
        families: crate::spec::cvs!["unix"],
        // Cc::Yes means use the compiler as the linker instead of the actual linker
        // Uses our custom clang as the linker so it can recognize
        // the seele target and beable to link the crt and libc.
        linker_flavor: LinkerFlavor::Gnu(Cc::Yes, Lld::No),
        linker: Some("clang".into()),
        dynamic_linking: true,
        has_rpath: true,
        plt_by_default: false,
        max_atomic_width: Some(64),
        stack_probes: StackProbeType::Inline,
        position_independent_executables: true,
        static_position_independent_executables: false,
        crt_static_default: false,
        crt_static_respected: false,
        relro_level: RelroLevel::Full,
        features: "-mmx,-avx,-avx2".into(),
        supported_sanitizers: SanitizerSet::KCFI | SanitizerSet::KERNELADDRESS,
        disable_redzone: true,
        panic_strategy: PanicStrategy::Abort,
        // Userland code, not the kernel itself.
        code_model: Some(CodeModel::Small),
        ..Default::default()
    };

    opts.add_pre_link_args(
        LinkerFlavor::Gnu(Cc::Yes, Lld::No),
        &["--target=x86_64-unknown-seele", "-m64"],
    );

    Target {
        llvm_target: "x86_64-unknown-seele".into(),
        metadata: TargetMetadata {
            description: Some("Freestanding/bare-metal x86_64 softfloat".into()),
            tier: Some(2),
            host_tools: Some(false),
            std: Some(true),
        },
        pointer_width: 64,
        data_layout:
            "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128".into(),
        arch: Arch::X86_64,
        options: opts,
    }
}
