use crate::spec::{
    crt_objects, Arch, Cc, CodeModel, Env, LinkSelfContainedDefault, LinkerFlavor, Lld, Os,
    PanicStrategy, RelroLevel, RustcAbi, SanitizerSet, StackProbeType, Target, TargetMetadata,
    TargetOptions,
};

pub(crate) fn target() -> Target {
    let opts = TargetOptions {
        os: Os::Other("seele".into()),
        env: Env::Relibc,
        families: crate::spec::cvs!["unix"],
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        linker: Some("rust-lld".into()),
        plt_by_default: false,
        max_atomic_width: Some(64),
        stack_probes: StackProbeType::Inline,
        position_independent_executables: false,
        static_position_independent_executables: false,
        relro_level: RelroLevel::Full,
        rustc_abi: Some(RustcAbi::Softfloat),
        features: "-mmx,-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-avx,-avx2,+soft-float".into(),
        supported_sanitizers: SanitizerSet::KCFI | SanitizerSet::KERNELADDRESS,
        disable_redzone: true,
        panic_strategy: PanicStrategy::Abort,
        // Userland code, not the kernel itself.
        code_model: Some(CodeModel::Small),
        // Always link against relibc's CRT objects for this target.
        //
        // 之前我们把它们挂在 `pre_link_objects_self_contained` 上，
        // 依赖 `link_self_contained` / self-contained 组件机制，
        // 结果在实际链接里没有稳定地把 `crt0.o` 拉进来，最后 ELF 的
        // `e_entry` 还是 0。
        //
        // 为了简单粗暴、可预期，直接用普通的 `pre_link_objects` /
        // `post_link_objects` 路径，不再依赖 self-contained 组件。
        pre_link_objects: crt_objects::pre_seele_self_contained(),
        post_link_objects: crt_objects::post_seele_self_contained(),
        link_self_contained: LinkSelfContainedDefault::False,
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
