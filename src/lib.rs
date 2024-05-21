use std::collections::HashMap;

use syn::{
    punctuated::{IterMut, Punctuated},
    token::{Comma, PathSep, Plus},
    AngleBracketedGenericArguments, Arm, AssocConst, AssocType, BareFnArg, Block, ConstParam,
    Constraint, Expr, ExprArray, ExprAssign, ExprAsync, ExprAwait, ExprBinary, ExprBlock,
    ExprBreak, ExprCall, ExprCast, ExprClosure, ExprConst, ExprField, ExprForLoop, ExprGroup,
    ExprIf, ExprIndex, ExprLet, ExprLoop, ExprMatch, ExprMethodCall, ExprParen, ExprPath,
    ExprRange, ExprReference, ExprRepeat, ExprReturn, ExprStruct, ExprTry, ExprTryBlock, ExprTuple,
    ExprUnary, ExprUnsafe, ExprWhile, ExprYield, Field, FieldPat, FieldValue, Fields, FieldsNamed,
    FieldsUnnamed, FnArg, GenericArgument, GenericParam, Generics, ImplItem, ImplItemConst,
    ImplItemFn, ImplItemType, Item, ItemConst, ItemEnum, ItemFn, ItemImpl, ItemMod, ItemStatic,
    ItemStruct, ItemTrait, ItemTraitAlias, ItemType, ItemUnion, Local, LocalInit,
    ParenthesizedGenericArguments, Pat, PatIdent, PatOr, PatParen, PatReference, PatSlice,
    PatStruct, PatTuple, PatTupleStruct, PatType, Path, PathArguments, PathSegment, PredicateType,
    QSelf, Receiver, ReturnType, Signature, Stmt, TraitBound, TraitItem, TraitItemConst,
    TraitItemFn, TraitItemType, Type, TypeArray, TypeBareFn, TypeGroup, TypeImplTrait, TypeParam,
    TypeParamBound, TypeParen, TypePath, TypePtr, TypeReference, TypeSlice, TypeTraitObject,
    TypeTuple, Variant, WhereClause, WherePredicate,
};

/// Extension trait to replace `TypePath` for 
pub trait ReplaceTypes {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>);
}

impl<'a, T> ReplaceTypes for IterMut<'a, T>
where
    T: ReplaceTypes,
{
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.for_each(|expr| expr.replace_types(substitutions));
    }
}

impl<'a, T> ReplaceTypes for core::slice::IterMut<'a, T>
where
    T: ReplaceTypes,
{
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.for_each(|expr| expr.replace_types(substitutions));
    }
}

impl ReplaceTypes for Type {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        match self {
            Type::Array(array) => array.replace_types(substitutions),
            Type::BareFn(bare_fn) => bare_fn.replace_types(substitutions),
            Type::Group(group) => group.replace_types(substitutions),
            Type::ImplTrait(impl_trait) => impl_trait.replace_types(substitutions),
            Type::Infer(_) => {}
            // TODO: Add support for replacing within macros
            Type::Macro(_) => {}
            Type::Never(_) => {}
            Type::Paren(paren_type) => paren_type.replace_types(substitutions),
            Type::Path(path_type) => path_type.replace_types(substitutions),
            Type::Ptr(ptr_type) => ptr_type.replace_types(substitutions),
            Type::Reference(reference_type) => reference_type.replace_types(substitutions),
            Type::Slice(slice_type) => slice_type.replace_types(substitutions),
            Type::TraitObject(trait_object) => trait_object.replace_types(substitutions),
            Type::Tuple(tuple_type) => tuple_type.replace_types(substitutions),
            Type::Verbatim(_) => {}
            _ => {}
        }
    }
}

impl ReplaceTypes for Punctuated<Type, Comma> {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.iter_mut().replace_types(substitutions);
    }
}

impl ReplaceTypes for TypeArray {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.elem.replace_types(substitutions);
    }
}

impl ReplaceTypes for TypeBareFn {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.inputs.iter_mut().replace_types(substitutions);
        self.output.replace_types(substitutions);
    }
}

