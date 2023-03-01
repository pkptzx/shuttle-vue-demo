#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Default)]
pub enum InputLang {
    #[default]
    Auto,
    Galego,   // 갈리시아어 - gl
    Guarani,  // 과라니어 - gn
    Gujarati, // 구자라트어 - gu
    Greek,    // 그리스어 - el
    // TODO
    Dutch,              // 네덜란드어 - nl
    Nepali,             // 네팔어 - ne
    Norwegian,          // 노르웨이어 - no
    Danish,             // 덴마크어 - da
    Dogri,              // 도그리어 - doi
    German,             // 독일어 - de
    Dhivehi,            // 디베히어 - dv
    Lao,                // 라오어 - lo
    Latvian,            // 라트비아어 - lv
    Latin,              // 라틴어 - la
    Russian,            // 러시아어 - ru
    Luganda,            // 루간다어 - lg
    Romanian,           // 루마니아어 - ro
    Luxembourgish,      // 룩셈부르크어 - lb
    Lithuanian,         // 리투아니아어 - lt
    Lingala,            // 링갈라어 - ln
    Marathi,            // 마라티어 - mr
    Maori,              // 마오리어 - mi
    Maithili,           // 마이틸어 - mai
    Macedonian,         // 마케도니아어 - mk
    Malagasy,           // 말라가시어 - mg
    Malayalam,          // 말라얄람어 - ml
    Malay,              // 말레이어 - ms
    Meithei,            // 메이테이어(마니푸르어) - mni-Mtei
    Malti,              // 몰타어 - mt
    Mongolian,          // 몽골어 - mn
    Hmong,              // 몽어 - hmn
    Burmese,            // 미얀마어(버마어) - my
    Mizo,               // 미조어 - lus
    Basque,             // 바스크어 - eu
    Bambara,            // 밤바라어 - bm
    Vietnamese,         // 베트남어 - vi
    Belarusian,         // 벨라루스어 - be
    Bengali,            // 벵골어 - bn
    Bosnian,            // 보스니아어 - bs
    Bhojpuri,           // 보즈푸리어 - bho
    NSotho,             // 북소토어 - nso
    Bulgarian,          // 불가리아어 - bg
    Samoan,             // 사모아어 - sm
    Sanskrit,           // 산스크리트 - sa
    Serbian,            // 세르비아어 - sr
    Cebuano,            // 세부아노어 - ceb
    Sotho,              // 세소토어 - st
    Somali,             // 소말리아어 - so
    Shona,              // 쇼나어 - sn
    Sundanese,          // 순다어 - su
    Swahili,            // 스와힐리어 - sw
    Swedish,            // 스웨덴어 - sv
    ScottishGaelic,     // 스코틀랜드 게일어 - gd
    Spanish,            // 스페인어 - es
    Slovak,             // 슬로바키아어 - sk
    Slovene,            // 슬로베니아어 - sl
    Sindhi,             // 신디어 - sd
    Sinhala,            // 싱할라어 - si
    Arabic,             // 아랍어 - ar
    Armenian,           // 아르메니아어 - hy
    Assamese,           // 아삼어 - as
    Aymara,             // 아이마라어 - ay
    Icelandic,          // 아이슬란드어 - is
    HaitianCreole,      // 아이티 크리올어 - ht
    Irish,              // 아일랜드어 - ga
    Azerbaijani,        // 아제르바이잔어 - az
    Afrikaans,          // 아프리칸스어 - af
    Albanian,           // 알바니아어 - sq
    Amharic,            // 암하라어 - am
    Estonian,           // 에스토니아어 - et
    Esperanto,          // 에스페란토어 - eo
    Ewe,                // 에웨어 - ee
    English,            // 영어 - en
    Oromo,              // 오로모어 - om
    Odia,               // 오리야어 - or
    Yoruba,             // 요루바어 - yo
    Urdu,               // 우르두어 - ur
    Uzbek,              // 우즈베크어 - uz
    Ukrainian,          // 우크라이나어 - uk
    Welsh,              // 웨일즈어 - cy
    Uyghur,             // 위구르어 - ug
    Igbo,               // 이그보어 - ig
    Yiddish,            // 이디시어 - yi
    Italian,            // 이탈리아어 - it
    Indonesian,         // 인도네시아어 - id
    Ilocano,            // 일로카노어 - ilo
    Japanese,           // 일본어 - ja
    Javanese,           // 자바어 - jw
    Georgian,           // 조지아어 - ka
    Zulu,               // 줄루어 - zu
    SimplifiedChinese,  // 중국어(간체) - zh-CN
    TraditionalChinese, // 중국어(번체) - zh-TW
    Chewa,              // 체와어 - ny
    Czech,              // 체코어 - cs
    Tsonga,             // 총가어 - ts
    Kazakh,             // 카자흐어 - kk
    Catalan,            // 카탈로니아어 - ca
    Kannada,            // 칸나다어 kn
    Quechuan,           // 케추아어 - qu
    Corsican,           // 코르시카어 - co
    Xhosa,              // 코사어 - xh
    Konkani,            // 콘칸어 - gom
    Sorani,             // 쿠르드어(소라니) - ckb
    Kurmanji,           // 쿠트드어(쿠르만지) - ku
    Croatian,           // 크로아티아어 - hr
    Krio,               // 크리오어 - kri
    Khmer,              // 크메르어 - km
    Kinyarwanda,        // 키냐르완다어 - rw
    Kyrgyz,             // 키르기스어 - ky
    Tamil,              // 타밀어 - ta
    Tajik,              // 타지크어 - tg
    Tatar,              // 타타르어 - tt
    Thai,               // 태국어 - th
    Turkish,            // 터키어 - tr
    Telugu,             // 텔루구어 - te
    Turkmen,            // 투르크멘어 - tk
    Akan,               // 트위어 - ak
    Tigrinya,           // 티그리냐어 - ti
    Pashto,             // 파슈토어 - ps
    Punjabi,            // 펀자브어 - pa
    Persian,            // 페르시아어 - fa
    Portuguese,         // 포르투갈어 - pt
    Polish,             // 폴란드어 - pl
    French,             // 프랑스어 - fr
    Frisian,            // 프리지아어 - fy
    Finnish,            // 핀란드어 - fi
    Filipino,           // 필리핀어 - tl
    Hawaiian,           // 하와이어 - haw
    Hausa,              // 하우사어 - ha
    Korean,             // 한국어 - ko
    Hungarian,          // 헝가리어 - hu
    Hebrew,             // 히브리어 - iw
    Hindi,              // 힌디어 - hi
}

