use syn::Field;

pub fn parse_option(field: &Field) -> (bool, &syn::Type) {
    let mut is_option: bool = false;
    let mut ty: &syn::Type = &field.ty;

    match &field.ty {
        syn::Type::Path(type_path) => {
            is_option = type_path
                .path
                .segments
                .last()
                .map(|s| s.ident == "Option")
                .unwrap_or(false);
            if is_option {
                ty = extract_type_from_option(&ty).expect("Inner type should not be None...");
            };
        }
        _ => (),
    };

    (is_option, ty)
}

fn extract_type_from_option(ty: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(syn::TypePath { qself: None, path }) = ty {
        let segments_str = &path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>()
            .join(":");

        let option_segment = ["Option", "std:option:Option", "core:option:Option"]
            .iter()
            .find(|s| segments_str == *s)
            .and_then(|_| path.segments.last());

        let inner_type = option_segment
            .and_then(|path_seg| match &path_seg.arguments {
                syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                    args,
                    ..
                }) => args.first(),
                _ => None,
            })
            .and_then(|generic_arg| match generic_arg {
                syn::GenericArgument::Type(ty) => Some(ty),
                _ => None,
            });

        return inner_type;
    }
    None
}