impl ReplaceTypes for BareFnArg {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.ty.replace_types(substitutions);
    }
}

impl ReplaceTypes for ReturnType {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        match self {
            ReturnType::Default => {}
            ReturnType::Type(_, return_type) => return_type.replace_types(substitutions),
        }
    }
}

impl ReplaceTypes for TypeGroup {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.elem.replace_types(substitutions);
    }
}

impl ReplaceTypes for TypeImplTrait {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.bounds.replace_types(substitutions);
    }
}

impl ReplaceTypes for Punctuated<TypeParamBound, Plus> {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.iter_mut().replace_types(substitutions);
    }
}

impl ReplaceTypes for TypeParamBound {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        match self {
            TypeParamBound::Trait(trait_bound) => {
                trait_bound.path.replace_types(substitutions);
            }
            TypeParamBound::Lifetime(_) => {}
            TypeParamBound::Verbatim(_) => {}
            _ => {}
        }
    }
}

impl ReplaceTypes for TraitBound {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.path.replace_types(substitutions);
    }
}

impl ReplaceTypes for Path {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.segments.replace_types(substitutions);
    }
}

impl ReplaceTypes for Punctuated<PathSegment, PathSep> {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.iter_mut().replace_types(substitutions);
    }
}

impl ReplaceTypes for PathSegment {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.arguments.replace_types(substitutions);
    }
}

impl ReplaceTypes for PathArguments {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        match self {
            PathArguments::None => {}
            PathArguments::AngleBracketed(arguments) => arguments.replace_types(substitutions),
            PathArguments::Parenthesized(arguments) => arguments.replace_types(substitutions),
        }
    }
}

impl ReplaceTypes for AngleBracketedGenericArguments {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.args.iter_mut().replace_types(substitutions);
    }
}

impl ReplaceTypes for Punctuated<GenericArgument, Comma> {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.iter_mut().replace_types(substitutions);
    }
}

impl ReplaceTypes for ParenthesizedGenericArguments {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.inputs.replace_types(substitutions);
        self.output.replace_types(substitutions);
    }
}

impl ReplaceTypes for GenericArgument {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        match self {
            GenericArgument::Lifetime(_) => {}
            GenericArgument::Type(generic_type) => generic_type.replace_types(substitutions),
            GenericArgument::Const(generic_const) => generic_const.replace_types(substitutions),
            GenericArgument::AssocType(generic_associated_type) => {
                generic_associated_type.replace_types(substitutions)
            }
            GenericArgument::AssocConst(generic_associated_const) => {
                generic_associated_const.replace_types(substitutions)
            }
            GenericArgument::Constraint(_generic_constraint) => {}
            _ => {}
        }
    }
}

