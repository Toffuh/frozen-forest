use proc_macro::Span;
use syn::__private::quote::quote;
use syn::__private::TokenStream;
use syn::{parse_macro_input, parse_quote, Fields, ItemStruct, LitInt, LitStr};

#[proc_macro_attribute]
pub fn sprite_sheet(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut path: Option<LitStr> = None;
    let mut count: Option<LitInt> = None;

    let args_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("path") {
            path = meta.value()?.parse()?;
            Ok(())
        } else if meta.path.is_ident("count") {
            count = meta.value()?.parse()?;
            Ok(())
        } else {
            Err(meta.error("unsupported spriteSheet property"))
        }
    });

    let mut input = parse_macro_input!(input as ItemStruct);
    parse_macro_input!(args with args_parser);

    let name = input.ident.clone();

    let Fields::Named(named) = &mut input.fields else {
        return syn::Error::into_compile_error(syn::Error::new(
            Span::call_site().into(),
            "wrong struct type use {}",
        ))
        .into();
    };

    let Some(path) = path else {
        return syn::Error::into_compile_error(syn::Error::new(
            Span::call_site().into(),
            "no path was set",
        ))
        .into();
    };

    let Some(count) = count else {
        return syn::Error::into_compile_error(syn::Error::new(
            Span::call_site().into(),
            "no count was set",
        ))
        .into();
    };

    input.attrs.push(parse_quote!(#[derive(Resource)]));

    named
        .named
        .push(parse_quote!(layout: Handle<TextureAtlasLayout>));
    named.named.push(parse_quote!(texture: Handle<Image>));

    let load = quote! {
        impl #name {
            fn load(
                asset_server: &Res<AssetServer>,
                texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
            ) -> Self {
                let texture_handle = asset_server.load(#path);
                let layout_handle = texture_atlases.add(TextureAtlasLayout::from_grid(
                    vec2(16., 16.),
                    #count,
                    1,
                    None,
                    None,
                ));

                Self {
                    layout: layout_handle,
                    texture: texture_handle,
                }
            }

            fn atlas(&self) -> TextureAtlas {
                let mut rng = thread_rng();

                TextureAtlas {
                    layout: self.layout.clone(),
                    index: rng.gen_range(0..#count),
                }
            }

            fn texture(&self) -> Handle<Image> {
                self.texture.clone()
            }
        }
    };

    proc_macro::TokenStream::from(quote! {
        #input
        #load
    })
}
