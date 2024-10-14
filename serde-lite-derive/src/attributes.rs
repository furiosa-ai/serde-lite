use syn::{
    parse::ParseStream, punctuated::Punctuated, token::Comma, Attribute, Expr, Field, Lit, Meta,
    Result, Variant,
};

/// Get the rename attribute for a given field or the field name.
pub fn get_field_name(field: &Field) -> String {
    if let Some(v) = get_attr_value(&field.attrs, "rename") {
        if let Some(Lit::Str(n)) = v.lit() {
            return n.value();
        } else {
            panic!("invalid rename attribute");
        }
    }

    field.ident.as_ref().unwrap().to_string()
}

/// Get the skip_serializing_if path for a given field (if present).
pub fn get_skip_field_serializing_if(field: &Field) -> Option<String> {
    if let Some(v) = get_attr_value(&field.attrs, "skip_serializing_if") {
        if let Some(Lit::Str(n)) = v.lit() {
            Some(n.value())
        } else {
            panic!("invalid skip_serializing_if attribute");
        }
    } else {
        None
    }
}

/// Get field serializer path (if present).
pub fn get_field_serializer(field: &Field) -> Option<String> {
    if let Some(v) = get_attr_value(&field.attrs, "serialize_with") {
        if let Some(Lit::Str(n)) = v.lit() {
            Some(n.value())
        } else {
            panic!("invalid serialize_with attribute");
        }
    } else {
        None
    }
}

/// Get field deserializer path (if present).
pub fn get_field_deserializer(field: &Field) -> Option<String> {
    if let Some(v) = get_attr_value(&field.attrs, "deserialize_with") {
        if let Some(Lit::Str(n)) = v.lit() {
            Some(n.value())
        } else {
            panic!("invalid deserialize_with attribute");
        }
    } else {
        None
    }
}

/// Get field updater path (if present).
pub fn get_field_updater(field: &Field) -> Option<String> {
    if let Some(v) = get_attr_value(&field.attrs, "update_with") {
        if let Some(Lit::Str(n)) = v.lit() {
            Some(n.value())
        } else {
            panic!("invalid update_with attribute");
        }
    } else {
        None
    }
}

/// Get enum tag attribute (if present).
pub fn get_enum_tag(attrs: &[Attribute]) -> Option<String> {
    if let Some(v) = get_attr_value(attrs, "tag") {
        if let Some(Lit::Str(n)) = v.lit() {
            Some(n.value())
        } else {
            panic!("invalid tag attribute");
        }
    } else {
        None
    }
}

/// Get enum content attribute (if present).
pub fn get_enum_content(attrs: &[Attribute]) -> Option<String> {
    if let Some(v) = get_attr_value(attrs, "content") {
        if let Some(Lit::Str(n)) = v.lit() {
            Some(n.value())
        } else {
            panic!("invalid content attribute");
        }
    } else {
        None
    }
}

/// Get `from` attribute (if present).
pub fn get_from(attrs: &[Attribute]) -> Option<String> {
    if let Some(v) = get_attr_value(attrs, "from") {
        if let Some(Lit::Str(n)) = v.lit() {
            Some(n.value())
        } else {
            panic!("invalid from attribute");
        }
    } else {
        None
    }
}

/// Get the rename attribute for a given enum variant or the variant name.
pub fn get_variant_name(variant: &Variant) -> String {
    if let Some(v) = get_attr_value(&variant.attrs, "rename") {
        if let Some(Lit::Str(n)) = v.lit() {
            return n.value();
        } else {
            panic!("invalid rename attribute");
        }
    }

    variant.ident.to_string()
}

/// Get value of a given attribute.
pub fn get_attr_value(attrs: &[Attribute], name: &str) -> Option<Expr> {
    for attr in attrs {
        if attr.path().is_ident("serde") {
            if let Ok(nested) = attr.parse_args_with(parse_nested_meta) {
                for meta in nested {
                    if let Meta::NameValue(a) = meta {
                        if a.path.is_ident(name) {
                            return Some(a.value);
                        }
                    }
                }
            }
        }
    }

    None
}

/// Check if a given attribute flag is present.
pub fn has_flag(attrs: &[Attribute], name: &str) -> bool {
    for attr in attrs {
        if attr.path().is_ident("serde") {
            if let Ok(nested) = attr.parse_args_with(parse_nested_meta) {
                for meta in nested {
                    if let Meta::Path(a) = meta {
                        if a.is_ident(name) {
                            return true;
                        }
                    }
                }
            }
        }
    }

    false
}

/// Helper trait.
trait ExprEx {
    /// Get the literal expression (if any).
    fn lit(&self) -> Option<&Lit>;
}

impl ExprEx for Expr {
    fn lit(&self) -> Option<&Lit> {
        if let Expr::Lit(l) = self {
            Some(&l.lit)
        } else {
            None
        }
    }
}

/// Helper function.
fn parse_nested_meta(stream: ParseStream) -> Result<Punctuated<Meta, Comma>> {
    Punctuated::parse_terminated(stream)
}