impl ReplaceTypes for Expr {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        match self {
            Expr::Array(array_expr) => array_expr.replace_types(substitutions),
            Expr::Assign(assign_expr) => assign_expr.replace_types(substitutions),
            Expr::Async(async_expr) => async_expr.replace_types(substitutions),
            Expr::Await(await_expr) => await_expr.replace_types(substitutions),
            Expr::Binary(binary_expr) => binary_expr.replace_types(substitutions),
            Expr::Block(block_expr) => block_expr.replace_types(substitutions),
            Expr::Break(break_expr) => break_expr.replace_types(substitutions),
            Expr::Call(call_expr) => call_expr.replace_types(substitutions),
            Expr::Cast(cast_expr) => cast_expr.replace_types(substitutions),
            Expr::Closure(closure_expr) => closure_expr.replace_types(substitutions),
            Expr::Const(const_expr) => const_expr.replace_types(substitutions),
            Expr::Continue(_) => {}
            Expr::Field(field_expr) => field_expr.replace_types(substitutions),
            Expr::ForLoop(for_expr) => for_expr.replace_types(substitutions),
            Expr::Group(group_expr) => group_expr.replace_types(substitutions),
            Expr::If(if_expr) => if_expr.replace_types(substitutions),
            Expr::Index(index_expr) => index_expr.replace_types(substitutions),
            Expr::Infer(_) => {}
            Expr::Let(let_expr) => let_expr.replace_types(substitutions),
            Expr::Lit(_) => {}
            Expr::Loop(loop_expr) => loop_expr.replace_types(substitutions),
            // TODO: Add support for replacing within macros
            Expr::Macro(_) => {}
            Expr::Match(match_expr) => match_expr.replace_types(substitutions),
            Expr::MethodCall(method_call_expr) => method_call_expr.replace_types(substitutions),
            Expr::Paren(paren_expr) => paren_expr.replace_types(substitutions),
            Expr::Path(path_expr) => path_expr.replace_types(substitutions),
            Expr::Range(range_expr) => range_expr.replace_types(substitutions),
            Expr::Reference(reference_expr) => reference_expr.replace_types(substitutions),
            Expr::Repeat(repeat_expr) => repeat_expr.replace_types(substitutions),
            Expr::Return(return_expr) => return_expr.replace_types(substitutions),
            Expr::Struct(struct_expr) => struct_expr.replace_types(substitutions),
            Expr::Try(try_expr) => try_expr.replace_types(substitutions),
            Expr::TryBlock(try_block_expr) => try_block_expr.replace_types(substitutions),
            Expr::Tuple(tuple_expr) => tuple_expr.replace_types(substitutions),
            Expr::Unary(unary_expr) => unary_expr.replace_types(substitutions),
            Expr::Unsafe(unsafe_expr) => unsafe_expr.replace_types(substitutions),
            Expr::Verbatim(_) => {}
            Expr::While(while_expr) => while_expr.replace_types(substitutions),
            Expr::Yield(yield_expr) => yield_expr.replace_types(substitutions),
            _ => {}
        }
    }
}

impl ReplaceTypes for ExprArray {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.elems.iter_mut().replace_types(substitutions);
    }
}

impl ReplaceTypes for ExprAssign {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.left.replace_types(substitutions);
        self.right.replace_types(substitutions);
    }
}

impl ReplaceTypes for ExprAsync {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.block.replace_types(substitutions);
    }
}

impl ReplaceTypes for ExprAwait {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.base.replace_types(substitutions);
    }
}

impl ReplaceTypes for ExprBinary {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.left.replace_types(substitutions);
        self.right.replace_types(substitutions);
    }
}

impl ReplaceTypes for ExprBlock {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.block.replace_types(substitutions);
    }
}

impl ReplaceTypes for ExprBreak {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        if let Some(expr) = &mut self.expr {
            expr.replace_types(substitutions);
        }
    }
}

impl ReplaceTypes for ExprCall {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.func.replace_types(substitutions);
        self.args.iter_mut().replace_types(substitutions);
    }
}

impl ReplaceTypes for ExprCast {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.expr.replace_types(substitutions);
        self.ty.replace_types(substitutions);
    }
}

impl ReplaceTypes for ExprClosure {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.inputs.iter_mut().replace_types(substitutions);
        self.body.replace_types(substitutions);
        self.output.replace_types(substitutions);
    }
}

impl ReplaceTypes for ExprConst {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.block.replace_types(substitutions);
    }
}

impl ReplaceTypes for ExprField {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.base.replace_types(substitutions);
    }
}

impl ReplaceTypes for ExprForLoop {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.body.replace_types(substitutions);
        self.expr.replace_types(substitutions);
        self.pat.replace_types(substitutions);
    }
}

impl ReplaceTypes for ExprGroup {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.expr.replace_types(substitutions);
    }
}

impl ReplaceTypes for ExprIf {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.cond.replace_types(substitutions);

        self.then_branch.replace_types(substitutions);

        if let Some((_, expr)) = &mut self.else_branch {
            expr.replace_types(substitutions);
        }
    }
}

impl ReplaceTypes for ExprIndex {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.expr.replace_types(substitutions);
        self.index.replace_types(substitutions);
    }
}

impl ReplaceTypes for ExprLet {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.expr.replace_types(substitutions);
        self.pat.replace_types(substitutions);
    }
}

impl ReplaceTypes for ExprLoop {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.body.replace_types(substitutions);
    }
}

