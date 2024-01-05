#[derive(Clone, Copy)]
enum BitInfo {
    Unsigned {
        prim_max: u64,
    },
    Signed {
        signed_max: i64,
        signed_min: i64,
        prim_max: i64,
    },
}

struct EnumInfo {
    ty_name: &'static str,
    name: &'static str,
    bit_info: BitInfo,
}

const FILES: [EnumInfo; 4] = [
    EnumInfo {
        ty_name: "u8",
        name: "u8_repr.rs",
        bit_info: BitInfo::Unsigned {
            prim_max: u8::MAX as _,
        },
    },
    EnumInfo {
        ty_name: "i8",
        name: "i8_repr.rs",
        bit_info: BitInfo::Signed {
            signed_max: i8::MAX as _,
            signed_min: i8::MIN as _,
            prim_max: u8::MAX as _,
        },
    },
    EnumInfo {
        ty_name: "u16",
        name: "u16_repr.rs",
        bit_info: BitInfo::Unsigned {
            prim_max: u16::MAX as _,
        },
    },
    EnumInfo {
        ty_name: "i16",
        name: "i16_repr.rs",
        bit_info: BitInfo::Signed {
            signed_max: i16::MAX as _,
            signed_min: i16::MIN as _,
            prim_max: u16::MAX as _,
        },
    },
];

fn generate_variants(
    generated_file: &mut impl std::fmt::Write,
    repr_name: &str,
    bit_info: BitInfo,
) {
    write!(
        generated_file,
        "#[derive(Clone, Copy, PartialEq, Eq, Hash)]
        pub(crate) enum {repr_name} {{",
    )
    .unwrap();

    match bit_info {
        BitInfo::Unsigned { prim_max } => {
            for i in 0..prim_max {
                write!(generated_file, "V{i}={i},").unwrap()
            }
        }
        BitInfo::Signed {
            signed_max,
            signed_min,
            prim_max,
        } => {
            for i in 0..signed_max {
                write!(generated_file, "V{i}={i},").unwrap();
            }

            for (i, v) in (signed_max..prim_max).zip(signed_min..0) {
                write!(generated_file, "MV{i}={v},").unwrap();
            }
        }
    }

    write!(generated_file, "}}").unwrap();
}

fn generate_impl(generated_file: &mut impl std::fmt::Write, repr_name: &str, ty_name: &str) {
    write!(
        generated_file,
        "impl {repr_name} {{
            pub(crate) const fn new(value: {ty_name}) -> Option<Self> {{
                unsafe {{ std::mem::transmute(value) }}
            }}
        }}"
    )
    .unwrap()
}

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    for file in FILES {
        let mut generated_file =
            concat!("//! This file is @generated by a build script.\n").to_owned();

        let repr_name = format!("{}Repr", file.ty_name.to_uppercase());

        generate_variants(&mut generated_file, &repr_name, file.bit_info);
        generate_impl(&mut generated_file, &repr_name, file.ty_name);

        std::fs::write(format!("{out_dir}/{}", file.name), generated_file).unwrap();
    }
}