unsafe impl Send for InputLang {}

impl Into<InputLang> for String {
    fn into(self) -> InputLang {
        self.as_str().into()
    }
}

impl Into<InputLang> for &String {
    fn into(self) -> InputLang {
        self.as_str().into()
    }
}

impl Into<InputLang> for &str {
    /// 언어 코드, 언어 이름, 사용 지역(하나일 때)을 입력받아 InputLang을 반환합니다.
    fn into(self) -> InputLang {
        let data = self.to_lowercase();
        match data.as_str() {
            "auto" => InputLang::Auto,

            // 갈리시아어
            "gl" | "glg" => InputLang::Galego,
            "galego" => InputLang::Galego,
            "galicia" => InputLang::Galego,
            "galician" => InputLang::Galego,
            "gallego" => InputLang::Galego,

            // 과라니어
            "gn" | "grn" | "nhd" | "gui" | "gun" | "gug" | "gnw" => InputLang::Guarani,
            "guaraní" | "guarani" => InputLang::Guarani,
            "avañe'ẽ" => InputLang::Guarani,

            // 구자라트어
            "gu" | "guj" => InputLang::Gujarati,
            "gujarati" => InputLang::Gujarati,
            "gujarat" => InputLang::Gujarati,
            "ગુજરાતી" => InputLang::Gujarati,
            "ગુજરાત" => InputLang::Gujarati,
            "gujarātī" => InputLang::Gujarati,

            // 그리스어
            "el" | "gre" | "ell" | "grc" | "cpg" | "gmy" | "pnt" | "tsd" | "yej" => {
                InputLang::Greek
            }
            "eλληνικά" => InputLang::Greek,
            "elliniká" => InputLang::Greek,
            "Ἑλληνική" => InputLang::Greek,
            "eλληνική" => InputLang::Greek,
            "ελληνικά" => InputLang::Greek,
            "hellēnikḗ" => InputLang::Greek,
            "greece" => InputLang::Greek,
            "hellenic" => InputLang::Greek,

            "nl" => InputLang::Dutch,
            "ne" => InputLang::Nepali,
            "no" => InputLang::Norwegian,
            "da" => InputLang::Danish,
            "doi" => InputLang::Dogri,
            "de" => InputLang::German,
            "dv" => InputLang::Dhivehi,
            "lo" => InputLang::Lao,
            "lv" => InputLang::Latvian,
            "la" => InputLang::Latin,
            "ru" => InputLang::Russian,
            "lg" => InputLang::Luganda,
            "ro" => InputLang::Romanian,
            "lb" => InputLang::Luxembourgish,
            "lt" => InputLang::Lithuanian,
            "ln" => InputLang::Lingala,
            "mr" => InputLang::Marathi,
            "mi" => InputLang::Maori,
            "mai" => InputLang::Maithili,
            "mk" => InputLang::Macedonian,
            "mg" => InputLang::Malagasy,
            "ml" => InputLang::Malayalam,
            "ms" => InputLang::Malay,
            "mni-mtei" | "mni-Mtei" => InputLang::Meithei,
            "mt" => InputLang::Malti,
            "mn" => InputLang::Mongolian,
            "hmn" => InputLang::Hmong,
            "my" => InputLang::Burmese,
            "lus" => InputLang::Mizo,
            "eu" => InputLang::Basque,
            "bm" => InputLang::Bambara,
            "vi" => InputLang::Vietnamese,
            "be" => InputLang::Belarusian,
            "bn" => InputLang::Bengali,
            "bs" => InputLang::Bosnian,
            "bho" => InputLang::Bhojpuri,
            "nso" => InputLang::NSotho,
            "bg" => InputLang::Bulgarian,
            "sm" => InputLang::Samoan,
            "sa" => InputLang::Sanskrit,
            "sr" => InputLang::Serbian,
            "ceb" => InputLang::Cebuano,
            "st" => InputLang::Sotho,
            "so" => InputLang::Somali,
            "sn" => InputLang::Shona,
            "su" => InputLang::Sundanese,
            "sw" => InputLang::Swahili,
            "sv" => InputLang::Swedish,
            "gd" => InputLang::ScottishGaelic,
            "es" => InputLang::Spanish,
            "sk" => InputLang::Slovak,
            "sl" => InputLang::Slovene,
            "sd" => InputLang::Sindhi,
            "si" => InputLang::Sinhala,
            "ar" => InputLang::Arabic,
            "hy" => InputLang::Armenian,
            "as" => InputLang::Assamese,
            "ay" => InputLang::Aymara,
            "is" => InputLang::Icelandic,
            "ht" => InputLang::HaitianCreole,
            "ga" => InputLang::Irish,
            "az" => InputLang::Azerbaijani,
            "af" => InputLang::Afrikaans,
            "sq" => InputLang::Albanian,
            "am" => InputLang::Amharic,
            "et" => InputLang::Estonian,
            "eo" => InputLang::Esperanto,
            "ee" => InputLang::Ewe,
            "en" => InputLang::English,
            "om" => InputLang::Oromo,
            "or" => InputLang::Odia,
            "yo" => InputLang::Yoruba,
            "ur" => InputLang::Urdu,
            "uz" => InputLang::Uzbek,
            "uk" => InputLang::Ukrainian,
            "cy" => InputLang::Welsh,
            "ug" => InputLang::Uyghur,
            "ig" => InputLang::Igbo,
            "yi" => InputLang::Yiddish,
            "it" => InputLang::Italian,
            "id" => InputLang::Indonesian,
            "ilo" => InputLang::Ilocano,
            "ja" => InputLang::Japanese,
            "jw" => InputLang::Javanese,
            "ka" => InputLang::Georgian,
            "zu" => InputLang::Zulu,
            "zh-cn" | "zh-CN" => InputLang::SimplifiedChinese,
            "zh-tw" | "zh-TW" => InputLang::TraditionalChinese,
            "ny" => InputLang::Chewa,
            "cs" => InputLang::Czech,
            "ts" => InputLang::Tsonga,
            "kk" => InputLang::Kazakh,
            "ca" => InputLang::Catalan,
            "kn" => InputLang::Kannada,
            "qu" => InputLang::Quechuan,
            "co" => InputLang::Corsican,
            "xh" => InputLang::Xhosa,
            "gom" => InputLang::Konkani,
            "ckb" => InputLang::Sorani,
            "ku" => InputLang::Kurmanji,
            "hr" => InputLang::Croatian,
            "kri" => InputLang::Krio,
            "km" => InputLang::Khmer,
            "rw" => InputLang::Kinyarwanda,
            "ky" => InputLang::Kyrgyz,
            "ta" => InputLang::Tamil,
            "tg" => InputLang::Tajik,
            "tt" => InputLang::Tatar,
            "th" => InputLang::Thai,
            "tr" => InputLang::Turkish,
            "te" => InputLang::Telugu,
            "tk" => InputLang::Turkmen,
            "ak" => InputLang::Akan,
            "ti" => InputLang::Tigrinya,
            "ps" => InputLang::Pashto,
            "pa" => InputLang::Punjabi,
            "fa" => InputLang::Persian,
            "pt" => InputLang::Portuguese,
            "pl" => InputLang::Polish,
            "fr" => InputLang::French,
            "fy" => InputLang::Frisian,
            "fi" => InputLang::Finnish,
            "tl" => InputLang::Filipino,
            "haw" => InputLang::Hawaiian,
            "ha" => InputLang::Hausa,
            "ko" => InputLang::Korean,
            "hu" => InputLang::Hungarian,
            "iw" => InputLang::Hebrew,
            "hi" => InputLang::Hindi,

            // 그 외
            _ => InputLang::default(),
        }
    }
}