impl ReplaceTypes for ExprMatch {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.expr.replace_types(substitutions);
        self.arms.iter_mut().replace_types(substitutions);
    }
}

impl ReplaceTypes for ExprMethodCall {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.receiver.replace_types(substitutions);

        self.args.iter_mut().replace_types(substitutions);

        if let Some(turbofish) = &mut self.turbofish {
            turbofish.replace_types(substitutions);
        }
    }
}

impl ReplaceTypes for ExprParen {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.expr.replace_types(substitutions);
    }
}

impl ReplaceTypes for ExprPath {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        if let Some(qself) = &mut self.qself {
            qself.replace_types(substitutions);
        }

        self.path.replace_types(substitutions);
    }
}

impl ReplaceTypes for ExprRange {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        if let Some(start) = &mut self.start {
            start.replace_types(substitutions);
        }
        if let Some(stop) = &mut self.end {
            stop.replace_types(substitutions);
        }
    }
}

impl ReplaceTypes for ExprReference {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.expr.replace_types(substitutions);
    }
}

impl ReplaceTypes for ExprRepeat {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.expr.replace_types(substitutions);
        self.len.replace_types(substitutions);
    }
}

impl ReplaceTypes for ExprReturn {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        if let Some(expr) = &mut self.expr {
            expr.replace_types(substitutions);
        }
    }
}

impl ReplaceTypes for ExprStruct {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        if let Some(qself) = &mut self.qself {
            qself.replace_types(substitutions);
        }
        self.path.replace_types(substitutions);
        self.fields.replace_types(substitutions);
    }
}

impl ReplaceTypes for ExprTry {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.expr.replace_types(substitutions);
    }
}

impl ReplaceTypes for ExprTryBlock {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.block.replace_types(substitutions);
    }
}

impl ReplaceTypes for ExprTuple {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.elems.iter_mut().replace_types(substitutions);
    }
}

impl ReplaceTypes for ExprUnary {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.expr.replace_types(substitutions);
    }
}

impl ReplaceTypes for ExprUnsafe {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.block.replace_types(substitutions);
    }
}

impl ReplaceTypes for ExprWhile {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.cond.replace_types(substitutions);
        self.body.replace_types(substitutions);
    }
}

impl ReplaceTypes for ExprYield {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        if let Some(expr) = &mut self.expr {
            expr.replace_types(substitutions);
        }
    }
}

impl ReplaceTypes for Punctuated<FieldValue, Comma> {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.iter_mut().replace_types(substitutions);
    }
}

impl ReplaceTypes for FieldValue {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.expr.replace_types(substitutions);
    }
}

impl ReplaceTypes for Arm {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.pat.replace_types(substitutions);
        if let Some((_, guard)) = &mut self.guard {
            guard.replace_types(substitutions);
        }
        self.body.replace_types(substitutions);
    }
}

impl ReplaceTypes for AssocType {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        if let Some(generics) = &mut self.generics {
            generics.replace_types(substitutions);
        }
        self.ty.replace_types(substitutions);
    }
}

impl ReplaceTypes for AssocConst {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        if let Some(generics) = &mut self.generics {
            generics.replace_types(substitutions);
        }
        self.value.replace_types(substitutions);
    }
}

impl ReplaceTypes for Constraint {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.bounds.replace_types(substitutions);
        if let Some(generics) = &mut self.generics {
            generics.replace_types(substitutions);
        }
    }
}

impl ReplaceTypes for Stmt {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        match self {
            Stmt::Local(local_stmt) => local_stmt.replace_types(substitutions),
            Stmt::Item(item_stmt) => item_stmt.replace_types(substitutions),
            Stmt::Expr(expr_stmt, _) => expr_stmt.replace_types(substitutions),
            // TODO: Add support for replacing within macros
            Stmt::Macro(_) => {}
        }
    }
}

impl ReplaceTypes for Local {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.pat.replace_types(substitutions);

        if let Some(init) = &mut self.init {
            init.replace_types(substitutions);
        }
    }
}

impl ReplaceTypes for LocalInit {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        if let Some((_, expr)) = &mut self.diverge {
            expr.replace_types(substitutions);
        }
        self.expr.replace_types(substitutions);
    }
}

