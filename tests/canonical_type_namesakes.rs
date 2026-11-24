// SPDX-License-Identifier: AGPL-3.0-or-later
//! Source-level regression coverage for FR-273's semantic-value location namesakes.

use syn::{
    AngleBracketedGenericArguments, Fields, FnArg, GenericArgument, ImplItem, Item, PathArguments,
    ReturnType, Type, UseTree, Visibility,
};

use ix_trace_rs::trace;

const LOCATION_SOURCE: &str = include_str!("../src/location.rs");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ItemKind {
    Declaration,
    TypeAlias,
    ReExport,
    Import,
    Item,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ScannedName {
    identifier: String,
    kind: ItemKind,
}

fn scan_module_items(source: &str) -> syn::Result<Vec<ScannedName>> {
    let file = syn::parse_file(source)?;
    let mut names = Vec::new();

    for item in file.items {
        match item {
            Item::Const(item) => push_name(&mut names, item.ident.to_string(), ItemKind::Item),
            Item::Enum(item) => {
                push_name(&mut names, item.ident.to_string(), ItemKind::Declaration)
            }
            Item::ExternCrate(item) => push_name(
                &mut names,
                item.rename
                    .map_or_else(|| item.ident.to_string(), |(_, rename)| rename.to_string()),
                ItemKind::Item,
            ),
            Item::Fn(item) => push_name(&mut names, item.sig.ident.to_string(), ItemKind::Item),
            Item::ForeignMod(item) => {
                for foreign in item.items {
                    match foreign {
                        syn::ForeignItem::Fn(item) => {
                            push_name(&mut names, item.sig.ident.to_string(), ItemKind::Item)
                        }
                        syn::ForeignItem::Static(item) => {
                            push_name(&mut names, item.ident.to_string(), ItemKind::Item)
                        }
                        syn::ForeignItem::Type(item) => {
                            push_name(&mut names, item.ident.to_string(), ItemKind::Declaration)
                        }
                        syn::ForeignItem::Macro(item) => push_name(
                            &mut names,
                            item.mac
                                .path
                                .segments
                                .last()
                                .map_or_else(String::new, |segment| segment.ident.to_string()),
                            ItemKind::Item,
                        ),
                        syn::ForeignItem::Verbatim(_) => {}
                        _ => {}
                    }
                }
            }
            Item::Macro(item) => {
                if let Some(identifier) = item.ident {
                    push_name(&mut names, identifier.to_string(), ItemKind::Item);
                }
            }
            Item::Mod(item) => push_name(&mut names, item.ident.to_string(), ItemKind::Item),
            Item::Static(item) => push_name(&mut names, item.ident.to_string(), ItemKind::Item),
            Item::Struct(item) => {
                push_name(&mut names, item.ident.to_string(), ItemKind::Declaration)
            }
            Item::Trait(item) => push_name(&mut names, item.ident.to_string(), ItemKind::Item),
            Item::TraitAlias(item) => push_name(&mut names, item.ident.to_string(), ItemKind::Item),
            Item::Type(item) => push_name(&mut names, item.ident.to_string(), ItemKind::TypeAlias),
            Item::Union(item) => {
                push_name(&mut names, item.ident.to_string(), ItemKind::Declaration)
            }
            Item::Use(item) => {
                let kind = if matches!(item.vis, Visibility::Inherited) {
                    ItemKind::Import
                } else {
                    ItemKind::ReExport
                };
                scan_use_tree(&item.tree, kind, &mut names);
            }
            Item::Impl(_) | Item::Verbatim(_) => {}
            _ => {}
        }
    }

    Ok(names)
}

fn scan_use_tree(tree: &UseTree, kind: ItemKind, names: &mut Vec<ScannedName>) {
    match tree {
        UseTree::Path(path) => scan_use_tree(&path.tree, kind, names),
        UseTree::Name(name) => push_name(names, name.ident.to_string(), kind),
        UseTree::Rename(rename) => push_name(names, rename.rename.to_string(), kind),
        UseTree::Group(group) => {
            for item in &group.items {
                scan_use_tree(item, kind, names);
            }
        }
        UseTree::Glob(_) => {}
    }
}

fn push_name(names: &mut Vec<ScannedName>, identifier: String, kind: ItemKind) {
    names.push(ScannedName { identifier, kind });
}

fn named_item<'a>(items: &'a [Item], wanted: &str) -> Option<&'a Item> {
    items.iter().find(|item| match item {
        Item::Struct(item) => item.ident == wanted,
        Item::Enum(item) => item.ident == wanted,
        Item::Union(item) => item.ident == wanted,
        Item::Type(item) => item.ident == wanted,
        _ => false,
    })
}