impl ToString for InputLang {
    fn to_string(&self) -> String {
        let lang = match self {
            InputLang::Auto => "auto",
            InputLang::Galego => "gl",
            InputLang::Guarani => "gn",
            InputLang::Gujarati => "gu",
            InputLang::Greek => "el",
            InputLang::Dutch => "nl",
            InputLang::Nepali => "ne",
            InputLang::Norwegian => "no",
            InputLang::Danish => "da",
            InputLang::Dogri => "doi",
            InputLang::German => "de",
            InputLang::Dhivehi => "dv",
            InputLang::Lao => "lo",
            InputLang::Latvian => "lv",
            InputLang::Latin => "la",
            InputLang::Russian => "ru",
            InputLang::Luganda => "lg",
            InputLang::Romanian => "ro",
            InputLang::Luxembourgish => "lb",
            InputLang::Lithuanian => "lt",
            InputLang::Lingala => "ln",
            InputLang::Marathi => "mr",
            InputLang::Maori => "mi",
            InputLang::Maithili => "mai",
            InputLang::Macedonian => "mk",
            InputLang::Malagasy => "mg",
            InputLang::Malayalam => "ml",
            InputLang::Malay => "ms",
            InputLang::Meithei => "mni-Mtei",
            InputLang::Malti => "mt",
            InputLang::Mongolian => "mn",
            InputLang::Hmong => "hmn",
            InputLang::Burmese => "my",
            InputLang::Mizo => "lus",
            InputLang::Basque => "eu",
            InputLang::Bambara => "bm",
            InputLang::Vietnamese => "vi",
            InputLang::Belarusian => "be",
            InputLang::Bengali => "bn",
            InputLang::Bosnian => "bs",
            InputLang::Bhojpuri => "bho",
            InputLang::NSotho => "nso",
            InputLang::Bulgarian => "bg",
            InputLang::Samoan => "sm",
            InputLang::Sanskrit => "sa",
            InputLang::Serbian => "sr",
            InputLang::Cebuano => "ceb",
            InputLang::Sotho => "st",
            InputLang::Somali => "so",
            InputLang::Shona => "sn",
            InputLang::Sundanese => "su",
            InputLang::Swahili => "sw",
            InputLang::Swedish => "sv",
            InputLang::ScottishGaelic => "gd",
            InputLang::Spanish => "es",
            InputLang::Slovak => "sk",
            InputLang::Slovene => "sl",
            InputLang::Sindhi => "sd",
            InputLang::Sinhala => "si",
            InputLang::Arabic => "ar",
            InputLang::Armenian => "hy",
            InputLang::Assamese => "as",
            InputLang::Aymara => "ay",
            InputLang::Icelandic => "is",
            InputLang::HaitianCreole => "ht",
            InputLang::Irish => "ga",
            InputLang::Azerbaijani => "az",
            InputLang::Afrikaans => "af",
            InputLang::Albanian => "sq",
            InputLang::Amharic => "am",
            InputLang::Estonian => "et",
            InputLang::Esperanto => "eo",
            InputLang::Ewe => "ee",
            InputLang::English => "en",
            InputLang::Oromo => "om",
            InputLang::Odia => "or",
            InputLang::Yoruba => "yo",
            InputLang::Urdu => "ur",
            InputLang::Uzbek => "uz",
            InputLang::Ukrainian => "uk",
            InputLang::Welsh => "cy",
            InputLang::Uyghur => "ug",
            InputLang::Igbo => "ig",
            InputLang::Yiddish => "yi",
            InputLang::Italian => "it",
            InputLang::Indonesian => "id",
            InputLang::Ilocano => "ilo",
            InputLang::Japanese => "ja",
            InputLang::Javanese => "jw",
            InputLang::Georgian => "ka",
            InputLang::Zulu => "zu",
            InputLang::SimplifiedChinese => "zh-CN",
            InputLang::TraditionalChinese => "zh-TW",
            InputLang::Chewa => "ny",
            InputLang::Czech => "cs",
            InputLang::Tsonga => "ts",
            InputLang::Kazakh => "kk",
            InputLang::Catalan => "ca",
            InputLang::Kannada => "kn",
            InputLang::Quechuan => "qu",
            InputLang::Corsican => "co",
            InputLang::Xhosa => "xh",
            InputLang::Konkani => "gom",
            InputLang::Sorani => "ckb",
            InputLang::Kurmanji => "ku",
            InputLang::Croatian => "hr",
            InputLang::Krio => "kri",
            InputLang::Khmer => "km",
            InputLang::Kinyarwanda => "rw",
            InputLang::Kyrgyz => "ky",
            InputLang::Tamil => "ta",
            InputLang::Tajik => "tg",
            InputLang::Tatar => "tt",
            InputLang::Thai => "th",
            InputLang::Turkish => "tr",
            InputLang::Telugu => "te",
            InputLang::Turkmen => "tk",
            InputLang::Akan => "ak",
            InputLang::Tigrinya => "ti",
            InputLang::Pashto => "ps",
            InputLang::Punjabi => "pa",
            InputLang::Persian => "fa",
            InputLang::Portuguese => "pt",
            InputLang::Polish => "pl",
            InputLang::French => "fr",
            InputLang::Frisian => "fy",
            InputLang::Finnish => "fi",
            InputLang::Filipino => "tl",
            InputLang::Hawaiian => "haw",
            InputLang::Hausa => "ha",
            InputLang::Korean => "ko",
            InputLang::Hungarian => "hu",
            InputLang::Hebrew => "iw",
            InputLang::Hindi => "hi",
        };
        lang.to_owned()
    }
}