impl ReplaceTypes for Item {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        match self {
            Item::Const(const_item) => const_item.replace_types(substitutions),
            Item::Enum(enum_item) => enum_item.replace_types(substitutions),
            Item::ExternCrate(_) => {}
            Item::Fn(fn_item) => fn_item.replace_types(substitutions),
            Item::ForeignMod(_) => {}
            Item::Impl(impl_item) => impl_item.replace_types(substitutions),
            // TODO: Add support for replacing within macros
            Item::Macro(_) => {}
            Item::Mod(mod_item) => mod_item.replace_types(substitutions),
            Item::Static(static_item) => static_item.replace_types(substitutions),
            Item::Struct(struct_item) => struct_item.replace_types(substitutions),
            Item::Trait(trait_item) => trait_item.replace_types(substitutions),
            Item::TraitAlias(trait_alias_item) => trait_alias_item.replace_types(substitutions),
            Item::Type(type_item) => type_item.replace_types(substitutions),
            Item::Union(union_item) => union_item.replace_types(substitutions),
            Item::Use(_) => {}
            Item::Verbatim(_) => {}
            _ => {}
        }
    }
}

impl ReplaceTypes for ItemStatic {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.expr.replace_types(substitutions);
        self.ty.replace_types(substitutions);
    }
}

impl ReplaceTypes for ItemStruct {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.generics.replace_types(substitutions);
        self.fields.replace_types(substitutions);
    }
}

impl ReplaceTypes for ItemTrait {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.generics.replace_types(substitutions);
        self.items.iter_mut().replace_types(substitutions);
        self.supertraits.iter_mut().replace_types(substitutions);
    }
}

impl ReplaceTypes for ItemTraitAlias {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.bounds.iter_mut().replace_types(substitutions);
        self.generics.replace_types(substitutions);
    }
}

impl ReplaceTypes for ItemType {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.generics.replace_types(substitutions);
        self.ty.replace_types(substitutions);
    }
}

impl ReplaceTypes for ItemUnion {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.fields.replace_types(substitutions);
        self.generics.replace_types(substitutions);
    }
}

impl ReplaceTypes for TraitItem {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        match self {
            TraitItem::Const(trait_const) => trait_const.replace_types(substitutions),
            TraitItem::Fn(trait_fn) => trait_fn.replace_types(substitutions),
            TraitItem::Type(trait_type) => trait_type.replace_types(substitutions),
            // TODO: Add support for replacing within macros
            TraitItem::Macro(_) => {}
            TraitItem::Verbatim(_) => {}
            _ => {}
        }
    }
}

impl ReplaceTypes for TraitItemConst {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        if let Some((_, expr)) = &mut self.default {
            expr.replace_types(substitutions);
        }

        self.generics.replace_types(substitutions);
        self.ty.replace_types(substitutions);
    }
}

impl ReplaceTypes for TraitItemFn {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        if let Some(default) = &mut self.default {
            default.replace_types(substitutions);
        }

        self.sig.replace_types(substitutions);
    }
}

impl ReplaceTypes for TraitItemType {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.bounds.iter_mut().replace_types(substitutions);
        if let Some((_, default)) = &mut self.default {
            default.replace_types(substitutions);
        }
        self.generics.replace_types(substitutions);
    }
}

impl ReplaceTypes for ItemConst {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.expr.replace_types(substitutions);
        self.generics.replace_types(substitutions);
        self.ty.replace_types(substitutions);
    }
}

impl ReplaceTypes for ItemEnum {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.generics.replace_types(substitutions);
        self.variants.replace_types(substitutions);
    }
}

impl ReplaceTypes for ItemFn {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.block.replace_types(substitutions);
        self.sig.replace_types(substitutions);
    }
}

impl ReplaceTypes for Signature {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.generics.replace_types(substitutions);
        self.inputs.replace_types(substitutions);
        self.output.replace_types(substitutions);
    }
}

impl ReplaceTypes for Punctuated<FnArg, Comma> {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.iter_mut().replace_types(substitutions);
    }
}

