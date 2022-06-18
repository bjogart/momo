use std::collections::HashSet;
use std::env;
use std::fs;
use std::str::FromStr;

use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use ungrammar::Grammar;
use ungrammar::Rule;

enum Generable {
    Node(String, Vec<Field>),
    Delegate(String, Vec<String>),
}

#[derive(Debug)]
struct Field {
    name: String,
    cardi: Cardinality,
    kind: FieldKind,
}

#[derive(Clone, Debug)]
enum FieldKind {
    Node,
    Token,
}

#[derive(Debug)]
enum Cardinality {
    One,
    Many,
}

fn main() {
    let grm_text = fs::read_to_string("momo.ungram").unwrap();
    let grm = Grammar::from_str(&grm_text).unwrap();

    let mut file = quote! {
        use rowan::ast::AstNode;
        use rowan::Language;
        use rowan::NodeOrToken;
        use syntax::SyntaxKind;
        use syntax::MomoLang;

        use crate::SyntaxNode;
        use crate::SyntaxToken;
    };

    file.extend(
        grm.iter()
            .map(|node| impl_node(collect_rule(&grm, &grm[node].rule, grm[node].name.to_owned()))),
    );

    let out_dir = env::var("OUT_DIR").unwrap();
    fs::write(out_dir + "\\ast_impl.rs", file.to_string()).unwrap();
}

