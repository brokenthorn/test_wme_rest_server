use std::collections::HashMap;

use tracing::{info, Level};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub mod tva {
    //! Particularitati TVA intrari si iesiri.

    pub mod tip_tranzactie {
        pub mod intrari {
            pub const TRANZACTIE_INTERNA: i8 = 1;
            pub const ACHIZITIE_INTRACOMUNITARA: i8 = 2;
            pub const IMPORT_SERVICII: i8 = 3;
            pub const FACTURA_DE_TRANSPORT_TAXABILA_PE_DVI: i8 = 4;
        }

        pub mod invoice {
            pub const ACHIZITIE_INTRACOMUNITARA: i8 = 2;
            pub const IMPORT_BUNURI_SI_SERVICII: i8 = 3;
            pub const FACTURA_DE_TRANSPORT_TAXABILA_PE_DVI: i8 = 4;
        }

        pub mod iesiri {
            pub const TRANZACTIE_INTERNA: i8 = 1;
            pub const LIVRARE_INTRACOMUNITARA: i8 = 2;
            pub const EXPORT: i8 = 3;
            pub const INTERNA_AUTOFACTURARE: i8 = 4;
        }
    }

    pub mod tip_tva {
        pub mod intrari {
            pub const TAXARE_NORMALA: i8 = 1;
            pub const TAXARE_INVERSA: i8 = 2;
            pub const TRANZACTIE_TRIUNGHIULARA: i8 = 3;
            pub const TAXARE_NORMALA_PRORATA: i8 = 4;
            pub const REGIM_SPECIAL_ART1521_1522: i8 = 5;
            pub const REGIM_SPECIAL_DE_SCUTIRE_ART311_CF: i8 = 6;
            pub const REGIM_SPECIAL_DE_SCUTIRE_ART312_CF: i8 = 7;
            pub const ACHIZITII_UE_BUNURI_CU_INSTALARE_MONTAJ: i8 = 8;
        }

        pub mod invoice {
            pub const TAXARE_NORMALA: i8 = 1;
            pub const TAXARE_INVERSA: i8 = 2;
            pub const TRANZACTIE_TRIUNGHIULARA: i8 = 3;
            pub const TAXARE_NORMALA_PRORATA: i8 = 4;
        }

        pub mod iesiri {
            pub const TAXARE_NORMALA: i8 = 1;
            pub const TAXARE_INVERSA: i8 = 2;
            pub const TRANZACTIE_TRIUNGHIULARA: i8 = 3;
            pub const LOCUL_LIVRARII_PRESTARII_IN_AFARA_ROMANIEI: i8 = 4;
            pub const INTRACOMUNITAR_SCUTIT_CU_DREPT_DE_DEDUCERE_LIT_A_D: i8 = 5;
            pub const INTRACOMUNITAR_SCUTIT_CU_DREPT_DE_DEDUCERE_LIT_B_C: i8 = 6;
            pub const REGIM_SPECIAL_ART1521_1522: i8 = 7;
        }
    }
}

pub mod mod_plata {
    pub const NUMERAR: i8 = 1;
    pub const ORDIN_PLATA: i8 = 2;
    pub const CEC: i8 = 3;
    pub const BILET_LA_ORDIN: i8 = 4;
    pub const COMPENSARE: i8 = 5;
    pub const MAJORARI: i8 = 6;
    pub const CEC_BO_GIRAT: i8 = 7;
    pub const BO_AVALIZAT: i8 = 8;

    pub const MODALITATI_PLATA: [i8; 8] = [
        NUMERAR,
        ORDIN_PLATA,
        CEC,
        BILET_LA_ORDIN,
        COMPENSARE,
        MAJORARI,
        CEC_BO_GIRAT,
        BO_AVALIZAT,
    ];

    pub const MONEDE_ACCEPTATE: [&str; 3] = ["RON", "USD", "EUR"];
}

pub mod tip_document {
    pub const FACTURA_INTRARE: &str = "FACTURA INTRARE";
    pub const AVIZ_INTRARE: &str = "AVIZ INTRARE";
    pub const INVOICE: &str = "INVOICE";
    pub const FACTURA_IN_ASTEPTARE: &str = "FACTURA IN ASTEPTARE";
    pub const AVIZ_LA_FACTURA_IN_ASTEPTARE: &str = "AVIZ LA FACTURA IN ASTEPTARE";
    pub const FACTURA_LA_AVIZ: &str = "FACTURA LA AVIZ";
    pub const BON_FISCAL: &str = "BON FISCAL";

    pub const TIPURI_DOCUMENT: [&str; 7] = [
        FACTURA_INTRARE,
        AVIZ_INTRARE,
        INVOICE,
        FACTURA_IN_ASTEPTARE,
        AVIZ_LA_FACTURA_IN_ASTEPTARE,
        FACTURA_LA_AVIZ,
        BON_FISCAL,
    ];
}

pub mod models {
    use derive_builder::Builder;
    use serde::Serialize;

    use crate::mod_plata::MONEDE_ACCEPTATE;
    use crate::tip_document::TIPURI_DOCUMENT;

    #[derive(Debug, Clone, Serialize)]
    pub enum DN {
        Da,
        Nu,
    }

    #[derive(Debug, Clone, Builder, Serialize)]
    pub struct Scadenta {
        #[serde(rename = "Valoare")]
        pub valoare: Option<String>,
        pub termen: Option<String>,
        pub mod_plata: Option<String>,
        pub simbol_centru_cost: Option<String>,
    }