/*/////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////// */

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Default)]
pub enum OutputLang {
    Galego,         // 갈리시아어 - gl
    Guarani,        // 과라니어 - gn
    Gujarati,       // 구자라트어 - gu
    Greek,          // 그리스어 - el
    Dutch,          // 네덜란드어 - nl
    Nepali,         // 네팔어 - ne
    Norwegian,      // 노르웨이어 - no
    Danish,         // 덴마크어 - da
    Dogri,          // 도그리어 - doi
    German,         // 독일어 - de
    Dhivehi,        // 디베히어 - dv
    Lao,            // 라오어 - lo
    Latvian,        // 라트비아어 - lv
    Latin,          // 라틴어 - la
    Russian,        // 러시아어 - ru
    Luganda,        // 루간다어 - lg
    Romanian,       // 루마니아어 - ro
    Luxembourgish,  // 룩셈부르크어 - lb
    Lithuanian,     // 리투아니아어 - lt
    Lingala,        // 링갈라어 - ln
    Marathi,        // 마라티어 - mr
    Maori,          // 마오리어 - mi
    Maithili,       // 마이틸어 - mai
    Macedonian,     // 마케도니아어 - mk
    Malagasy,       // 말라가시어 - mg
    Malayalam,      // 말라얄람어 - ml
    Malay,          // 말레이어 - ms
    Meithei,        // 메이테이어(마니푸르어) - mni-Mtei
    Malti,          // 몰타어 - mt
    Mongolian,      // 몽골어 - mn
    Hmong,          // 몽어 - hmn
    Burmese,        // 미얀마어(버마어) - my
    Mizo,           // 미조어 - lus
    Basque,         // 바스크어 - eu
    Bambara,        // 밤바라어 - bm
    Vietnamese,     // 베트남어 - vi
    Belarusian,     // 벨라루스어 - be
    Bengali,        // 벵골어 - bn
    Bosnian,        // 보스니아어 - bs
    Bhojpuri,       // 보즈푸리어 - bho
    NSotho,         // 북소토어 - nso
    Bulgarian,      // 불가리아어 - bg
    Samoan,         // 사모아어 - sm
    Sanskrit,       // 산스크리트 - sa
    Serbian,        // 세르비아어 - sr
    Cebuano,        // 세부아노어 - ceb
    Sotho,          // 세소토어 - st
    Somali,         // 소말리아어 - so
    Shona,          // 쇼나어 - sn
    Sundanese,      // 순다어 - su
    Swahili,        // 스와힐리어 - sw
    Swedish,        // 스웨덴어 - sv
    ScottishGaelic, // 스코틀랜드 게일어 - gd
    Spanish,        // 스페인어 - es
    Slovak,         // 슬로바키아어 - sk
    Slovene,        // 슬로베니아어 - sl
    Sindhi,         // 신디어 - sd
    Sinhala,        // 싱할라어 - si
    Arabic,         // 아랍어 - ar
    Armenian,       // 아르메니아어 - hy
    Assamese,       // 아삼어 - as
    Aymara,         // 아이마라어 - ay
    Icelandic,      // 아이슬란드어 - is
    HaitianCreole,  // 아이티 크리올어 - ht
    Irish,          // 아일랜드어 - ga
    Azerbaijani,    // 아제르바이잔어 - az
    Afrikaans,      // 아프리칸스어 - af
    Albanian,       // 알바니아어 - sq
    Amharic,        // 암하라어 - am
    Estonian,       // 에스토니아어 - et
    Esperanto,      // 에스페란토어 - eo
    Ewe,            // 에웨어 - ee
    #[default]
    English, // 영어 - en
    Oromo,          // 오로모어 - om
    Odia,           // 오리야어 - or
    Yoruba,         // 요루바어 - yo
    Urdu,           // 우르두어 - ur
    Uzbek,          // 우즈베크어 - uz
    Ukrainian,      // 우크라이나어 - uk
    Welsh,          // 웨일즈어 - cy
    Uyghur,         // 위구르어 - ug
    Igbo,           // 이그보어 - ig
    Yiddish,        // 이디시어 - yi
    Italian,        // 이탈리아어 - it
    Indonesian,     // 인도네시아어 - id
    Ilocano,        // 일로카노어 - ilo
    Japanese,       // 일본어 - ja
    Javanese,       // 자바어 - jw
    Georgian,       // 조지아어 - ka
    Zulu,           // 줄루어 - zu
    SimplifiedChinese, // 중국어(간체) - zh-CN
    TraditionalChinese, // 중국어(번체) - zh-TW
    Chewa,          // 체와어 - ny
    Czech,          // 체코어 - cs
    Tsonga,         // 총가어 - ts
    Kazakh,         // 카자흐어 - kk
    Catalan,        // 카탈로니아어 - ca
    Kannada,        // 칸나다어 kn
    Quechuan,       // 케추아어 - qu
    Corsican,       // 코르시카어 - co
    Xhosa,          // 코사어 - xh
    Konkani,        // 콘칸어 - gom
    Sorani,         // 쿠르드어(소라니) - ckb
    Kurmanji,       // 쿠트드어(쿠르만지) - ku
    Croatian,       // 크로아티아어 - hr
    Krio,           // 크리오어 - kri
    Khmer,          // 크메르어 - km
    Kinyarwanda,    // 키냐르완다어 - rw
    Kyrgyz,         // 키르기스어 - ky
    Tamil,          // 타밀어 - ta
    Tajik,          // 타지크어 - tg
    Tatar,          // 타타르어 - tt
    Thai,           // 태국어 - th
    Turkish,        // 터키어 - tr
    Telugu,         // 텔루구어 - te
    Turkmen,        // 투르크멘어 - tk
    Akan,           // 트위어 - ak
    Tigrinya,       // 티그리냐어 - ti
    Pashto,         // 파슈토어 - ps
    Punjabi,        // 펀자브어 - pa
    Persian,        // 페르시아어 - fa
    Portuguese,     // 포르투갈어 - pt
    Polish,         // 폴란드어 - pl
    French,         // 프랑스어 - fr
    Frisian,        // 프리지아어 - fy
    Finnish,        // 핀란드어 - fi
    Filipino,       // 필리핀어 - tl
    Hawaiian,       // 하와이어 - haw
    Hausa,          // 하우사어 - ha
    Korean,         // 한국어 - ko
    Hungarian,      // 헝가리어 - hu
    Hebrew,         // 히브리어 - iw
    Hindi,          // 힌디어 - hi
}

