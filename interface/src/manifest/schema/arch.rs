use super::{ArchManifest, Manifest};

macro_rules! merge {
    ($m:ident, $arch:ident, $prop:ident) => {
        if $arch.$prop.is_none() {
            $arch.$prop = $m.$prop;
        }
    };
}

pub enum Architecture {
    X86,
    Amd64,
    Arm64,
    None,
}

impl Manifest {
    /// Get the part of manifest that changes depending on the architecture.
    /// The difference between just getting `architectures` field is that this methos also collects
    /// field from top-level architecture-dependent fields.
    /// That means, even there is no architecture-specific field, it will still return the top-level field.
    pub fn architecture(&self, arch: Architecture) -> ArchManifest {
        let m = self.clone();
        let mut arch_manifest = if let Some(architecture) = m.architecture {
            match arch {
                Architecture::X86 => architecture.the_32_bit,
                Architecture::Amd64 => architecture.the_64_bit,
                Architecture::Arm64 => architecture.arm64,
                Architecture::None => Some(ArchManifest::default()),
            }
        } else {
            None
        }
        .unwrap_or_default();

        merge!(m, arch_manifest, bin);
        merge!(m, arch_manifest, checkver);
        merge!(m, arch_manifest, env_add_path);
        merge!(m, arch_manifest, env_set);
        merge!(m, arch_manifest, extract_dir);
        merge!(m, arch_manifest, hash);
        merge!(m, arch_manifest, installer);
        merge!(m, arch_manifest, msi);
        merge!(m, arch_manifest, post_install);
        merge!(m, arch_manifest, post_uninstall);
        merge!(m, arch_manifest, pre_install);
        merge!(m, arch_manifest, pre_uninstall);
        merge!(m, arch_manifest, shortcuts);
        merge!(m, arch_manifest, uninstaller);
        merge!(m, arch_manifest, url);

        arch_manifest
    }
}
