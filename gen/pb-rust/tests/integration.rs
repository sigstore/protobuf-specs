use pretty_assertions::{assert_eq, assert_ne};
use serde_json;
use sigstore_protobuf_specs::{
    Bundle, DevSigstoreBundleV1VerificationMaterial as VerificationMaterial,
    DevSigstoreCommonV1HashOutput as HashOutput, DevSigstoreCommonV1LogId as LogId,
    DevSigstoreCommonV1MessageSignature as MessageSignature,
    DevSigstoreCommonV1X509Certificate as X509Certificate,
    DevSigstoreCommonV1X509CertificateChain as X509CertificateChain,
    DevSigstoreRekorV1Checkpoint as Checkpoint,
    DevSigstoreRekorV1InclusionPromise as InclusionPromise,
    DevSigstoreRekorV1InclusionProof as InclusionProof,
    DevSigstoreRekorV1KindVersion as KindVersion,
    DevSigstoreRekorV1TransparencyLogEntry as TransparencyLogEntry,
};

#[test]
fn bundle() {
    let bundle_json = std::fs::read_to_string("tests/bundle.txt.sigstore").unwrap();
    let bundle = serde_json::from_str::<Bundle>(&bundle_json);

    assert!(bundle.is_ok());
    assert_eq!(
        bundle.unwrap(),
        Bundle {
            media_type: Some(String::from("application/vnd.dev.sigstore.bundle+json;version=0.1")),
            verification_material: Some(VerificationMaterial{
                public_key: None,
                x_509_certificate_chain: Some(X509CertificateChain{
                    certificates: Some(vec![
                        X509Certificate{
                            raw_bytes: Some(String::from("MIIC5zCCAmygAwIBAgIUJ3vpewdf6e91rgjqCqagstF4qn8wCgYIKoZIzj0EAwMwNzEVMBMGA1UEChMMc2lnc3RvcmUuZGV2MR4wHAYDVQQDExVzaWdzdG9yZS1pbnRlcm1lZGlhdGUwHhcNMjMwNDI2MDAyMTA4WhcNMjMwNDI2MDAzMTA4WjAAMHYwEAYHKoZIzj0CAQYFK4EEACIDYgAE2sd6+lOBcn5MXtnbwca7zcwpprl7GUZiKTO9IWpAUfVTtx+BXGHQCRwsFy/d7dLlf4hurIqhzMD5yaC2kcU9/8c9G55JyBXF8Dx5SQm9y2rPWFIdm29Ql9A3I3yyEFyPo4IBbjCCAWowDgYDVR0PAQH/BAQDAgeAMBMGA1UdJQQMMAoGCCsGAQUFBwMDMB0GA1UdDgQWBBTlaUfjpiXGhBP3hOCW0JJZDSPxgzAfBgNVHSMEGDAWgBRxhjCmFHxib/n31vQFGn9f/+tvrDAYBgNVHREBAf8EDjAMgQphQHRueS50b3duMCwGCisGAQQBg78wAQEEHmh0dHBzOi8vZ2l0aHViLmNvbS9sb2dpbi9vYXV0aDAuBgorBgEEAYO/MAEIBCAMHmh0dHBzOi8vZ2l0aHViLmNvbS9sb2dpbi9vYXV0aDCBigYKKwYBBAHWeQIEAgR8BHoAeAB2ACswvNxoiMni4dgmKV50H0g5MZYC8pwzy15DQP6yrIZ6AAABh7rveBsAAAQDAEcwRQIhAKOZPMN9Q9qO1HXigHBPt+Ic16yy2Zgv2KQ23i5WLj16AiAzrFpuayGXdoK+hYePl9dEeXjG/vB2jK/E3sEsIrXtETAKBggqhkjOPQQDAwNpADBmAjEAgmhg80mI/Scr0isBnD5FYXZ8WxA8tnBBPmdf4aNGForGazGXaFQVPXgBVPv+YGI/AjEA0QzPC5dHD/WWXW2GbEC4dpwFk8OGRkiExMOy/+CqabbVg+/lx1N9VGBTlUTft45d"))
                        }
                    ]),
                }),
                tlog_entries: Some(vec![
                    TransparencyLogEntry{
                        log_index: Some(String::from("7390977")),
                        log_id: Some(LogId{
                            key_id: Some(String::from("0y8wo8MtY5wrdiIFohx7sHeI5oKDpK5vQhGHI6G+pJY="))
                        }),
                        kind_version: Some(KindVersion{
                            kind: Some(String::from("hashedrekord")),
                            version: Some(String::from("0.0.1"))
                        }),
                        integrated_time: Some(String::from("1682468469")),
                        inclusion_promise: Some(InclusionPromise{
                            signed_entry_timestamp: Some(String::from("MEUCICSJs5PgN4W3Lku3ybrwfNLAKMWaOvffg2tnqm19VrWEAiEA16MVPsWDoaAljsxGefpQazpvYfs1pv8lzdgZQ0I4rH0="))
                        }),
                        inclusion_proof: Some(InclusionProof{
                            log_index: Some(String::from("7376158")),
                            root_hash: Some(String::from("LE67t2Zlc0g35az81xMg0cgM2DULj8fNsGGHTcRthcs=")),
                            tree_size: Some(String::from("7376159")),
                            hashes: Some(vec![
                                String::from("zgesNHwk09VvW4IDaPrJMtX59glNyyLPzeJO1Gw1hCI="),
                                String::from("lJiFr9ZP5FO8BjqLAUQ16A/0/LoOOQ0gfeNhdxaxO2w="),
                                String::from("sMImd51DBHQnH1tz4sGk8gXB+FjWyusVXbP0GmpFnB4="),
                                String::from("cDU1nEpl0WCRlxLi/gNVzykDzobU4qG/7BQZxn0qDgU="),
                                String::from("4CRqWzG3qpxKvlHuZg5O6QjQiwOzerbjwsAh30EVlA8="),
                                String::from("Ru0p3GE/zB2zub2/xR5rY/aM4J+5VJmiIuIl2enF/ws="),
                                String::from("2W+NG5yGR68lrLGcw4gn9CSCfeQF98d3LMfdo8tPyok="),
                                String::from("bEs1eYxy9R6hR2veGEwYW4PEdrZKrdqZ7uDlmmNtlas="),
                                String::from("sgQMnwcK7VxxAi+fygxq8iJ+zWqShjXm07/AWobWcXU="),
                                String::from("y4BESazXFcefRzxpN1PfJHoqRaKnPJPM5H/jotx0QY8="),
                                String::from("xiNEdLOpmGQERCR+DCEFVRK+Ns6G0BLV9M6sQQkRhik=")
                            ]),
                            checkpoint: Some(Checkpoint{
                                envelope: Some(String::from("rekor.sigstage.dev - 8050909264565447525\n7376159\nLE67t2Zlc0g35az81xMg0cgM2DULj8fNsGGHTcRthcs=\nTimestamp: 1682468469199678948\n\n\u{2014} rekor.sigstage.dev 0y8wozBEAiBbAodz3dBqJjGMhnZEkbaTDVxc8+tBEPKbaWUZoqxFvwIgGtYzFgFaM3UXBRHmzgmcrCxA145dpQ2YD0yFqiPHO7U=\n"))
                            }),
                        }),
                        canonicalized_body: Some(String::from("eyJhcGlWZXJzaW9uIjoiMC4wLjEiLCJraW5kIjoiaGFzaGVkcmVrb3JkIiwic3BlYyI6eyJkYXRhIjp7Imhhc2giOnsiYWxnb3JpdGhtIjoic2hhMjU2IiwidmFsdWUiOiI4MDJkZDYwZmY4ODMzMzgwMmYyNTg1ZTczMDQzYmQyMWMzNDEyODVlMTk5MmZlNWIzMTc1NWUxY2FkZWFlMzBlIn19LCJzaWduYXR1cmUiOnsiY29udGVudCI6Ik1HVUNNUUNPT0pxVFk2WFdnQjY0aXpLMldWUDA3YjBTRzlNNVdQQ3dLaGZUUHdNdnRzZ1VpOEtlUkd3UWt2dkxZYktIZHFVQ01FYk9YRkcwTk1xRVF4V1ZiNnJtR25leGRBRHVHZjZKbDhxQUM4dG42N3AzUWZWb1h6TXZGQTYxUHp4d1Z3dmI4Zz09IiwicHVibGljS2V5Ijp7ImNvbnRlbnQiOiJMUzB0TFMxQ1JVZEpUaUJEUlZKVVNVWkpRMEZVUlMwdExTMHRDazFKU1VNMWVrTkRRVzE1WjBGM1NVSkJaMGxWU2pOMmNHVjNaR1kyWlRreGNtZHFjVU54WVdkemRFWTBjVzQ0ZDBObldVbExiMXBKZW1vd1JVRjNUWGNLVG5wRlZrMUNUVWRCTVZWRlEyaE5UV015Ykc1ak0xSjJZMjFWZFZwSFZqSk5ValIzU0VGWlJGWlJVVVJGZUZaNllWZGtlbVJIT1hsYVV6RndZbTVTYkFwamJURnNXa2RzYUdSSFZYZElhR05PVFdwTmQwNUVTVEpOUkVGNVRWUkJORmRvWTA1TmFrMTNUa1JKTWsxRVFYcE5WRUUwVjJwQlFVMUlXWGRGUVZsSUNrdHZXa2w2YWpCRFFWRlpSa3MwUlVWQlEwbEVXV2RCUlRKelpEWXJiRTlDWTI0MVRWaDBibUozWTJFM2VtTjNjSEJ5YkRkSFZWcHBTMVJQT1VsWGNFRUtWV1pXVkhSNEswSllSMGhSUTFKM2MwWjVMMlEzWkV4c1pqUm9kWEpKY1doNlRVUTFlV0ZETW10alZUa3ZPR001UnpVMVNubENXRVk0UkhnMVUxRnRPUXA1TW5KUVYwWkpaRzB5T1ZGc09VRXpTVE41ZVVWR2VWQnZORWxDWW1wRFEwRlhiM2RFWjFsRVZsSXdVRUZSU0M5Q1FWRkVRV2RsUVUxQ1RVZEJNVlZrQ2twUlVVMU5RVzlIUTBOelIwRlJWVVpDZDAxRVRVSXdSMEV4VldSRVoxRlhRa0pVYkdGVlptcHdhVmhIYUVKUU0yaFBRMWN3U2twYVJGTlFlR2Q2UVdZS1FtZE9Wa2hUVFVWSFJFRlhaMEpTZUdocVEyMUdTSGhwWWk5dU16RjJVVVpIYmpsbUx5dDBkbkpFUVZsQ1owNVdTRkpGUWtGbU9FVkVha0ZOWjFGd2FBcFJTRkoxWlZNMU1HSXpaSFZOUTNkSFEybHpSMEZSVVVKbk56aDNRVkZGUlVodGFEQmtTRUo2VDJrNGRsb3liREJoU0ZacFRHMU9kbUpUT1hOaU1tUndDbUpwT1haWldGWXdZVVJCZFVKbmIzSkNaMFZGUVZsUEwwMUJSVWxDUTBGTlNHMW9NR1JJUW5wUGFUaDJXakpzTUdGSVZtbE1iVTUyWWxNNWMySXlaSEFLWW1rNWRsbFlWakJoUkVOQ2FXZFpTMHQzV1VKQ1FVaFhaVkZKUlVGblVqaENTRzlCWlVGQ01rRkRjM2QyVG5odmFVMXVhVFJrWjIxTFZqVXdTREJuTlFwTldsbERPSEIzZW5reE5VUlJVRFo1Y2tsYU5rRkJRVUpvTjNKMlpVSnpRVUZCVVVSQlJXTjNVbEZKYUVGTFQxcFFUVTQ1VVRseFR6RklXR2xuU0VKUUNuUXJTV014Tm5sNU1scG5kakpMVVRJemFUVlhUR294TmtGcFFYcHlSbkIxWVhsSFdHUnZTeXRvV1dWUWJEbGtSV1ZZYWtjdmRrSXlha3N2UlROelJYTUtTWEpZZEVWVVFVdENaMmR4YUd0cVQxQlJVVVJCZDA1d1FVUkNiVUZxUlVGbmJXaG5PREJ0U1M5VFkzSXdhWE5DYmtRMVJsbFlXamhYZUVFNGRHNUNRZ3BRYldSbU5HRk9SMFp2Y2tkaGVrZFlZVVpSVmxCWVowSldVSFlyV1VkSkwwRnFSVUV3VVhwUVF6VmtTRVF2VjFkWVZ6SkhZa1ZETkdSd2QwWnJPRTlIQ2xKcmFVVjRUVTk1THl0RGNXRmlZbFpuS3k5c2VERk9PVlpIUWxSc1ZWUm1kRFExWkFvdExTMHRMVVZPUkNCRFJWSlVTVVpKUTBGVVJTMHRMUzB0Q2c9PSJ9fX19"))
                    }
                ]),
                timestamp_verification_data: None,
            }),
            message_signature: Some(MessageSignature {
                message_digest: Some(HashOutput{
                    algorithm: Some(String::from("SHA2_256")),
                    digest: Some(String::from("gC3WD/iDM4AvJYXnMEO9IcNBKF4Zkv5bMXVeHK3q4w4="))
                }),
                signature: Some(String::from("MGUCMQCOOJqTY6XWgB64izK2WVP07b0SG9M5WPCwKhfTPwMvtsgUi8KeRGwQkvvLYbKHdqUCMEbOXFG0NMqEQxWVb6rmGnexdADuGf6Jl8qAC8tn67p3QfVoXzMvFA61PzxwVwvb8g=="))
            }),
            dsse_envelope: None
    });
}
