pub mod io {
    pub mod intoto {
        include!("codegen/io.intoto.rs");
    }
}
pub mod dev {
    pub mod sigstore {
        pub mod common {
            pub mod v1 {
                include!("codegen/dev.sigstore.common.v1.rs");
            }
        }
        pub mod rekor {
            pub mod v1 {
                include!("codegen/dev.sigstore.rekor.v1.rs");
            }
        }
        pub mod bundle {
            pub mod v1 {
                include!("codegen/dev.sigstore.bundle.v1.rs");
            }
        }
        pub mod trustroot {
            pub mod v1 {
                include!("codegen/dev.sigstore.trustroot.v1.rs");
            }
        }
        pub mod verification {
            pub mod v1 {
                include!("codegen/dev.sigstore.verification.v1.rs");
            }
        }
    }
}
