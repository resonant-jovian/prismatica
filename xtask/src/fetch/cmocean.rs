use crate::codegen::{self, ColormapMeta};
use std::path::Path;

struct CmoceanMapDef {
    name: &'static str,
    kind: &'static str,
    grayscale_safe: bool,
}

const CMOCEAN_MAPS: &[CmoceanMapDef] = &[
    CmoceanMapDef {
        name: "thermal",
        kind: "Sequential",
        grayscale_safe: true,
    },
    CmoceanMapDef {
        name: "haline",
        kind: "Sequential",
        grayscale_safe: true,
    },
    CmoceanMapDef {
        name: "solar",
        kind: "Sequential",
        grayscale_safe: true,
    },
    CmoceanMapDef {
        name: "ice",
        kind: "Sequential",
        grayscale_safe: true,
    },
    CmoceanMapDef {
        name: "gray",
        kind: "Sequential",
        grayscale_safe: true,
    },
    CmoceanMapDef {
        name: "oxy",
        kind: "Sequential",
        grayscale_safe: true,
    },
    CmoceanMapDef {
        name: "deep",
        kind: "Sequential",
        grayscale_safe: true,
    },
    CmoceanMapDef {
        name: "dense",
        kind: "Sequential",
        grayscale_safe: true,
    },
    CmoceanMapDef {
        name: "algae",
        kind: "Sequential",
        grayscale_safe: true,
    },
    CmoceanMapDef {
        name: "matter",
        kind: "Sequential",
        grayscale_safe: true,
    },
    CmoceanMapDef {
        name: "turbid",
        kind: "Sequential",
        grayscale_safe: true,
    },
    CmoceanMapDef {
        name: "speed",
        kind: "Sequential",
        grayscale_safe: true,
    },
    CmoceanMapDef {
        name: "amp",
        kind: "Sequential",
        grayscale_safe: true,
    },
    CmoceanMapDef {
        name: "tempo",
        kind: "Sequential",
        grayscale_safe: true,
    },
    CmoceanMapDef {
        name: "rain",
        kind: "Sequential",
        grayscale_safe: true,
    },
    CmoceanMapDef {
        name: "phase",
        kind: "Cyclic",
        grayscale_safe: false,
    },
    CmoceanMapDef {
        name: "topo",
        kind: "Sequential",
        grayscale_safe: true,
    },
    CmoceanMapDef {
        name: "balance",
        kind: "Diverging",
        grayscale_safe: false,
    },
    CmoceanMapDef {
        name: "delta",
        kind: "Diverging",
        grayscale_safe: false,
    },
    CmoceanMapDef {
        name: "curl",
        kind: "Diverging",
        grayscale_safe: false,
    },
    CmoceanMapDef {
        name: "diff",
        kind: "Diverging",
        grayscale_safe: false,
    },
    CmoceanMapDef {
        name: "tarn",
        kind: "Diverging",
        grayscale_safe: false,
    },
];

pub fn fetch(project_root: &Path) {
    println!("Fetching CMOcean from GitHub...");

    let data_dir = project_root.join("data").join("cmocean");
    std::fs::create_dir_all(&data_dir).expect("create data/cmocean/");

    for def in CMOCEAN_MAPS {
        println!("  Fetching {}...", def.name);

        let url = format!(
            "https://raw.githubusercontent.com/matplotlib/cmocean/main/cmocean/rgb/{}-rgb.txt",
            def.name
        );

        let text = codegen::fetch_url_text(&url);
        let lut = codegen::parse_space_separated_floats(&text);

        if lut.len() != 256 {
            eprintln!(
                "  Warning: {} has {} rows, expected 256 -- resampling",
                def.name,
                lut.len()
            );
        }

        let lut = if lut.len() == 256 {
            lut
        } else {
            codegen::resample(&lut, 256)
        };

        let meta = ColormapMeta {
            name: def.name.to_string(),
            collection: "cmocean".to_string(),
            author: "Kristen Thyng".to_string(),
            kind: def.kind.to_string(),
            perceptually_uniform: true,
            cvd_friendly: true,
            grayscale_safe: def.grayscale_safe,
            citation:
                "Thyng, K. M. et al. (2016). True colors of oceanography. Oceanography, 29(3), 10."
                    .to_string(),
        };

        codegen::write_csv(&data_dir.join(format!("{}.csv", def.name)), &lut);
        codegen::write_json(&data_dir.join(format!("{}.json", def.name)), &meta);
        println!(
            "  Wrote data/cmocean/{}.csv + .json ({})",
            def.name, def.kind
        );
    }
}
