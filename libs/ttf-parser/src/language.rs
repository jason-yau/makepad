#[rustfmt::skip]
static TABLE: &[(u16, Language, &str, &str)] = &[
    (0x0000, Language::Unknown, "Unknown", "Unknown"),
    (0x0436, Language::Afrikaans_SouthAfrica, "Afrikaans", "South Africa"),
    (0x041C, Language::Albanian_Albania, "Albanian", "Albania"),
    (0x0484, Language::Alsatian_France, "Alsatian", "France"),
    (0x045E, Language::Amharic_Ethiopia, "Amharic", "Ethiopia"),
    (0x1401, Language::Arabic_Algeria, "Arabic", "Algeria"),
    (0x3C01, Language::Arabic_Bahrain, "Arabic", "Bahrain"),
    (0x0C01, Language::Arabic_Egypt, "Arabic", "Egypt"),
    (0x0801, Language::Arabic_Iraq, "Arabic", "Iraq"),
    (0x2C01, Language::Arabic_Jordan, "Arabic", "Jordan"),
    (0x3401, Language::Arabic_Kuwait, "Arabic", "Kuwait"),
    (0x3001, Language::Arabic_Lebanon, "Arabic", "Lebanon"),
    (0x1001, Language::Arabic_Libya, "Arabic", "Libya"),
    (0x1801, Language::Arabic_Morocco, "Arabic", "Morocco"),
    (0x2001, Language::Arabic_Oman, "Arabic", "Oman"),
    (0x4001, Language::Arabic_Qatar, "Arabic", "Qatar"),
    (0x0401, Language::Arabic_SaudiArabia, "Arabic", "Saudi Arabia"),
    (0x2801, Language::Arabic_Syria, "Arabic", "Syria"),
    (0x1C01, Language::Arabic_Tunisia, "Arabic", "Tunisia"),
    (0x3801, Language::Arabic_UAE, "Arabic", "U.A.E."),
    (0x2401, Language::Arabic_Yemen, "Arabic", "Yemen"),
    (0x042B, Language::Armenian_Armenia, "Armenian", "Armenia"),
    (0x044D, Language::Assamese_India, "Assamese", "India"),
    (0x082C, Language::Azeri_Cyrillic_Azerbaijan, "Azeri (Cyrillic)", "Azerbaijan"),
    (0x042C, Language::Azeri_Latin_Azerbaijan, "Azeri (Latin)", "Azerbaijan"),
    (0x046D, Language::Bashkir_Russia, "Bashkir", "Russia"),
    (0x042D, Language::Basque_Basque, "Basque", "Basque"),
    (0x0423, Language::Belarusian_Belarus, "Belarusian", "Belarus"),
    (0x0845, Language::Bengali_Bangladesh, "Bengali", "Bangladesh"),
    (0x0445, Language::Bengali_India, "Bengali", "India"),
    (0x201A, Language::Bosnian_Cyrillic_BosniaAndHerzegovina, "Bosnian (Cyrillic)", "Bosnia and Herzegovina"),
    (0x141A, Language::Bosnian_Latin_BosniaAndHerzegovina, "Bosnian (Latin)", "Bosnia and Herzegovina"),
    (0x047E, Language::Breton_France, "Breton", "France"),
    (0x0402, Language::Bulgarian_Bulgaria, "Bulgarian", "Bulgaria"),
    (0x0403, Language::Catalan_Catalan, "Catalan", "Catalan"),
    (0x0C04, Language::Chinese_HongKongSAR, "Chinese", "Hong Kong S.A.R."),
    (0x1404, Language::Chinese_MacaoSAR, "Chinese", "Macao S.A.R."),
    (0x0804, Language::Chinese_PeoplesRepublicOfChina, "Chinese", "People's Republic of China"),
    (0x1004, Language::Chinese_Singapore, "Chinese", "Singapore"),
    (0x0404, Language::Chinese_Taiwan, "Chinese", "Taiwan"),
    (0x0483, Language::Corsican_France, "Corsican", "France"),
    (0x041A, Language::Croatian_Croatia, "Croatian", "Croatia"),
    (0x101A, Language::Croatian_Latin_BosniaAndHerzegovina, "Croatian (Latin)", "Bosnia and Herzegovina"),
    (0x0405, Language::Czech_CzechRepublic, "Czech", "Czech Republic"),
    (0x0406, Language::Danish_Denmark, "Danish", "Denmark"),
    (0x048C, Language::Dari_Afghanistan, "Dari", "Afghanistan"),
    (0x0465, Language::Divehi_Maldives, "Divehi", "Maldives"),
    (0x0813, Language::Dutch_Belgium, "Dutch", "Belgium"),
    (0x0413, Language::Dutch_Netherlands, "Dutch", "Netherlands"),
    (0x0C09, Language::English_Australia, "English", "Australia"),
    (0x2809, Language::English_Belize, "English", "Belize"),
    (0x1009, Language::English_Canada, "English", "Canada"),
    (0x2409, Language::English_Caribbean, "English", "Caribbean"),
    (0x4009, Language::English_India, "English", "India"),
    (0x1809, Language::English_Ireland, "English", "Ireland"),
    (0x2009, Language::English_Jamaica, "English", "Jamaica"),
    (0x4409, Language::English_Malaysia, "English", "Malaysia"),
    (0x1409, Language::English_NewZealand, "English", "New Zealand"),
    (0x3409, Language::English_RepublicOfThePhilippines, "English", "Republic of the Philippines"),
    (0x4809, Language::English_Singapore, "English", "Singapore"),
    (0x1C09, Language::English_SouthAfrica, "English", "South Africa"),
    (0x2C09, Language::English_TrinidadAndTobago, "English", "Trinidad and Tobago"),
    (0x0809, Language::English_UnitedKingdom, "English", "United Kingdom"),
    (0x0409, Language::English_UnitedStates, "English", "United States"),
    (0x3009, Language::English_Zimbabwe, "English", "Zimbabwe"),
    (0x0425, Language::Estonian_Estonia, "Estonian", "Estonia"),
    (0x0438, Language::Faroese_FaroeIslands, "Faroese", "Faroe Islands"),
    (0x0464, Language::Filipino_Philippines, "Filipino", "Philippines"),
    (0x040B, Language::Finnish_Finland, "Finnish", "Finland"),
    (0x080C, Language::French_Belgium, "French", "Belgium"),
    (0x0C0C, Language::French_Canada, "French", "Canada"),
    (0x040C, Language::French_France, "French", "France"),
    (0x140c, Language::French_Luxembourg, "French", "Luxembourg"),
    (0x180C, Language::French_PrincipalityOfMonaco, "French", "Principality of Monaco"),
    (0x100C, Language::French_Switzerland, "French", "Switzerland"),
    (0x0462, Language::Frisian_Netherlands, "Frisian", "Netherlands"),
    (0x0456, Language::Galician_Galician, "Galician", "Galician"),
    (0x0437, Language::Georgian_Georgia, "Georgian", "Georgia"),
    (0x0C07, Language::German_Austria, "German", "Austria"),
    (0x0407, Language::German_Germany, "German", "Germany"),
    (0x1407, Language::German_Liechtenstein, "German", "Liechtenstein"),
    (0x1007, Language::German_Luxembourg, "German", "Luxembourg"),
    (0x0807, Language::German_Switzerland, "German", "Switzerland"),
    (0x0408, Language::Greek_Greece, "Greek", "Greece"),
    (0x046F, Language::Greenlandic_Greenland, "Greenlandic", "Greenland"),
    (0x0447, Language::Gujarati_India, "Gujarati", "India"),
    (0x0468, Language::Hausa_Latin_Nigeria, "Hausa (Latin)", "Nigeria"),
    (0x040D, Language::Hebrew_Israel, "Hebrew", "Israel"),
    (0x0439, Language::Hindi_India, "Hindi", "India"),
    (0x040E, Language::Hungarian_Hungary, "Hungarian", "Hungary"),
    (0x040F, Language::Icelandic_Iceland, "Icelandic", "Iceland"),
    (0x0470, Language::Igbo_Nigeria, "Igbo", "Nigeria"),
    (0x0421, Language::Indonesian_Indonesia, "Indonesian", "Indonesia"),
    (0x045D, Language::Inuktitut_Canada, "Inuktitut", "Canada"),
    (0x085D, Language::Inuktitut_Latin_Canada, "Inuktitut (Latin)", "Canada"),
    (0x083C, Language::Irish_Ireland, "Irish", "Ireland"),
    (0x0434, Language::isiXhosa_SouthAfrica, "isiXhosa", "South Africa"),
    (0x0435, Language::isiZulu_SouthAfrica, "isiZulu", "South Africa"),
    (0x0410, Language::Italian_Italy, "Italian", "Italy"),
    (0x0810, Language::Italian_Switzerland, "Italian", "Switzerland"),
    (0x0411, Language::Japanese_Japan, "Japanese", "Japan"),
    (0x044B, Language::Kannada_India, "Kannada", "India"),
    (0x043F, Language::Kazakh_Kazakhstan, "Kazakh", "Kazakhstan"),
    (0x0453, Language::Khmer_Cambodia, "Khmer", "Cambodia"),
    (0x0486, Language::Kiche_Guatemala, "K'iche", "Guatemala"),
    (0x0487, Language::Kinyarwanda_Rwanda, "Kinyarwanda", "Rwanda"),
    (0x0441, Language::Kiswahili_Kenya, "Kiswahili", "Kenya"),
    (0x0457, Language::Konkani_India, "Konkani", "India"),
    (0x0412, Language::Korean_Korea, "Korean", "Korea"),
    (0x0440, Language::Kyrgyz_Kyrgyzstan, "Kyrgyz", "Kyrgyzstan"),
    (0x0454, Language::Lao_LaoPDR, "Lao", "Lao P.D.R."),
    (0x0426, Language::Latvian_Latvia, "Latvian", "Latvia"),
    (0x0427, Language::Lithuanian_Lithuania, "Lithuanian", "Lithuania"),
    (0x082E, Language::LowerSorbian_Germany, "Lower Sorbian", "Germany"),
    (0x046E, Language::Luxembourgish_Luxembourg, "Luxembourgish", "Luxembourg"),
    (0x042F, Language::Macedonian_NorthMacedonia, "Macedonian", "North Macedonia"),
    (0x083E, Language::Malay_BruneiDarussalam, "Malay", "Brunei Darussalam"),
    (0x043E, Language::Malay_Malaysia, "Malay", "Malaysia"),
    (0x044C, Language::Malayalam_India, "Malayalam", "India"),
    (0x043A, Language::Maltese_Malta, "Maltese", "Malta"),
    (0x0481, Language::Maori_NewZealand, "Maori", "New Zealand"),
    (0x047A, Language::Mapudungun_Chile, "Mapudungun", "Chile"),
    (0x044E, Language::Marathi_India, "Marathi", "India"),
    (0x047C, Language::Mohawk_Mohawk, "Mohawk", "Mohawk"),
    (0x0450, Language::Mongolian_Cyrillic_Mongolia, "Mongolian (Cyrillic)", "Mongolia"),
    (0x0850, Language::Mongolian_Traditional_PeoplesRepublicOfChina, "Mongolian (Traditional)", "People's Republic of China"),
    (0x0461, Language::Nepali_Nepal, "Nepali", "Nepal"),
    (0x0414, Language::Norwegian_Bokmal_Norway, "Norwegian (Bokmal)", "Norway"),
    (0x0814, Language::Norwegian_Nynorsk_Norway, "Norwegian (Nynorsk)", "Norway"),
    (0x0482, Language::Occitan_France, "Occitan", "France"),
    (0x0448, Language::Odia_India, "Odia (formerly Oriya)", "India"),
    (0x0463, Language::Pashto_Afghanistan, "Pashto", "Afghanistan"),
    (0x0415, Language::Polish_Poland, "Polish", "Poland"),
    (0x0416, Language::Portuguese_Brazil, "Portuguese", "Brazil"),
    (0x0816, Language::Portuguese_Portugal, "Portuguese", "Portugal"),
    (0x0446, Language::Punjabi_India, "Punjabi", "India"),
    (0x046B, Language::Quechua_Bolivia, "Quechua", "Bolivia"),
    (0x086B, Language::Quechua_Ecuador, "Quechua", "Ecuador"),
    (0x0C6B, Language::Quechua_Peru, "Quechua", "Peru"),
    (0x0418, Language::Romanian_Romania, "Romanian", "Romania"),
    (0x0417, Language::Romansh_Switzerland, "Romansh", "Switzerland"),
    (0x0419, Language::Russian_Russia, "Russian", "Russia"),
    (0x243B, Language::Sami_Inari_Finland, "Sami (Inari)", "Finland"),
    (0x103B, Language::Sami_Lule_Norway, "Sami (Lule)", "Norway"),
    (0x143B, Language::Sami_Lule_Sweden, "Sami (Lule)", "Sweden"),
    (0x0C3B, Language::Sami_Northern_Finland, "Sami (Northern)", "Finland"),
    (0x043B, Language::Sami_Northern_Norway, "Sami (Northern)", "Norway"),
    (0x083B, Language::Sami_Northern_Sweden, "Sami (Northern)", "Sweden"),
    (0x203B, Language::Sami_Skolt_Finland, "Sami (Skolt)", "Finland"),
    (0x183B, Language::Sami_Southern_Norway, "Sami (Southern)", "Norway"),
    (0x1C3B, Language::Sami_Southern_Sweden, "Sami (Southern)", "Sweden"),
    (0x044F, Language::Sanskrit_India, "Sanskrit", "India"),
    (0x1C1A, Language::Serbian_Cyrillic_BosniaAndHerzegovina, "Serbian (Cyrillic)", "Bosnia and Herzegovina"),
    (0x0C1A, Language::Serbian_Cyrillic_Serbia, "Serbian (Cyrillic)", "Serbia"),
    (0x181A, Language::Serbian_Latin_BosniaAndHerzegovina, "Serbian (Latin)", "Bosnia and Herzegovina"),
    (0x081A, Language::Serbian_Latin_Serbia, "Serbian (Latin)", "Serbia"),
    (0x046C, Language::SesothoSaLeboa_SouthAfrica, "Sesotho sa Leboa", "South Africa"),
    (0x0432, Language::Setswana_SouthAfrica, "Setswana", "South Africa"),
    (0x045B, Language::Sinhala_SriLanka, "Sinhala", "Sri Lanka"),
    (0x041B, Language::Slovak_Slovakia, "Slovak", "Slovakia"),
    (0x0424, Language::Slovenian_Slovenia, "Slovenian", "Slovenia"),
    (0x2C0A, Language::Spanish_Argentina, "Spanish", "Argentina"),
    (0x400A, Language::Spanish_Bolivia, "Spanish", "Bolivia"),
    (0x340A, Language::Spanish_Chile, "Spanish", "Chile"),
    (0x240A, Language::Spanish_Colombia, "Spanish", "Colombia"),
    (0x140A, Language::Spanish_CostaRica, "Spanish", "Costa Rica"),
    (0x1C0A, Language::Spanish_DominicanRepublic, "Spanish", "Dominican Republic"),
    (0x300A, Language::Spanish_Ecuador, "Spanish", "Ecuador"),
    (0x440A, Language::Spanish_ElSalvador, "Spanish", "El Salvador"),
    (0x100A, Language::Spanish_Guatemala, "Spanish", "Guatemala"),
    (0x480A, Language::Spanish_Honduras, "Spanish", "Honduras"),
    (0x080A, Language::Spanish_Mexico, "Spanish", "Mexico"),
    (0x4C0A, Language::Spanish_Nicaragua, "Spanish", "Nicaragua"),
    (0x180A, Language::Spanish_Panama, "Spanish", "Panama"),
    (0x3C0A, Language::Spanish_Paraguay, "Spanish", "Paraguay"),
    (0x280A, Language::Spanish_Peru, "Spanish", "Peru"),
    (0x500A, Language::Spanish_PuertoRico, "Spanish", "Puerto Rico"),
    (0x0C0A, Language::Spanish_ModernSort_Spain, "Spanish (Modern Sort)", "Spain"),
    (0x040A, Language::Spanish_TraditionalSort_Spain, "Spanish (Traditional Sort)", "Spain"),
    (0x540A, Language::Spanish_UnitedStates, "Spanish", "United States"),
    (0x380A, Language::Spanish_Uruguay, "Spanish", "Uruguay"),
    (0x200A, Language::Spanish_Venezuela, "Spanish", "Venezuela"),
    (0x081D, Language::Swedish_Finland, "Swedish", "Finland"),
    (0x041D, Language::Swedish_Sweden, "Swedish", "Sweden"),
    (0x045A, Language::Syriac_Syria, "Syriac", "Syria"),
    (0x0428, Language::Tajik_Cyrillic_Tajikistan, "Tajik (Cyrillic)", "Tajikistan"),
    (0x085F, Language::Tamazight_Latin_Algeria, "Tamazight (Latin)", "Algeria"),
    (0x0449, Language::Tamil_India, "Tamil", "India"),
    (0x0444, Language::Tatar_Russia, "Tatar", "Russia"),
    (0x044A, Language::Telugu_India, "Telugu", "India"),
    (0x041E, Language::Thai_Thailand, "Thai", "Thailand"),
    (0x0451, Language::Tibetan_PRC, "Tibetan", "PRC"),
    (0x041F, Language::Turkish_Turkey, "Turkish", "Turkey"),
    (0x0442, Language::Turkmen_Turkmenistan, "Turkmen", "Turkmenistan"),
    (0x0480, Language::Uighur_PRC, "Uighur", "PRC"),
    (0x0422, Language::Ukrainian_Ukraine, "Ukrainian", "Ukraine"),
    (0x042E, Language::UpperSorbian_Germany, "Upper Sorbian", "Germany"),
    (0x0420, Language::Urdu_IslamicRepublicOfPakistan, "Urdu", "Islamic Republic of Pakistan"),
    (0x0843, Language::Uzbek_Cyrillic_Uzbekistan, "Uzbek (Cyrillic)", "Uzbekistan"),
    (0x0443, Language::Uzbek_Latin_Uzbekistan, "Uzbek (Latin)", "Uzbekistan"),
    (0x042A, Language::Vietnamese_Vietnam, "Vietnamese", "Vietnam"),
    (0x0452, Language::Welsh_UnitedKingdom, "Welsh", "United Kingdom"),
    (0x0488, Language::Wolof_Senegal, "Wolof", "Senegal"),
    (0x0485, Language::Yakut_Russia, "Yakut", "Russia"),
    (0x0478, Language::Yi_PRC, "Yi", "PRC"),
    (0x046A, Language::Yoruba_Nigeria, "Yoruba", "Nigeria"),
];