impl ReplaceTypes for FnArg {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        match self {
            FnArg::Receiver(receiver) => receiver.replace_types(substitutions),
            FnArg::Typed(pat_type) => pat_type.replace_types(substitutions),
        }
    }
}

impl ReplaceTypes for Receiver {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.ty.replace_types(substitutions);
    }
}

impl ReplaceTypes for ItemImpl {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.generics.replace_types(substitutions);
        self.self_ty.replace_types(substitutions);

        if let Some((_, path, _)) = &mut self.trait_ {
            path.replace_types(substitutions);
        }

        self.items.iter_mut().replace_types(substitutions);
    }
}

impl ReplaceTypes for ImplItem {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        match self {
            ImplItem::Const(const_item) => const_item.replace_types(substitutions),
            ImplItem::Fn(fn_item) => fn_item.replace_types(substitutions),
            ImplItem::Type(type_item) => type_item.replace_types(substitutions),
            // TODO: Add support for replacing within macros
            ImplItem::Macro(_) => {}
            ImplItem::Verbatim(_) => {}
            _ => {}
        }
    }
}

impl ReplaceTypes for ImplItemConst {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.expr.replace_types(substitutions);
        self.generics.replace_types(substitutions);
        self.ty.replace_types(substitutions);
    }
}

impl ReplaceTypes for ImplItemFn {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.block.replace_types(substitutions);
        self.sig.replace_types(substitutions);
    }
}

impl ReplaceTypes for ImplItemType {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.generics.replace_types(substitutions);
        self.ty.replace_types(substitutions);
    }
}

impl ReplaceTypes for ItemMod {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        if let Some((_, content)) = &mut self.content {
            content.iter_mut().replace_types(substitutions);
        }
    }
}

impl ReplaceTypes for TypeParen {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.elem.replace_types(substitutions);
    }
}

impl ReplaceTypes for TypeParam {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.bounds.replace_types(substitutions);
    }
}

impl ReplaceTypes for Punctuated<Variant, Comma> {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.iter_mut().replace_types(substitutions);
    }
}

impl ReplaceTypes for Variant {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.fields.replace_types(substitutions);

        if let Some((_, discriminant)) = &mut self.discriminant {
            discriminant.replace_types(substitutions);
        }
    }
}

impl ReplaceTypes for Fields {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        match self {
            Fields::Named(fields_named) => fields_named.replace_types(substitutions),
            Fields::Unnamed(fields_unnamed) => fields_unnamed.replace_types(substitutions),
            Fields::Unit => {}
        }
    }
}

impl ReplaceTypes for FieldsNamed {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.named.replace_types(substitutions);
    }
}

impl ReplaceTypes for Punctuated<Field, Comma> {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.iter_mut().replace_types(substitutions);
    }
}

impl ReplaceTypes for FieldsUnnamed {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.unnamed.replace_types(substitutions);
    }
}

impl ReplaceTypes for Field {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.ty.replace_types(substitutions);
    }
}

impl ReplaceTypes for Generics {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.params.iter_mut().replace_types(substitutions);

        if let Some(where_clause) = &mut self.where_clause {
            where_clause.replace_types(substitutions);
        }
    }
}

impl ReplaceTypes for Punctuated<GenericParam, Comma> {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.iter_mut().replace_types(substitutions);
    }
}

impl ReplaceTypes for GenericParam {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        match self {
            GenericParam::Lifetime(_) => {}
            GenericParam::Type(generic_type) => generic_type.replace_types(substitutions),
            GenericParam::Const(generic_const) => generic_const.replace_types(substitutions),
        }
    }
}

impl ReplaceTypes for ConstParam {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        if let Some(default) = &mut self.default {
            default.replace_types(substitutions);
        }

        self.ty.replace_types(substitutions);
    }
}
impl ReplaceTypes for WhereClause {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.predicates.replace_types(substitutions);
    }
}

impl ReplaceTypes for Punctuated<WherePredicate, Comma> {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.iter_mut().replace_types(substitutions);
    }
}

impl ReplaceTypes for WherePredicate {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        match self {
            WherePredicate::Lifetime(_) => {}
            WherePredicate::Type(where_type) => where_type.replace_types(substitutions),
            _ => {}
        }
    }
}