fn impl_node(node: Generable) -> TokenStream {
    match node {
        Generable::Node(node_name, fields) => {
            let node_name = format_ident!("{}", node_name);
            let mut imp = quote! {
                #[derive(Debug, Clone)]
                pub struct #node_name(SyntaxNode);
                impl AstNode for #node_name {
                    type Language = MomoLang;
                    fn can_cast(kind: <Self::Language as Language>::Kind) -> bool { matches!(kind, SyntaxKind::#node_name) }
                    fn cast(syntax: SyntaxNode) -> Option<Self> { if Self::can_cast(syntax.kind()) { Some(Self(syntax)) } else { None } }
                    fn syntax(&self) -> &SyntaxNode { &self.0 }
                }
            };

            let field_imps: Vec<TokenStream> = fields
                .into_iter()
                .map(|Field { name, cardi, kind }| {
                    let func_stem = snake_cased(&name);
                    let ty_ident = format_ident!("{}", name);

                    let (func_ident, imp) = match cardi {
                        Cardinality::One => (
                            format_ident!("{}", &func_stem),
                            match kind {
                                FieldKind::Node => quote! { Option<#ty_ident> { self.0.children().find_map(#ty_ident::cast) } } ,
                                FieldKind::Token => quote! { Option<SyntaxToken> { self.0.children_with_tokens().find_map(|c| match c { NodeOrToken::Token(c) if matches!(c.kind(), SyntaxKind::#ty_ident) => Some(c), _ => None, }) } },
                            },
                        ),
                        Cardinality::Many => (
                            format_ident!("{}", plural(&func_stem)),
                            match kind {
                                FieldKind::Node => quote! { impl Iterator<Item = #ty_ident> { self.0.children().filter_map(#ty_ident::cast) } } ,
                                FieldKind::Token => quote! { impl Iterator<Item = SyntaxToken> { self.0.children_with_tokens().filter_map(|c| match c { NodeOrToken::Token(c) if matches!(c.kind(), SyntaxKind::#ty_ident) => Some(c), _ => None, }) } },
                            },
                        ),
                    };

                    quote! { pub fn #func_ident(&self) -> #imp }
                })
                .collect();

            imp.extend(quote! { impl #node_name { #(#field_imps)* }});

            imp
        }
        Generable::Delegate(node_name, variants) => {
            let node_name = format_ident!("{}", node_name);
            let (variants, try_intos): (Vec<Ident>, Vec<Ident>) = variants
                .into_iter()
                .map(|name| {
                    (format_ident!("{}", name), format_ident!("try_into_{}", snake_cased(&name)))
                })
                .unzip();

            quote! {
                #[derive(Debug, Clone)]
                pub enum #node_name { #(#variants(#variants)),* }
                impl AstNode for #node_name {
                    type Language = MomoLang;
                    fn can_cast(kind: <Self::Language as Language>::Kind) -> bool { matches!(kind, #(SyntaxKind::#variants)|*) }
                    fn cast(syntax: SyntaxNode) -> Option<Self> { match syntax.kind() { #(SyntaxKind::#variants => Some(Self::#variants(#variants::cast(syntax).unwrap())),)* _ => None } }
                    fn syntax(&self) -> &SyntaxNode { match self { #(Self::#variants(node) => node.syntax()),* } }
                }
                impl #node_name {
                    #(pub fn #try_intos(self) -> Result<#variants, Self> { match self { Self::#variants(n) => Ok(n), _ => Err(self), } })*
                }
            }
        }
    }
}

fn collect_rule(grm: &Grammar, rule: &Rule, name: String) -> Generable {
    return match rule {
        Rule::Node(node) => Generable::Node(
            name,
            vec![Field {
                name: grm[*node].name.clone(),
                cardi: Cardinality::One,
                kind: FieldKind::Node,
            }],
        ),
        Rule::Token(tok) => Generable::Node(
            name,
            vec![Field {
                name: token_ident(&grm[*tok].name),
                cardi: Cardinality::One,
                kind: FieldKind::Token,
            }],
        ),
        Rule::Alt(alts) => Generable::Delegate(
            name,
            alts.iter()
                .map(|alt| match alt {
                    Rule::Node(node) => grm[*node].name.clone(),
                    Rule::Labeled { .. }
                    | Rule::Token(_)
                    | Rule::Seq(_)
                    | Rule::Alt(_)
                    | Rule::Opt(_)
                    | Rule::Rep(_) => panic!("Alternatives (with '|') must be nodes: {alt:?}"),
                })
                .collect(),
        ),
        Rule::Opt(inner) => collect_rule(grm, inner, name),
        Rule::Seq(inner) => {
            let fields: Vec<Field> = inner
                .iter()
                .map(|rule| {
                    let (name, kind) =
                        name_of_rule_atom(grm, rule).expect("Ungrammar alts must be top-level");
                    Field { name, kind, cardi: Cardinality::One }
                })
                .collect();

            // fields must be unique
            assert_eq!(
                fields
                    .iter()
                    .map(|Field { name, .. }| name.as_ref())
                    .collect::<HashSet<&str>>()
                    .len(),
                fields.len()
            );

            Generable::Node(name, fields)
        }
        Rule::Rep(inner) => Generable::Node(
            name,
            vec![{
                let (name, kind) = name_of_rule_atom(grm, inner).unwrap();
                Field { name, kind, cardi: Cardinality::Many }
            }],
        ),
        Rule::Labeled { .. } => {
            unreachable!("this syntax generator does not support labels: {rule:?}")
        }
    };
}

fn name_of_rule_atom(grm: &Grammar, rule: &Rule) -> Option<(String, FieldKind)> {
    match rule {
        Rule::Node(node) => Some((grm[*node].name.clone(), FieldKind::Node)),
        Rule::Token(tok) => Some((token_ident(&grm[*tok].name), FieldKind::Token)),
        Rule::Opt(inner) => name_of_rule_atom(grm, inner), // optionals of atoms are fine
        Rule::Labeled { .. } | Rule::Seq(_) | Rule::Alt(_) | Rule::Rep(_) => None,
    }
}

fn token_ident(tok: &str) -> String {
    match tok {
        "use" => "UZE",
        "path" => "PATH",
        "ident" => "IDENT",
        "mod" => "MODULE",
        "as" => "AZ",
        "data" => "DAT",
        "->" => "ARROW",
        "(" => "LPAR",
        ")" => "RPAR",
        ":" => "COLON",
        ";" => "SEMI",
        "{" => "LCURL",
        "}" => "RCURL",
        "[" => "LBRAC",
        "]" => "RBRAC",
        "," => "COMMA",
        "=" => "EQ",
        "todo" => "DUMMY",
        _ => panic!("unexpected token identifier: {tok:?}:"),
    }
    .to_owned()
}

fn snake_cased(s: &str) -> String {
    let mut buf = String::with_capacity(s.len());
    let mut prev = s.chars().next().unwrap();

    for ch in s.chars() {
        let are_letters = ch.is_ascii_alphabetic() && prev.is_ascii_alphabetic();
        let case_changed = ch.is_ascii_uppercase() && prev.is_ascii_lowercase();
        if are_letters && case_changed {
            buf.push('_');
        }

        prev = ch;
        buf.push(ch.to_ascii_lowercase());
    }

    buf
}

fn plural(s: &str) -> String {
    let mut s = s.to_owned();
    s.push('s');
    s
}