fn type_path_ends_with(ty: &Type, wanted: &str) -> bool {
    matches!(ty, Type::Path(path) if path.path.segments.last().is_some_and(|segment| segment.ident == wanted))
}

fn named_field_type<'a>(fields: &'a Fields, wanted: &str) -> Option<&'a Type> {
    let Fields::Named(fields) = fields else {
        return None;
    };
    fields
        .named
        .iter()
        .find(|field| {
            field
                .ident
                .as_ref()
                .is_some_and(|identifier| identifier == wanted)
        })
        .map(|field| &field.ty)
}

fn expression_location_impl(items: &[Item]) -> Option<&syn::ItemImpl> {
    items.iter().find_map(|item| {
        let Item::Impl(item) = item else {
            return None;
        };
        type_path_ends_with(&item.self_ty, "ExpressionLocation").then_some(item)
    })
}

fn method<'a>(item: &'a syn::ItemImpl, wanted: &str) -> Option<&'a syn::Signature> {
    item.items.iter().find_map(|item| match item {
        ImplItem::Fn(item) if item.sig.ident == wanted => Some(&item.sig),
        _ => None,
    })
}

fn returns_vec_usize(signature: &syn::Signature) -> bool {
    let ReturnType::Type(_, ty) = &signature.output else {
        return false;
    };
    let Type::Path(vector) = &**ty else {
        return false;
    };
    let Some(segment) = vector.path.segments.last() else {
        return false;
    };
    if segment.ident != "Vec" {
        return false;
    }
    let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =
        &segment.arguments
    else {
        return false;
    };
    matches!(args.first(), Some(GenericArgument::Type(Type::Path(path))) if path.path.is_ident("usize"))
}

fn accepts_child_index_path(signature: &syn::Signature) -> bool {
    signature.inputs.iter().any(|argument| {
        let FnArg::Typed(argument) = argument else {
            return false;
        };
        let syn::Pat::Ident(pattern) = &*argument.pat else {
            return false;
        };
        if pattern.ident != "path" {
            return false;
        }
        let Type::Reference(reference) = &*argument.ty else {
            return false;
        };
        let Type::Slice(slice) = &*reference.elem else {
            return false;
        };
        type_path_ends_with(&slice.elem, "usize")
    })
}

fn owner_field_is_expression_owner(items: &[Item]) -> bool {
    let Some(Item::Struct(expression_location)) = named_item(items, "ExpressionLocation") else {
        return false;
    };
    named_field_type(&expression_location.fields, "origin")
        .is_some_and(|ty| type_path_ends_with(ty, "ExpressionOwner"))
}

#[test]
#[trace("TC-748", "FR-273-AC-49")]
fn location_module_declares_expression_location_and_mutation_scan_finds_location_alias() {
    let names = scan_module_items(LOCATION_SOURCE).expect("the checked location module parses");
    let file = syn::parse_file(LOCATION_SOURCE).expect("the checked location module parses");

    assert!(names.iter().any(|item| {
        item.identifier == "ExpressionLocation" && item.kind == ItemKind::Declaration
    }));
    assert!(owner_field_is_expression_owner(&file.items));
    let location_impl = expression_location_impl(&file.items).expect("ExpressionLocation impl");
    assert!(method(location_impl, "child").is_some());
    let at = method(location_impl, "at").expect("path-based ExpressionLocation constructor");
    assert!(accepts_child_index_path(at));
    let path = method(location_impl, "path").expect("child-index path accessor");
    assert!(returns_vec_usize(path));
    assert!(!names.iter().any(|item| item.identifier == "Location"));

    let mutated = format!("{LOCATION_SOURCE}\npub type Location = ExpressionLocation;\n");
    let mutated_names = scan_module_items(&mutated).expect("the alias mutation parses");
    assert!(mutated_names
        .iter()
        .any(|item| { item.identifier == "Location" && item.kind == ItemKind::TypeAlias }));
}

#[test]
#[trace("TC-748", "FR-273-AC-50")]
fn location_module_declares_expression_owner_and_mutation_scan_finds_origin_reexport() {
    let names = scan_module_items(LOCATION_SOURCE).expect("the checked location module parses");
    let file = syn::parse_file(LOCATION_SOURCE).expect("the checked location module parses");

    assert!(names
        .iter()
        .any(|item| item.identifier == "ExpressionOwner" && item.kind == ItemKind::Declaration));
    assert!(owner_field_is_expression_owner(&file.items));
    assert!(!names.iter().any(|item| item.identifier == "Origin"));

    let mutated = format!("{LOCATION_SOURCE}\npub use self::ExpressionOwner as Origin;\n");
    let mutated_names = scan_module_items(&mutated).expect("the re-export mutation parses");
    assert!(mutated_names
        .iter()
        .any(|item| item.identifier == "Origin" && item.kind == ItemKind::ReExport));
}
