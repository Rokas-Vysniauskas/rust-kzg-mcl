#[cfg(test)]
mod tests {
    use kzg_bench::tests::fft_g1::{compare_sft_fft, roundtrip_fft, stride_fft};
    use rust_kzg_arkworks::fft_g1::{fft_g1_fast, fft_g1_slow, make_data};
    use rust_kzg_arkworks::kzg_proofs::FFTSettings;
    use rust_kzg_arkworks::kzg_types::{ArkG1, FsFr};

    #[test]
    fn roundtrip_fft_() {
        roundtrip_fft::<FsFr, ArkG1, FFTSettings>(&make_data);
    }

    #[test]
    fn stride_fft_() {
        stride_fft::<FsFr, ArkG1, FFTSettings>(&make_data);
    }

    #[test]
    fn compare_sft_fft_() {
        compare_sft_fft::<FsFr, ArkG1, FFTSettings>(&fft_g1_fast, &fft_g1_slow, &make_data);
    }
}
