



use convert_case::Casing;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote};
use syn::{parse_macro_input, Variant, punctuated::Punctuated, token::{Comma}, Attribute, parse::{Parse, ParseStream}};
use syn::Token;
use syn::ItemEnum;


#[proc_macro_attribute]
pub fn object_builder(args: TokenStream, input:TokenStream) -> TokenStream {
    let ItemEnum { ident, variants, .. } = parse_macro_input!(input as ItemEnum);

   let out= if args.to_string()=="display" {
        create_object_display(&ident, &variants)
    }else {
        create_object(&ident, &variants)
    };
    
        out
}

fn create_object(_ident:&Ident,_variants:&Punctuated<Variant,Comma>)->TokenStream{
quote!().into()
}

fn create_object_display(ident:&Ident,variants:&Punctuated<Variant,Comma>)->TokenStream{
    let set = variants.iter().map(|it| {
        let type_s = &it.ident;
        let a = format!("Nextion{}", &type_s);
        let name_struct = Ident::new(a.as_str(), it.ident.span());

        let atr =it.attrs.iter().map(|attribute|{
           if attribute.path().is_ident("nextion") {
                get_atr_impl(&name_struct, attribute)
        }else{
                quote!()
            }
        });
      
        quote!(
            pub struct #type_s;
            pub struct #name_struct <'l,USART> (NextionObjectDisplay<'l,#type_s,USART>);
            impl<'l,USART> #name_struct <'l,USART>
            where
                USART: embedded_hal::serial::Read<u8> + embedded_hal::blocking::serial::Write<u8>,
            {
                pub fn bind(device: &mut Nextion<USART>, pid: u8, cid: u8, name: &'l str)->
                    Self
                {
                   Self(NextionObjectDisplay::bind(device,pid,cid,name))
                }
            }

            impl<'l> #type_s
            {
                pub fn bind<USART>(device: &mut Nextion<USART>, pid: u8, cid: u8, name: &'l str)->
                    #name_struct<'l,USART>
                where
                    USART: embedded_hal::serial::Read<u8> + embedded_hal::blocking::serial::Write<u8>,
                {
                    #name_struct(NextionObjectDisplay::bind(device,pid,cid,name))
                }
            }
            impl ObjectTypes for #type_s{}
            
            // impl basic
            impl<'l, USART> NextionCom<USART> for #name_struct<'l, USART>
            where
                USART: embedded_hal::serial::Read<u8> + embedded_hal::blocking::serial::Write<u8>,{}

            impl<'l,USART> ObjInfo<USART> for #name_struct<'l, USART>
            where
                USART: embedded_hal::serial::Read<u8> + embedded_hal::blocking::serial::Write<u8>,
            {
                fn get_device(&mut self) -> &mut Nextion<USART>{
                    self.0.get_device()
                }
            }

            impl<'l, USART> BaseInfo for #name_struct<'l, USART>
            where
                USART: embedded_hal::serial::Read<u8> + embedded_hal::blocking::serial::Write<u8>,
            {
                fn get_page_id(&self) -> u8 {
                            self.0.get_page_id()
                }
                fn get_component_id(&self) -> u8 {
                    self.0.get_component_id()
                }
                fn get_component_name(&self) -> &str {
                    self.0.get_component_name()
                }
            }
            impl<'l, USART> TouchHandler<'l> for #name_struct<'l, USART>
            where
                USART: embedded_hal::serial::Read<u8> + embedded_hal::blocking::serial::Write<u8>,
            {
                fn set_on_click(&mut self, handler: &'l mut dyn FnMut()) {
                        self.0.set_on_click(handler);
                }
                
                fn set_on_release(&mut self, handler: &'l mut dyn FnMut()) {
                    self.0.set_on_release(handler);
                }

                fn call_on_click(&mut self) {
                    self.0.call_on_click();
                }
                fn call_on_release(&mut self) {
                    self.0.call_on_release();
                }
            }
            // impl
            #(#atr)*

        )
    });

    let case = ident.to_string().to_case(convert_case::Case::Snake);
    let mods = Ident::new(&case, ident.span());

    let identifier=if let "NextionObjectDisplay" = ident.to_string().as_str() {
        Ident::new("crate", ident.span())
    }else{
        Ident::new("gx_rust_nextion", ident.span())
    };

    quote! {
        pub mod #mods{
            use #identifier::components::ObjectTypes;
            use #identifier::components::objects::NextionObjectDisplay;
            use #identifier::nextion::Nextion;
            use #identifier::components::NextionVal;
            use #identifier::components::NextionAct;
            use #identifier::components::ObjInfo;
            use #identifier::components::BaseInfo;
            use #identifier::nextion::NextionCom;
            use #identifier::components::objects::TouchHandler;

            use #identifier::components::component_trait::*;
        #(
            #set
        )*
    }
}.into()
}