impl ReplaceTypes for PredicateType {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.bounded_ty.replace_types(substitutions);
        self.bounds.replace_types(substitutions);
    }
}

impl ReplaceTypes for Block {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.stmts.iter_mut().replace_types(substitutions);
    }
}

impl ReplaceTypes for Pat {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        match self {
            Pat::Const(const_pat) => const_pat.replace_types(substitutions),
            Pat::Ident(ident_pat) => ident_pat.replace_types(substitutions),
            Pat::Lit(_) => {}
            // TODO: Add support for replacing within macros
            Pat::Macro(_) => {}
            Pat::Or(or_pat) => or_pat.replace_types(substitutions),
            Pat::Paren(paren_pat) => paren_pat.replace_types(substitutions),
            Pat::Path(path_pat) => path_pat.replace_types(substitutions),
            Pat::Range(range_pat) => range_pat.replace_types(substitutions),
            Pat::Reference(reference_pat) => reference_pat.replace_types(substitutions),
            Pat::Rest(_) => {}
            Pat::Slice(slice_pat) => slice_pat.replace_types(substitutions),
            Pat::Struct(struct_pat) => struct_pat.replace_types(substitutions),
            Pat::Tuple(tuple_pat) => tuple_pat.replace_types(substitutions),
            Pat::TupleStruct(tuple_struct_pat) => tuple_struct_pat.replace_types(substitutions),
            Pat::Type(type_pat) => type_pat.replace_types(substitutions),
            Pat::Verbatim(_) => {}
            Pat::Wild(_) => {}
            _ => {}
        }
    }
}

impl ReplaceTypes for PatIdent {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        if let Some((_, pat)) = &mut self.subpat {
            pat.replace_types(substitutions);
        }
    }
}

impl ReplaceTypes for PatOr {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.cases.iter_mut().replace_types(substitutions);
    }
}

impl ReplaceTypes for PatParen {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.pat.replace_types(substitutions);
    }
}

impl ReplaceTypes for PatSlice {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.elems.iter_mut().replace_types(substitutions);
    }
}

impl ReplaceTypes for PatStruct {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        if let Some(qself) = &mut self.qself {
            qself.replace_types(substitutions);
        }

        self.path.replace_types(substitutions);
        self.fields.iter_mut().replace_types(substitutions);
    }
}
impl ReplaceTypes for FieldPat {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.pat.replace_types(substitutions);
    }
}
impl ReplaceTypes for PatReference {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.pat.replace_types(substitutions);
    }
}

impl ReplaceTypes for PatTuple {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.elems.iter_mut().replace_types(substitutions);
    }
}

impl ReplaceTypes for PatTupleStruct {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        if let Some(qself) = &mut self.qself {
            qself.replace_types(substitutions);
        }
        self.path.replace_types(substitutions);
        self.elems.iter_mut().replace_types(substitutions);
    }
}

impl ReplaceTypes for PatType {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.pat.replace_types(substitutions);
        self.ty.replace_types(substitutions);
    }
}

impl ReplaceTypes for TypePath {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        if let Some(substitution) = substitutions.get(&self) {
            *self = substitution.to_owned();
        } else {
            if let Some(qself) = &mut self.qself {
                qself.replace_types(substitutions);
            }

            self.path.replace_types(substitutions);
        }
    }
}

impl ReplaceTypes for QSelf {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.ty.replace_types(substitutions);
    }
}

impl ReplaceTypes for TypePtr {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.elem.replace_types(substitutions);
    }
}

impl ReplaceTypes for TypeReference {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.elem.replace_types(substitutions);
    }
}

impl ReplaceTypes for TypeSlice {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.elem.replace_types(substitutions);
    }
}

impl ReplaceTypes for TypeTraitObject {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.bounds.replace_types(substitutions);
    }
}

impl ReplaceTypes for TypeTuple {
    fn replace_types(&mut self, substitutions: &HashMap<TypePath, TypePath>) {
        self.elems.replace_types(substitutions);
    }
}
