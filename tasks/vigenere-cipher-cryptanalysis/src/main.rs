const FREQ: [f64; 26] = [
    0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, 0.06094, 0.06966, 0.00153,
    0.00772, 0.04025, 0.02406, 0.06749, 0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056,
    0.02758, 0.00978, 0.02360, 0.00150, 0.01974, 0.00074,
];

const ENCODED: &str = r##"
MOMUD EKAPV TQEFM OEVHP AJMII CDCTI FGYAG JSPXY ALUYM NSMYH
VUXJE LEPXJ FXGCM JHKDZ RYICU HYPUS PGIGM OIYHF WHTCQ KMLRD
ITLXZ LJFVQ GHOLW CUHLO MDSOE KTALU VYLNZ RFGBX PHVGA LWQIS
FGRPH JOOFW GUBYI LAPLA LCAFA AMKLG CETDW VOELJ IKGJB XPHVG
ALWQC SNWBU BYHCU HKOCE XJEYK BQKVY KIIEH GRLGH XEOLW AWFOJ
ILOVV RHPKD WIHKN ATUHN VRYAQ DIVHX FHRZV QWMWV LGSHN NLVZS
JLAKI FHXUF XJLXM TBLQV RXXHR FZXGV LRAJI EXPRV OSMNP KEPDT
LPRWM JAZPK LQUZA ALGZX GVLKL GJTUI ITDSU REZXJ ERXZS HMPST
MTEOE PAPJH SMFNB YVQUZ AALGA YDNMP AQOWT UHDBV TSMUE UIMVH
QGVRW AEFSP EMPVE PKXZY WLKJA GWALT VYYOB YIXOK IHPDS EVLEV
RVSGB JOGYW FHKBL GLXYA MVKIS KIEHY IMAPX UOISK PVAGN MZHPW
TTZPV XFCCD TUHJH WLAPF YULTB UXJLN SIJVV YOVDJ SOLXG TGRVO
SFRII CTMKO JFCQF KTINQ BWVHG TENLH HOGCS PSFPV GJOKM SIFPR
ZPAAS ATPTZ FTPPD PORRF TAXZP KALQA WMIUD BWNCT LEFKO ZQDLX
BUXJL ASIMR PNMBF ZCYLV WAPVF QRHZV ZGZEF KBYIO OFXYE VOWGB
BXVCB XBAWG LQKCM ICRRX MACUO IKHQU AJEGL OIJHH XPVZW JEWBA
FWAML ZZRXJ EKAHV FASMU LVVUT TGK
"##;

const ASCII_A: u8 = b'A';

fn best_match(a: &[f64]) -> u8 {
    let sum: f64 = a.iter().sum();
    let mut best_fit = 1E100;
    let mut best_rotate = 0;
    (0..26).for_each(|rotate| {
        let mut fit = 0.0;
        (0..26).for_each(|i| {
            let d = a[(i + rotate) % 26] / sum - FREQ[i];
            fit += d * d / FREQ[i];
        });
        if fit < best_fit {
            best_fit = fit;
            best_rotate = rotate as u8;
        }
    });

    best_rotate
}

fn freq_every_nth(msg: &[u8], interval: usize) -> (f64, Vec<char>) {
    let len = msg.len();
    let mut key = vec!['?'; interval];
    let mut out = [0.0; 26];
    let mut accu = [0.0; 26];
    (0..interval).for_each(|j| {
        out = [0.0; 26];
        (j..len)
            .step_by(interval)
            .for_each(|i| out[(msg[i] - ASCII_A) as usize] += 1.0);
        let rot = best_match(&out);
        key[j] = (rot + ASCII_A) as char;
        (0..26).for_each(|i| {
            accu[i] += out[(i + rot as usize) % 26];
        })
    });

    let sum: f64 = accu.iter().sum();
    let mut ret = 0.0;
    (0..26).for_each(|i| {
        let d = accu[i] / sum - FREQ[i];
        ret += d * d / FREQ[i];
    });

    (ret, key)
}

fn main() {
    let mut best_fit = 1E100;
    let mut best_key = String::new();

    let sanitized_encoded = to_sanitized_bytes(ENCODED);
    (1..=26).for_each(|j| {
        let (fit, key) = freq_every_nth(&sanitized_encoded, j);
        if fit < best_fit {
            best_fit = fit;
            best_key = key.iter().collect();
        }
    });
    println!("Best key: {}", &best_key);
    println!("Decrypted text:\n{}", vigenere(ENCODED, &best_key, false));
}

fn vigenere(plaintext: &str, key: &str, encrypt: bool) -> String {
    let plaintext_bytes = to_sanitized_bytes(plaintext);
    let key_bytes = to_sanitized_bytes(key);
    let key_len = key_bytes.len();
    let mut output = String::with_capacity(plaintext_bytes.len());

    for (i, byte) in plaintext_bytes.iter().enumerate() {
        let c = *byte;
        let b = key_bytes[i % key_len];

        let output_byte = if encrypt {
            enc_byte(c, b)
        } else {
            dec_byte(c, b)
        };

        output.push(output_byte as char);
    }
    output
}

fn to_sanitized_bytes(string: &str) -> Vec<u8> {
    string
        .chars()
        .filter(|&c| c.is_alphabetic())
        .map(|c| c.to_ascii_uppercase() as u8)
        .collect::<Vec<u8>>()
}

fn enc_byte(m: u8, k: u8) -> u8 {
    ASCII_A + (m.wrapping_add(k).wrapping_sub(2 * (ASCII_A))) % 26
}

fn dec_byte(c: u8, k: u8) -> u8 {
    ASCII_A + (c.wrapping_sub(k).wrapping_add(26)) % 26
}