    #[derive(Debug, Clone, Builder, Serialize)]
    pub struct Serii {
        #[serde(rename = "Serie")]
        pub serie: Option<String>,
        #[serde(rename = "Cant")]
        pub cant: Option<String>,
        #[serde(rename = "Observatii")]
        pub observatii: Option<String>,
        #[serde(rename = "DataProd")]
        pub data_prod: Option<String>,
    }

    #[derive(Debug, Clone, Builder, Serialize)]
    pub struct ItemDocumentIntrareFurnizori {
        #[serde(rename = "IDArticol")]
        pub id_articol: Option<String>,
        #[serde(rename = "UM")]
        pub um: Option<String>,
        #[serde(rename = "Cant")]
        pub cant: Option<String>,
        #[serde(rename = "TVANeded")]
        pub tva_neded: Option<String>,
        #[serde(rename = "SimbolCentruCost")]
        pub simbol_centru_cost: Option<String>,
        #[serde(rename = "CodAnalizaNod")]
        pub cod_analiza_nod: Option<String>,
        #[serde(rename = "Observatii")]
        pub observatii: Option<String>,
        #[serde(rename = "NrAuto")]
        pub nr_auto: Option<String>,
        #[serde(rename = "Serii")]
        pub serii: Option<Vec<Serii>>,
        #[serde(rename = "Pret")]
        pub pret: Option<String>,
        #[serde(rename = "PretIntreg")]
        pub pret_intreg: Option<String>,
        #[serde(rename = "Gestiune")]
        pub gestiune: Option<String>,
        #[serde(rename = "LocatieGest")]
        pub locatie_gest: Option<String>,
        #[serde(rename = "Discount")]
        pub discount: Option<String>,
        #[serde(rename = "D1")]
        pub d1: Option<String>,
        #[serde(rename = "D2")]
        pub d2: Option<String>,
        #[serde(rename = "D3")]
        pub d3: Option<String>,
        #[serde(rename = "EXTENSIELINIE")]
        pub extensie_linie: Option<String>,
    }

    #[derive(Debug, Clone, Builder, Serialize)]
    #[builder(build_fn(validate = "Self::validate"))]
    pub struct DocumentIntrareFurnizori {
        pub serie_doc: Option<String>,
        pub nr_doc: Option<String>,
        pub nr_intreg: Option<String>,
        pub operat: Option<DN>,
        pub data: Option<String>,
        pub data_dvi: Option<String>,
        pub simbol_carnet_nir: Option<String>,
        pub nr_nir: Option<String>,
        pub data_nir: Option<String>,
        pub cod_furnizori: Option<String>,
        pub locatie: Option<String>,
        pub observatii: Option<String>,
        pub observatii_nir: Option<String>,
        pub autofacturare: Option<DN>,
        pub moneda: Option<String>,
        pub curs: Option<String>,
        pub tip_tranzactie: Option<String>,
        pub tva_la_incasare: Option<String>,
        pub tip_tva: Option<String>,
        pub cod_subunitate: Option<String>,
        pub scadenta: Option<String>,
        pub mod_plata: Option<String>,
        pub scadente: Option<Vec<Scadenta>>,
        pub extensie_document: Option<String>,
        pub items: Vec<ItemDocumentIntrareFurnizori>,
    }

    impl DocumentIntrareFurnizoriBuilder {
        pub fn validate(&self) -> Result<(), String> {
            if let Some(ref moneda_option) = self.moneda {
                if let Some(ref moneda) = moneda_option {
                    if !MONEDE_ACCEPTATE.contains(&moneda.as_str()) {
                        return Err(format!(
                            "{} nu este o moneda acceptata. Monedele acceptate sunt: {:?}.",
                            moneda, MONEDE_ACCEPTATE
                        ));
                    }
                }
            }

            if let Some(ref items) = self.items {
                if items.is_empty() {
                    return Err(
                        "Documentul nu are linii (campul `items` nu contine nici un element)."
                            .to_string(),
                    );
                }
            }

            Ok(())
        }
    }

    #[derive(Builder, Serialize)]
    #[builder(build_fn(validate = "Self::validate"))]
    pub struct IntrareFurnizori {
        pub tip_document: Option<String>,
        pub an_lucru: Option<String>,
        pub luna_lucru: Option<String>,
        pub documente: Vec<DocumentIntrareFurnizori>,
    }

    impl IntrareFurnizoriBuilder {
        pub fn validate(&self) -> Result<(), String> {
            if let Some(ref tip_document_option) = self.tip_document {
                if let Some(ref tip_document) = tip_document_option {
                    if !TIPURI_DOCUMENT.contains(&tip_document.as_str()) {
                        return Err(format!(
                            "{} nu este un tip de document acceptat. Tipurile de document acceptate sunt: {:?}",
                            tip_document, TIPURI_DOCUMENT
                        ));
                    }
                }
            }

            if let Some(ref documente) = self.documente {
                if documente.is_empty() {
                    return Err(
                        "Intrarea de la furnizori nu are documente (campul `documente` nu contine nici un element)."
                            .to_string()
                    );
                }
            }

            Ok(())
        }
    }
}

pub mod client {
    pub async fn update_intrari_furnizori() -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

#[tracing::instrument]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set default logging level(s):
    if std::env::var_os("RUST_LOG").is_none() {
        println!("RUST_LOG not found. Setting RUST_LOG=info.");
        std::env::set_var("RUST_LOG", "info");
    }

    FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE
        // (e.g, debug, info, warn, etc.) will be written to stdout.
        .with_max_level(Level::TRACE)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Starting up.");
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    info!("Response: {:?}", resp);
    Ok(())
}