/// A [`Name`] language.
///
/// Consists of Language + Region pairs.
///
/// https://learn.microsoft.com/en-us/typography/opentype/spec/name#windows-language-ids
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Language {
    Unknown = 0,
    Afrikaans_SouthAfrica,
    Albanian_Albania,
    Alsatian_France,
    Amharic_Ethiopia,
    Arabic_Algeria,
    Arabic_Bahrain,
    Arabic_Egypt,
    Arabic_Iraq,
    Arabic_Jordan,
    Arabic_Kuwait,
    Arabic_Lebanon,
    Arabic_Libya,
    Arabic_Morocco,
    Arabic_Oman,
    Arabic_Qatar,
    Arabic_SaudiArabia,
    Arabic_Syria,
    Arabic_Tunisia,
    Arabic_UAE,
    Arabic_Yemen,
    Armenian_Armenia,
    Assamese_India,
    Azeri_Cyrillic_Azerbaijan,
    Azeri_Latin_Azerbaijan,
    Bashkir_Russia,
    Basque_Basque,
    Belarusian_Belarus,
    Bengali_Bangladesh,
    Bengali_India,
    Bosnian_Cyrillic_BosniaAndHerzegovina,
    Bosnian_Latin_BosniaAndHerzegovina,
    Breton_France,
    Bulgarian_Bulgaria,
    Catalan_Catalan,
    Chinese_HongKongSAR,
    Chinese_MacaoSAR,
    Chinese_PeoplesRepublicOfChina,
    Chinese_Singapore,
    Chinese_Taiwan,
    Corsican_France,
    Croatian_Croatia,
    Croatian_Latin_BosniaAndHerzegovina,
    Czech_CzechRepublic,
    Danish_Denmark,
    Dari_Afghanistan,
    Divehi_Maldives,
    Dutch_Belgium,
    Dutch_Netherlands,
    English_Australia,
    English_Belize,
    English_Canada,
    English_Caribbean,
    English_India,
    English_Ireland,
    English_Jamaica,
    English_Malaysia,
    English_NewZealand,
    English_RepublicOfThePhilippines,
    English_Singapore,
    English_SouthAfrica,
    English_TrinidadAndTobago,
    English_UnitedKingdom,
    English_UnitedStates,
    English_Zimbabwe,
    Estonian_Estonia,
    Faroese_FaroeIslands,
    Filipino_Philippines,
    Finnish_Finland,
    French_Belgium,
    French_Canada,
    French_France,
    French_Luxembourg,
    French_PrincipalityOfMonaco,
    French_Switzerland,
    Frisian_Netherlands,
    Galician_Galician,
    Georgian_Georgia,
    German_Austria,
    German_Germany,
    German_Liechtenstein,
    German_Luxembourg,
    German_Switzerland,
    Greek_Greece,
    Greenlandic_Greenland,
    Gujarati_India,
    Hausa_Latin_Nigeria,
    Hebrew_Israel,
    Hindi_India,
    Hungarian_Hungary,
    Icelandic_Iceland,
    Igbo_Nigeria,
    Indonesian_Indonesia,
    Inuktitut_Canada,
    Inuktitut_Latin_Canada,
    Irish_Ireland,
    isiXhosa_SouthAfrica,
    isiZulu_SouthAfrica,
    Italian_Italy,
    Italian_Switzerland,
    Japanese_Japan,
    Kannada_India,
    Kazakh_Kazakhstan,
    Khmer_Cambodia,
    Kiche_Guatemala,
    Kinyarwanda_Rwanda,
    Kiswahili_Kenya,
    Konkani_India,
    Korean_Korea,
    Kyrgyz_Kyrgyzstan,
    Lao_LaoPDR,
    Latvian_Latvia,
    Lithuanian_Lithuania,
    LowerSorbian_Germany,
    Luxembourgish_Luxembourg,
    Macedonian_NorthMacedonia,
    Malay_BruneiDarussalam,
    Malay_Malaysia,
    Malayalam_India,
    Maltese_Malta,
    Maori_NewZealand,
    Mapudungun_Chile,
    Marathi_India,
    Mohawk_Mohawk,
    Mongolian_Cyrillic_Mongolia,
    Mongolian_Traditional_PeoplesRepublicOfChina,
    Nepali_Nepal,
    Norwegian_Bokmal_Norway,
    Norwegian_Nynorsk_Norway,
    Occitan_France,
    Odia_India,
    Pashto_Afghanistan,
    Polish_Poland,
    Portuguese_Brazil,
    Portuguese_Portugal,
    Punjabi_India,
    Quechua_Bolivia,
    Quechua_Ecuador,
    Quechua_Peru,
    Romanian_Romania,
    Romansh_Switzerland,
    Russian_Russia,
    Sami_Inari_Finland,
    Sami_Lule_Norway,
    Sami_Lule_Sweden,
    Sami_Northern_Finland,
    Sami_Northern_Norway,
    Sami_Northern_Sweden,
    Sami_Skolt_Finland,
    Sami_Southern_Norway,
    Sami_Southern_Sweden,
    Sanskrit_India,
    Serbian_Cyrillic_BosniaAndHerzegovina,
    Serbian_Cyrillic_Serbia,
    Serbian_Latin_BosniaAndHerzegovina,
    Serbian_Latin_Serbia,
    SesothoSaLeboa_SouthAfrica,
    Setswana_SouthAfrica,
    Sinhala_SriLanka,
    Slovak_Slovakia,
    Slovenian_Slovenia,
    Spanish_Argentina,
    Spanish_Bolivia,
    Spanish_Chile,
    Spanish_Colombia,
    Spanish_CostaRica,
    Spanish_DominicanRepublic,
    Spanish_Ecuador,
    Spanish_ElSalvador,
    Spanish_Guatemala,
    Spanish_Honduras,
    Spanish_Mexico,
    Spanish_Nicaragua,
    Spanish_Panama,
    Spanish_Paraguay,
    Spanish_Peru,
    Spanish_PuertoRico,
    Spanish_ModernSort_Spain,
    Spanish_TraditionalSort_Spain,
    Spanish_UnitedStates,
    Spanish_Uruguay,
    Spanish_Venezuela,
    Swedish_Finland,
    Swedish_Sweden,
    Syriac_Syria,
    Tajik_Cyrillic_Tajikistan,
    Tamazight_Latin_Algeria,
    Tamil_India,
    Tatar_Russia,
    Telugu_India,
    Thai_Thailand,
    Tibetan_PRC,
    Turkish_Turkey,
    Turkmen_Turkmenistan,
    Uighur_PRC,
    Ukrainian_Ukraine,
    UpperSorbian_Germany,
    Urdu_IslamicRepublicOfPakistan,
    Uzbek_Cyrillic_Uzbekistan,
    Uzbek_Latin_Uzbekistan,
    Vietnamese_Vietnam,
    Welsh_UnitedKingdom,
    Wolof_Senegal,
    Yakut_Russia,
    Yi_PRC,
    Yoruba_Nigeria,
}

impl Language {
    pub(crate) fn windows_language(id: u16) -> Self {
        if let Some(index) = TABLE.iter().position(|v| v.0 == id) {
            TABLE[index].1
        } else {
            Self::Unknown
        }
    }

    /// Returns the primary language.
    pub fn primary_language(&self) -> &'static str {
        TABLE[*self as usize].2
    }

    /// Returns a language region.
    pub fn region(&self) -> &'static str {
        TABLE[*self as usize].3
    }
}

impl core::fmt::Display for Language {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} ({})", self.primary_language(), self.region())
    }
}
