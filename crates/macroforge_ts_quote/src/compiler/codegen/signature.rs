use super::error::GenResult;
use super::*;

impl Codegen {
    pub(super) fn generate_prop_signature(
        &self,
        readonly: bool,
        name: &IrNode,
        optional: bool,
        type_ann: Option<&IrNode>,
    ) -> GenResult<TokenStream> {
        let name_code = self.generate_expr(name)?;
        // Type annotation is legitimately optional for TypeScript property signatures
        let type_ann_code = type_ann
            .map(|t| -> GenResult<TokenStream> {
                let tc = self.generate_type(t)?;
                Ok(quote! {
                    Some(Box::new(macroforge_ts::swc_core::ecma::ast::TsTypeAnn {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        type_ann: Box::new(#tc),
                    }))
                })
            })
            .transpose()?
            .unwrap_or(quote! { None });

        Ok(quote! {
            macroforge_ts::swc_core::ecma::ast::TsTypeElement::TsPropertySignature(
                macroforge_ts::swc_core::ecma::ast::TsPropertySignature {
                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                    readonly: #readonly,
                    key: Box::new(#name_code),
                    computed: false,
                    optional: #optional,
                    type_ann: #type_ann_code,
                }
            )
        })
    }

    pub(super) fn generate_method_signature(
        &self,
        name: &IrNode,
        optional: bool,
        type_params: Option<&IrNode>,
        params: &[IrNode],
        return_type: Option<&IrNode>,
    ) -> GenResult<TokenStream> {
        let name_code = self.generate_expr(name)?;
        // Type parameters are legitimately optional for TypeScript method signatures
        let type_params_code = type_params
            .map(|tp| -> GenResult<TokenStream> {
                let tpc = self.generate_type_params(tp)?;
                Ok(quote! { Some(Box::new(#tpc)) })
            })
            .transpose()?
            .unwrap_or(quote! { None });
        // TsMethodSignature expects Vec<TsFnParam>, not Vec<Param>
        let params_code = self.generate_ts_fn_params(params)?;
        // Return type is legitimately optional for TypeScript method signatures
        let return_type_code = return_type
            .map(|rt| -> GenResult<TokenStream> {
                let rtc = self.generate_type(rt)?;
                Ok(quote! {
                    Some(Box::new(macroforge_ts::swc_core::ecma::ast::TsTypeAnn {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        type_ann: Box::new(#rtc),
                    }))
                })
            })
            .transpose()?
            .unwrap_or(quote! { None });

        Ok(quote! {
            macroforge_ts::swc_core::ecma::ast::TsTypeElement::TsMethodSignature(
                macroforge_ts::swc_core::ecma::ast::TsMethodSignature {
                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                    key: Box::new(#name_code),
                    computed: false,
                    optional: #optional,
                    type_params: #type_params_code,
                    params: #params_code,
                    type_ann: #return_type_code,
                }
            )
        })
    }
}
