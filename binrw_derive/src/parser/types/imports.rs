use crate::parser::{
    attrs,
    meta_types::{Enclosure, IdentTypeMaybeDefault},
    KeywordToken, TrySet,
};

use syn::{Ident, Type, spanned::Spanned};

#[derive(Debug, Clone)]
pub(crate) enum Imports {
    None,
    Raw(Ident, Box<Type>),
    List(Vec<Ident>, Vec<Type>),
    Named(Vec<IdentTypeMaybeDefault>),
}

impl Default for Imports {
    fn default() -> Self {
        Imports::None
    }
}

struct LifetimeReplacer;

impl syn::visit_mut::VisitMut for LifetimeReplacer {
    fn visit_lifetime_mut(&mut self, lifetime: &mut syn::Lifetime) {
        if lifetime.ident == "_" {
            lifetime.ident = syn::Ident::new("this", lifetime.span());
        }
    }

    fn visit_type_reference_mut(&mut self, reference: &mut syn::TypeReference) {
        if reference.lifetime.is_none() {
            reference.lifetime = Some(syn::Lifetime {
                apostrophe: reference.and_token.span(),
                ident: syn::Ident::new("this", reference.and_token.span())
            });
        }
    }
}

fn set_lifetime(mut ty: Type) -> Type {
    syn::visit_mut::visit_type_mut(&mut LifetimeReplacer, &mut ty);

    ty
}

impl From<attrs::Import> for Imports {
    fn from(value: attrs::Import) -> Self {
        match &value.list {
            Enclosure::Paren { fields, .. } => {
                if fields.is_empty() {
                    Self::None
                } else {
                    let (idents, tys) = fields
                        .iter()
                        .cloned()
                        .map(|field| (field.ident, set_lifetime(field.ty)))
                        .unzip();
                    Self::List(idents, tys)
                }
            }
            Enclosure::Brace { fields, .. } => {
                if fields.is_empty() {
                    Self::None
                } else {
                    Self::Named(
                        fields.iter()
                            .cloned()
                            .map(|mut field| {
                                field.ty = set_lifetime(field.ty);
                                field
                            })
                            .collect()
                    )
                }
            }
        }
    }
}

impl From<attrs::ImportRaw> for Imports {
    fn from(value: attrs::ImportRaw) -> Self {
        Imports::Raw(value.value.ident, Box::new(set_lifetime(value.value.ty)))
    }
}

impl<T: Into<Imports> + KeywordToken> TrySet<Imports> for T {
    fn try_set(self, to: &mut Imports) -> syn::Result<()> {
        if matches!(*to, Imports::None) {
            *to = self.into();
            Ok(())
        } else {
            Err(syn::Error::new(
                self.keyword_span(),
                "conflicting import keyword",
            ))
        }
    }
}
