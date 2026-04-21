/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x98A53A2C | REVISION: 2026.04.20                           │
 * │ PATH: crates/02-resonance/dsp/src/filter/zdf.rs                          │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Ultra-Analog Zero-Delay Feedback State-Variable Filter.      │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Protected by Aztec Sacred Geometry Encryption.          │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_macros::aztec_unlock;

aztec_unlock!(
    r#"
    ████████████████████████████████████████████████████████████████████████████████
    ██                                                                            ██
    ██  S A C R E D   G E O M E T R Y   |   A Z T E C   E N C R Y P T I O N       ██
    ██                                                                            ██
    ██▒▒ QDQgKGk9GQoHARwAUmpQXjgkaXc5PBEJ ▒▒██
    ██▒▒ HRERUw0fCDtAMSYvaT0AFx0WAEltUVVi ▒▒██
    ██▒▒ PCdzNkNuVEVIBkVTF1MFBWZLc21pbgdX ▒▒██
    ██▒▒ UlUSXwMZORFqYXMqc24SU1xZfkkXFRNa ▒▒██
    ██▒▒ cGE1e31ifhhifx0ER1kTay4nADsvbg9v ▒▒██
    ██▒▒ SFVUSUdAUREsL3MjLDlcTEhYSklkUF9X ▒▒██
    ██▒▒ ajpZbWluVEVIVVQ6UllVETFhIHxzbkRL ▒▒██
    ██▒▒ WFlUGgUPEwFkcX9tLnRUVUZFWElcDxMB ▒▒██
    ██▒▒ ZHFzMENuVEVICH5JFxUTO2phc205OxZF ▒▒██
    ██▒▒ DhtUGlJBbEErMzIgOmZSCB0BVBpSWVUd ▒▒██
    ██▒▒ aiImOSYoEl9IE0JdGxVBVDkuPSwnLRFf ▒▒██
    ██▒▒ SBNCXRsVQFAnMT8oFjwVEQ1PVA8BARoR ▒▒██
    ██▒▒ MUtzbWluVEVIVRgMQxVEVWp8c39nflRP ▒▒██
    ██▒▒ SEZaWAMEBgh4d2Z+fHZNUlFGVEMXVkZF ▒▒██
    ██▒▒ JSc1dkNuVEVIVVRJF1lWRWo1c3Bpf1pV ▒▒██
    ██▒▒ SFpUGlZYQ10vHiEsPStPb0hVVEkXFRMR ▒▒██
    ██▒▒ JiQnbT4vVFhIXUZHBxUcET5oc2dpZgMB ▒▒██
    ██▒▒ SF9UHRcaEwNkcXpjPS8aTUFOfkkXFRMR ▒▒██
    ██▒▒ amFzPiwiEksPVUlJQFQTG2o1c2JpfFpV ▒▒██
    ██▒▒ U39USRcVExFqYSAoJShaDkhIVFsZBRMc ▒▒██
    ██▒▒ anN9fWlkVBcNBhsHVltQVHFLc21pbglv ▒▒██
    ██▒▒ FX9+AFpFXxEaLSYqICA7FiYaEAwXU1xD ▒▒██
    ██▒▒ ahs3Kxo4EkUTf1RJFxUQaiMvPyQnK1wE ▒▒██
    ██▒▒ BAIVEEQcbjtqYXNtLyBUFRoaFwxERhsX ▒▒██
    ██▒▒ JzQnbTorGANEVQxTF1MFBWNhfnNpKEJR ▒▒██
    ██▒▒ SA5+SRcVExFqYXMhLDpUAQ0bVFQXBB0B ▒▒██
    ██▒▒ ampzPiwiEksPVV5JH0ZWXSxvNG1ibgcA ▒▒██
    ██▒▒ BBNaAh4OORFqYXNtaW5UCQ0BVBBoXUMR ▒▒██
    ██▒▒ d2F7NWljVBYNGRJHRAQTG2ppICglKFoC ▒▒██
    ██▒▒ SF5UGlJZVR8haHNgaT0RCQ5bB1seFRwR ▒▒██
    ██▒▒ LiQ9dkNuVEVIVVRJF1lWRWo3Ym10bgcA ▒▒██
    ██▒▒ BBNaDhcfE0gVKSN2Q25URUhVVEkXWVZF ▒▒██
    ██▒▒ ajgMLzluSUUeRFRCF0ZWXSxvIHxyRFRF ▒▒██
    ██▒▒ SFVUSRcVQFQmJ30+eG5JRR5EVEIXTGxT ▒▒██
    ██▒▒ OnpZbWluVEVIVVQFUkETR3hhbm06KxgD ▒▒██
    ██▒▒ RhJUQxdMbFM6elltaW5URUhVVAVSQRNI ▒▒██
    ██▒▒ FS0jbXRuAldIXlQaUllVHzlzaEdpblRF ▒▒██
    ██▒▒ SFVUSURQX1dkMmFtdG4CV0heVBBoWUMK ▒▒██
    ██▒▒ QGFzbWluVEVIDCsFRz8TEWphLkc0RA== ▒▒██
    ██                                                                            ██
    ████████████████████████████████████████████████████████████████████████████████
    "#
);