unsafe impl Send for OutputLang {}

impl Into<OutputLang> for String {
    fn into(self) -> OutputLang {
        self.as_str().into()
    }
}

impl Into<OutputLang> for &String {
    fn into(self) -> OutputLang {
        self.as_str().into()
    }
}

impl Into<OutputLang> for &str {
    /// 언어 코드, 언어 이름, 사용 지역(하나일 때)을 입력받아 OutputLang을 반환합니다.
    fn into(self) -> OutputLang {
        let data = self.to_lowercase();
        match  data.as_str() {
            // 갈리시아어
            "gl" | "glg" => OutputLang::Galego,
            "galego" => OutputLang::Galego,
            "galicia" => OutputLang::Galego,
            "galician" => OutputLang::Galego,
            "gallego" => OutputLang::Galego,

            // 과라니어
            "gn" | "grn" | "nhd" | "gui" | "gun" | "gug" | "gnw" => OutputLang::Guarani,
            "guaraní" | "guarani" => OutputLang::Guarani,
            "avañe'ẽ" => OutputLang::Guarani,

            // 구자라트어
            "gu" | "guj" => OutputLang::Gujarati,
            "gujarati" => OutputLang::Gujarati,
            "gujarat" => OutputLang::Gujarati,
            "ગુજરાતી" => OutputLang::Gujarati,
            "ગુજરાત" => OutputLang::Gujarati,
            "gujarātī" => OutputLang::Gujarati,

            // 그리스어
            "el" | "gre" | "ell" | "grc" | "cpg" | "gmy" | "pnt" | "tsd" | "yej" => {
                OutputLang::Greek
            }
            "eλληνικά" => OutputLang::Greek,
            "elliniká" => OutputLang::Greek,
            "Ἑλληνική" => OutputLang::Greek,
            "eλληνική" => OutputLang::Greek,
            "ελληνικά" => OutputLang::Greek,
            "hellēnikḗ" => OutputLang::Greek,
            "greece" => OutputLang::Greek,
            "hellenic" => OutputLang::Greek,

            "nl" => OutputLang::Dutch,
            "ne" => OutputLang::Nepali,
            "no" => OutputLang::Norwegian,
            "da" => OutputLang::Danish,
            "doi" => OutputLang::Dogri,
            "de" => OutputLang::German,
            "dv" => OutputLang::Dhivehi,
            "lo" => OutputLang::Lao,
            "lv" => OutputLang::Latvian,
            "la" => OutputLang::Latin,
            "ru" => OutputLang::Russian,
            "lg" => OutputLang::Luganda,
            "ro" => OutputLang::Romanian,
            "lb" => OutputLang::Luxembourgish,
            "lt" => OutputLang::Lithuanian,
            "ln" => OutputLang::Lingala,
            "mr" => OutputLang::Marathi,
            "mi" => OutputLang::Maori,
            "mai" => OutputLang::Maithili,
            "mk" => OutputLang::Macedonian,
            "mg" => OutputLang::Malagasy,
            "ml" => OutputLang::Malayalam,
            "ms" => OutputLang::Malay,
            "mni-mtei"|"mni-Mtei" => OutputLang::Meithei,
            "mt" => OutputLang::Malti,
            "mn" => OutputLang::Mongolian,
            "hmn" => OutputLang::Hmong,
            "my" => OutputLang::Burmese,
            "lus" => OutputLang::Mizo,
            "eu" => OutputLang::Basque,
            "bm" => OutputLang::Bambara,
            "vi" => OutputLang::Vietnamese,
            "be" => OutputLang::Belarusian,
            "bn" => OutputLang::Bengali,
            "bs" => OutputLang::Bosnian,
            "bho" => OutputLang::Bhojpuri,
            "nso" => OutputLang::NSotho,
            "bg" => OutputLang::Bulgarian,
            "sm" => OutputLang::Samoan,
            "sa" => OutputLang::Sanskrit,
            "sr" => OutputLang::Serbian,
            "ceb" => OutputLang::Cebuano,
            "st" => OutputLang::Sotho,
            "so" => OutputLang::Somali,
            "sn" => OutputLang::Shona,
            "su" => OutputLang::Sundanese,
            "sw" => OutputLang::Swahili,
            "sv" => OutputLang::Swedish,
            "gd" => OutputLang::ScottishGaelic,
            "es" => OutputLang::Spanish,
            "sk" => OutputLang::Slovak,
            "sl" => OutputLang::Slovene,
            "sd" => OutputLang::Sindhi,
            "si" => OutputLang::Sinhala,
            "ar" => OutputLang::Arabic,
            "hy" => OutputLang::Armenian,
            "as" => OutputLang::Assamese,
            "ay" => OutputLang::Aymara,
            "is" => OutputLang::Icelandic,
            "ht" => OutputLang::HaitianCreole,
            "ga" => OutputLang::Irish,
            "az" => OutputLang::Azerbaijani,
            "af" => OutputLang::Afrikaans,
            "sq" => OutputLang::Albanian,
            "am" => OutputLang::Amharic,
            "et" => OutputLang::Estonian,
            "eo" => OutputLang::Esperanto,
            "ee" => OutputLang::Ewe,
            "en" => OutputLang::English,
            "om" => OutputLang::Oromo,
            "or" => OutputLang::Odia,
            "yo" => OutputLang::Yoruba,
            "ur" => OutputLang::Urdu,
            "uz" => OutputLang::Uzbek,
            "uk" => OutputLang::Ukrainian,
            "cy" => OutputLang::Welsh,
            "ug" => OutputLang::Uyghur,
            "ig" => OutputLang::Igbo,
            "yi" => OutputLang::Yiddish,
            "it" => OutputLang::Italian,
            "id" => OutputLang::Indonesian,
            "ilo" => OutputLang::Ilocano,
            "ja" => OutputLang::Japanese,
            "jw" => OutputLang::Javanese,
            "ka" => OutputLang::Georgian,
            "zu" => OutputLang::Zulu,
            "zh-cn" | "zh-CN" => OutputLang::SimplifiedChinese,
            "zh-tw" | "zh-TW" => OutputLang::TraditionalChinese,
            "ny" => OutputLang::Chewa,
            "cs" => OutputLang::Czech,
            "ts" => OutputLang::Tsonga,
            "kk" => OutputLang::Kazakh,
            "ca" => OutputLang::Catalan,
            "kn" => OutputLang::Kannada,
            "qu" => OutputLang::Quechuan,
            "co" => OutputLang::Corsican,
            "xh" => OutputLang::Xhosa,
            "gom" => OutputLang::Konkani,
            "ckb" => OutputLang::Sorani,
            "ku" => OutputLang::Kurmanji,
            "hr" => OutputLang::Croatian,
            "kri" => OutputLang::Krio,
            "km" => OutputLang::Khmer,
            "rw" => OutputLang::Kinyarwanda,
            "ky" => OutputLang::Kyrgyz,
            "ta" => OutputLang::Tamil,
            "tg" => OutputLang::Tajik,
            "tt" => OutputLang::Tatar,
            "th" => OutputLang::Thai,
            "tr" => OutputLang::Turkish,
            "te" => OutputLang::Telugu,
            "tk" => OutputLang::Turkmen,
            "ak" => OutputLang::Akan,
            "ti" => OutputLang::Tigrinya,
            "ps" => OutputLang::Pashto,
            "pa" => OutputLang::Punjabi,
            "fa" => OutputLang::Persian,
            "pt" => OutputLang::Portuguese,
            "pl" => OutputLang::Polish,
            "fr" => OutputLang::French,
            "fy" => OutputLang::Frisian,
            "fi" => OutputLang::Finnish,
            "tl" => OutputLang::Filipino,
            "haw" => OutputLang::Hawaiian,
            "ha" => OutputLang::Hausa,
            "ko" => OutputLang::Korean,
            "hu" => OutputLang::Hungarian,
            "iw" => OutputLang::Hebrew,
            "hi" => OutputLang::Hindi,

            // 그 외
            _ => {println!("未匹配到: {}",self);OutputLang::default()},
        }
    }
}

