use name::Def;
use query_base::QueryBase;

pub trait Queries<'qs>: QueryBase + ment::Queries<'qs> {
    fn codegen_binary(self, func: Def) -> wasm::Mod {
        let ment = self.ment_func(func);
        let mut b = Builder::default();
        codegen_func(&mut b, ment);
        b.into_wasm_module()
    }
}

#[derive(Default)]
struct Builder;

fn codegen_func(b: &mut Builder, func: ment::Func) {
    todo!()
}

impl Builder {
    fn into_wasm_module(self) -> wasm::Mod {
        todo!()
    }
}