fn get_atr_impl(ident:&Ident,attr:&Attribute)->proc_macro2::TokenStream{


            let tokens:NextionParser = match attr.parse_args(){
                Ok(x) => x,
                Err(_) =>return quote!(),
            };
           let out =tokens.result.iter().map(|ptv|{
            let name=&ptv.name_trait;
            let a=format!("Nextion_{}",name).to_case(convert_case::Case::UpperCamel);
            let name_trait=Ident::new(&a,name.span());
            let a=
                    if name=="val"{
                        let b=match &ptv.trait_value {
                            Some(x) => {
                                if x.to_string()=="bool" {
                                    quote!(
                                        impl<'l,USART> #name_trait<USART> for #ident<'l,USART>
                                        where
                                        USART:embedded_hal::serial::Read<u8>+embedded_hal::blocking::serial::Write<u8>,
                                        {
                                            type ValueType=u8;
                                        }

                                        impl<'l,USART> NextionAct<USART> for #ident<'l,USART>
                                        where
                                        USART:embedded_hal::serial::Read<u8>+embedded_hal::blocking::serial::Write<u8>,
                                        {
                                        }
                                    )
                                }else{
                                quote!(
                                    impl<'l,USART> #name_trait<USART> for #ident<'l,USART>
                                    where
                                    USART:embedded_hal::serial::Read<u8>+embedded_hal::blocking::serial::Write<u8>,
                                    {
                                        type ValueType=#x;
                                    }
                                )
                            }},
                            None =>{
                                quote!(
                                    impl<'l,USART> #name_trait<USART> for #ident<'l,USART>
                                    where
                                    USART:embedded_hal::serial::Read<u8>+embedded_hal::blocking::serial::Write<u8>,
                                    {
                                        type ValueType=i32;
                                    }
                                )
                            },
                        };
                        b
                    }else {
                       let b= match &ptv.trait_value {
                            Some(x) => {
                                let a=format!("Nextion_{}_{}",x,name).to_case(convert_case::Case::UpperCamel);
                                let name_trait=Ident::new(&a,name.span());
                                quote!(
                                    impl<'l,USART> #name_trait<USART> for #ident<'l,USART>
                                    where
                                        USART:embedded_hal::serial::Read<u8>+embedded_hal::blocking::serial::Write<u8>,{}
                                )
                            },
                            None => {
                                quote!(
                                impl<'l,USART> #name_trait<USART> for #ident<'l,USART>
                                    where
                                        USART:embedded_hal::serial::Read<u8>+embedded_hal::blocking::serial::Write<u8>,{}
                            )
                        },
                        };
                        b
                    };
            a

           });
           
quote!(#(#out)*)
}

struct ParseTokenValue {
    name_trait: Ident,
    trait_value: Option<Ident>,

}

struct NextionParser{
    result:Vec<ParseTokenValue>
}

impl Parse for NextionParser {
    fn parse(input: ParseStream) ->syn::Result<Self> {

        // split input with `'`;
        let binding = input.to_string();
        let strs:Vec<&str> =binding.split(',').collect();

        let vec =strs.iter().map(|str|{
            // get name trait
            let name_trait:Ident =input.parse().unwrap();
            // if str containt `=`, parse value:
            let trait_value =if str.contains('=') {
                let _:Token![=]=input.parse().unwrap();
                let value:Ident=input.parse().unwrap();
                Some(value)
            }else{
                None
            };
            //if not last parse `,`
            if strs.last().unwrap()!=str {
                let _:Token![,]=input.parse().unwrap();
            }
            ParseTokenValue{name_trait,trait_value}
        }).collect();

        Ok(NextionParser{result:vec})
    
    }
}