impl ToString for OutputLang {
    fn to_string(&self) -> String {
        let lang = match self {
            OutputLang::Galego => "gl",
            OutputLang::Guarani => "gn",
            OutputLang::Gujarati => "gu",
            OutputLang::Greek => "el",
            OutputLang::Dutch => "nl",
            OutputLang::Nepali => "ne",
            OutputLang::Norwegian => "no",
            OutputLang::Danish => "da",
            OutputLang::Dogri => "doi",
            OutputLang::German => "de",
            OutputLang::Dhivehi => "dv",
            OutputLang::Lao => "lo",
            OutputLang::Latvian => "lv",
            OutputLang::Latin => "la",
            OutputLang::Russian => "ru",
            OutputLang::Luganda => "lg",
            OutputLang::Romanian => "ro",
            OutputLang::Luxembourgish => "lb",
            OutputLang::Lithuanian => "lt",
            OutputLang::Lingala => "ln",
            OutputLang::Marathi => "mr",
            OutputLang::Maori => "mi",
            OutputLang::Maithili => "mai",
            OutputLang::Macedonian => "mk",
            OutputLang::Malagasy => "mg",
            OutputLang::Malayalam => "ml",
            OutputLang::Malay => "ms",
            OutputLang::Meithei => "mni-Mtei",
            OutputLang::Malti => "mt",
            OutputLang::Mongolian => "mn",
            OutputLang::Hmong => "hmn",
            OutputLang::Burmese => "my",
            OutputLang::Mizo => "lus",
            OutputLang::Basque => "eu",
            OutputLang::Bambara => "bm",
            OutputLang::Vietnamese => "vi",
            OutputLang::Belarusian => "be",
            OutputLang::Bengali => "bn",
            OutputLang::Bosnian => "bs",
            OutputLang::Bhojpuri => "bho",
            OutputLang::NSotho => "nso",
            OutputLang::Bulgarian => "bg",
            OutputLang::Samoan => "sm",
            OutputLang::Sanskrit => "sa",
            OutputLang::Serbian => "sr",
            OutputLang::Cebuano => "ceb",
            OutputLang::Sotho => "st",
            OutputLang::Somali => "so",
            OutputLang::Shona => "sn",
            OutputLang::Sundanese => "su",
            OutputLang::Swahili => "sw",
            OutputLang::Swedish => "sv",
            OutputLang::ScottishGaelic => "gd",
            OutputLang::Spanish => "es",
            OutputLang::Slovak => "sk",
            OutputLang::Slovene => "sl",
            OutputLang::Sindhi => "sd",
            OutputLang::Sinhala => "si",
            OutputLang::Arabic => "ar",
            OutputLang::Armenian => "hy",
            OutputLang::Assamese => "as",
            OutputLang::Aymara => "ay",
            OutputLang::Icelandic => "is",
            OutputLang::HaitianCreole => "ht",
            OutputLang::Irish => "ga",
            OutputLang::Azerbaijani => "az",
            OutputLang::Afrikaans => "af",
            OutputLang::Albanian => "sq",
            OutputLang::Amharic => "am",
            OutputLang::Estonian => "et",
            OutputLang::Esperanto => "eo",
            OutputLang::Ewe => "ee",
            OutputLang::English => "en",
            OutputLang::Oromo => "om",
            OutputLang::Odia => "or",
            OutputLang::Yoruba => "yo",
            OutputLang::Urdu => "ur",
            OutputLang::Uzbek => "uz",
            OutputLang::Ukrainian => "uk",
            OutputLang::Welsh => "cy",
            OutputLang::Uyghur => "ug",
            OutputLang::Igbo => "ig",
            OutputLang::Yiddish => "yi",
            OutputLang::Italian => "it",
            OutputLang::Indonesian => "id",
            OutputLang::Ilocano => "ilo",
            OutputLang::Japanese => "ja",
            OutputLang::Javanese => "jw",
            OutputLang::Georgian => "ka",
            OutputLang::Zulu => "zu",
            OutputLang::SimplifiedChinese => "zh-CN",
            OutputLang::TraditionalChinese => "zh-TW",
            OutputLang::Chewa => "ny",
            OutputLang::Czech => "cs",
            OutputLang::Tsonga => "ts",
            OutputLang::Kazakh => "kk",
            OutputLang::Catalan => "ca",
            OutputLang::Kannada => "kn",
            OutputLang::Quechuan => "qu",
            OutputLang::Corsican => "co",
            OutputLang::Xhosa => "xh",
            OutputLang::Konkani => "gom",
            OutputLang::Sorani => "ckb",
            OutputLang::Kurmanji => "ku",
            OutputLang::Croatian => "hr",
            OutputLang::Krio => "kri",
            OutputLang::Khmer => "km",
            OutputLang::Kinyarwanda => "rw",
            OutputLang::Kyrgyz => "ky",
            OutputLang::Tamil => "ta",
            OutputLang::Tajik => "tg",
            OutputLang::Tatar => "tt",
            OutputLang::Thai => "th",
            OutputLang::Turkish => "tr",
            OutputLang::Telugu => "te",
            OutputLang::Turkmen => "tk",
            OutputLang::Akan => "ak",
            OutputLang::Tigrinya => "ti",
            OutputLang::Pashto => "ps",
            OutputLang::Punjabi => "pa",
            OutputLang::Persian => "fa",
            OutputLang::Portuguese => "pt",
            OutputLang::Polish => "pl",
            OutputLang::French => "fr",
            OutputLang::Frisian => "fy",
            OutputLang::Finnish => "fi",
            OutputLang::Filipino => "tl",
            OutputLang::Hawaiian => "haw",
            OutputLang::Hausa => "ha",
            OutputLang::Korean => "ko",
            OutputLang::Hungarian => "hu",
            OutputLang::Hebrew => "iw",
            OutputLang::Hindi => "hi",
        };
        lang.to_owned()
    }
}