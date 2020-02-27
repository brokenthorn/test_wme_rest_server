use tracing::{info, Level};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use crate::models::{DocumentIntrareFurnizori, IntrareFurnizoriBuilder};

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
    #[serde(rename_all = "PascalCase")]
    pub struct Scadenta {
        pub valoare: Option<String>,
        pub termen: Option<String>,
        pub mod_plata: Option<String>,
        pub simbol_centru_cost: Option<String>,
    }

    #[derive(Debug, Clone, Builder, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct Serii {
        pub serie: Option<String>,
        pub cant: Option<String>,
        pub observatii: Option<String>,
        pub data_prod: Option<String>,
    }

    #[derive(Debug, Clone, Builder, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct ItemDocumentIntrareFurnizori {
        #[serde(rename = "IDArticol")]
        pub id_articol: Option<String>,
        #[serde(rename = "UM")]
        pub um: Option<String>,
        pub cant: Option<String>,
        #[serde(rename = "TVANeded")]
        pub tva_neded: Option<String>,
        pub simbol_centru_cost: Option<String>,
        pub cod_analiza_nod: Option<String>,
        pub observatii: Option<String>,
        pub nr_auto: Option<String>,
        pub serii: Option<Vec<Serii>>,
        pub pret: Option<String>,
        pub pret_intreg: Option<String>,
        pub gestiune: Option<String>,
        pub locatie_gest: Option<String>,
        pub discount: Option<String>,
        pub d1: Option<String>,
        pub d2: Option<String>,
        pub d3: Option<String>,
        #[serde(rename = "EXTENSIELINIE")]
        pub extensie_linie: Option<String>,
    }

    #[derive(Debug, Clone, Builder, Serialize)]
    #[serde(rename_all = "PascalCase")]
    #[builder(build_fn(validate = "Self::validate"))]
    pub struct DocumentIntrareFurnizori {
        pub serie_doc: Option<String>,
        pub nr_doc: Option<String>,
        pub nr_intreg: Option<String>,
        pub operat: Option<DN>,
        pub data: Option<String>,
        #[serde(rename = "DataDVI")]
        pub data_dvi: Option<String>,
        #[serde(rename = "SimbolCarnetNIR")]
        pub simbol_carnet_nir: Option<String>,
        #[serde(rename = "NrNIR")]
        pub nr_nir: Option<String>,
        #[serde(rename = "DataNIR")]
        pub data_nir: Option<String>,
        pub cod_furnizori: Option<String>,
        pub locatie: Option<String>,
        pub observatii: Option<String>,
        #[serde(rename = "ObservatiiNIR")]
        pub observatii_nir: Option<String>,
        pub autofacturare: Option<DN>,
        pub moneda: Option<String>,
        pub curs: Option<String>,
        pub tip_tranzactie: Option<String>,
        #[serde(rename = "TVALaIncasare")]
        pub tva_la_incasare: Option<String>,
        #[serde(rename = "TipTVA")]
        pub tip_tva: Option<String>,
        pub cod_subunitate: Option<String>,
        pub scadenta: Option<String>,
        pub mod_plata: Option<String>,
        pub scadente: Option<Vec<Scadenta>>,
        #[serde(rename = "EXTENSIEDOCUMENT")]
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

    #[derive(Debug, Builder, Serialize)]
    #[builder(build_fn(validate = "Self::validate"))]
    pub struct IntrareFurnizori {
        #[serde(rename = "TipDocument")]
        pub tip_document: Option<String>,
        #[serde(rename = "AnLucru")]
        pub an_lucru: Option<String>,
        #[serde(rename = "LunaLucru")]
        pub luna_lucru: Option<String>,
        #[serde(rename = "Documente")]
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
    use tracing::info;

    use crate::models::IntrareFurnizori;

    pub async fn update_intrari_furnizori(
        client: &reqwest::Client,
        intrare: IntrareFurnizori,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!(
            "Se trimite o cerere de update intrare de la furnizori: {:?}",
            intrare
        );
        let response = client
            .post("http://10.0.0.132:8080/datasnap/rest/TServerMethods/IntrariFurnizori")
            .json(&intrare)
            .send()
            .await?;
        info!("S-a trimis intrarea de la furnizori: {:?}", response);
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

    info!("Incep testele.");
    let connect_timeout = std::time::Duration::from_secs(5);
    info!(
        "Timeout de conectare general setat la {:?}.",
        connect_timeout
    );

    let c = reqwest::ClientBuilder::new()
        .connection_verbose(true)
        .connect_timeout(connect_timeout)
        .build()?;

    let intrare = IntrareFurnizoriBuilder::default()
        .tip_document(Some(tip_document::FACTURA_INTRARE.into()))
        .an_lucru(Some("2020".to_string()))
        .luna_lucru(Some("02".to_string()))
        .documente(vec![DocumentIntrareFurnizori {
            serie_doc: None,
            nr_doc: None,
            nr_intreg: None,
            operat: None,
            data: None,
            data_dvi: None,
            simbol_carnet_nir: None,
            nr_nir: None,
            data_nir: None,
            cod_furnizori: None,
            locatie: None,
            observatii: None,
            observatii_nir: None,
            autofacturare: None,
            moneda: None,
            curs: None,
            tip_tranzactie: None,
            tva_la_incasare: None,
            tip_tva: None,
            cod_subunitate: None,
            scadenta: None,
            mod_plata: None,
            scadente: None,
            extensie_document: None,
            items: vec![],
        }])
        .build()?;

    client::update_intrari_furnizori(&c, intrare).await?;

    info!("Terminat cu succes.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intrare_furnizori_builder_fail_plain_build() {
        let result: Result<_, _> = models::IntrareFurnizoriBuilder::default().build();
        assert!(result.is_err());
    }
}
