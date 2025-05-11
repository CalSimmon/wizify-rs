pub fn extract_type_from_option(ty: &syn::Type) -> Option<&syn::Type> {
    // If it is not TypePath, it cannot be Option<T>
    if let syn::Type::Path(syn::TypePath { qself: None, path }) = ty {
        // Join path segments to create a string for comparison
        let segments_str = &path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>()
            .join(":");

        // Find the segment containing Option
        let option_segment = ["Option", "std:option:Option", "core:option:Option"]
            .iter()
            .find(|s| segments_str == *s)
            .and_then(|_| path.segments.last());

        // Extract the inner type from the generic arguments
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
