//! Emit token from a TypeScript type definition

use intwc_semantic_tokens::token;
use swc_core::ecma::ast::{TsKeywordType, TsThisType, TsType, TsTypeParamDecl, TsTypeParamInstantiation};

use crate::SemanticTokenizer;

impl<'a> SemanticTokenizer<'a> {
    /// Emit tokens from a TypeScript type definition
    pub fn emit_type(&mut self, typ: &TsType) {
        println!("{:#?}", typ);
        match typ {
            TsType::TsKeywordType(TsKeywordType {span,.. }) 
            |
            TsType::TsThisType(TsThisType {span,.. })
            => {
                self.add_span(token!(type), *span);
            },
            TsType::TsFnOrConstructorType(ts_fn_or_constructor_type) => todo!(),
            TsType::TsTypeRef(ts_type_ref) => todo!(),
            TsType::TsTypeQuery(ts_type_query) => todo!(),
            TsType::TsTypeLit(ts_type_lit) => todo!(),
            TsType::TsArrayType(ts_array_type) => todo!(),
            TsType::TsTupleType(ts_tuple_type) => todo!(),
            TsType::TsOptionalType(ts_optional_type) => todo!(),
            TsType::TsRestType(ts_rest_type) => todo!(),
            TsType::TsUnionOrIntersectionType(ts_union_or_intersection_type) => todo!(),
            TsType::TsConditionalType(ts_conditional_type) => todo!(),
            TsType::TsInferType(ts_infer_type) => todo!(),
            TsType::TsParenthesizedType(ts_parenthesized_type) => todo!(),
            TsType::TsTypeOperator(ts_type_operator) => todo!(),
            TsType::TsIndexedAccessType(ts_indexed_access_type) => todo!(),
            TsType::TsMappedType(ts_mapped_type) => todo!(),
            TsType::TsLitType(ts_lit_type) => todo!(),
            TsType::TsTypePredicate(ts_type_predicate) => todo!(),
            TsType::TsImportType(ts_import_type) => todo!(),
        }
    }

    /// Emit type parameters
    pub fn emit_type_param(&mut self, typ: &TsTypeParamDecl) {
        for param in &typ.params {
            // name is a type
            self.add_span(token!(type), param.name.span);

            // .. extends Foo
            if let Some(t) = &param.constraint {
                self.emit_type(t);
            }

            // .. = Foo
            if let Some(t) = &param.default {
                self.emit_type(t);
            }
        }
    }

    pub fn emit_type_param_inst(&mut self, typ: &TsTypeParamInstantiation) {
        for param in &typ.params {
            self.emit_type(param);
        }
    }
}
