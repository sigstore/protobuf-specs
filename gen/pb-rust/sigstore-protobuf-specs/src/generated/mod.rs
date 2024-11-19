pub mod dev {
    pub mod sigstore {
        pub mod bundle {
            pub mod v1 {
                include!("dev.sigstore.bundle.v1.rs");
            }
        }
        pub mod common {
            pub mod v1 {
                include!("dev.sigstore.common.v1.rs");
            }
        }
        pub mod events {
            pub mod v1 {
                include!("dev.sigstore.events.v1.rs");
            }
        }
        pub mod rekor {
            pub mod v1 {
                include!("dev.sigstore.rekor.v1.rs");
            }
        }
        pub mod signing {
            pub mod v1 {
                include!("dev.sigstore.signing.v1.rs");
            }
        }
        pub mod trustroot {
            pub mod v1 {
                include!("dev.sigstore.trustroot.v1.rs");
            }
        }
        pub mod verification {
            pub mod v1 {
                include!("dev.sigstore.verification.v1.rs");
            }
        }
    }
}
pub mod google {
    pub mod api {
        include!("google.api.rs");
    }
}
pub mod io {
    pub mod intoto {
        include!("io.intoto.rs");
    }
}
