macro_rules! lua_fn {
    ($(pub unsafe fn $name:ident($targ:ident: &mut ProxyTransaction, $larg:ident: &mut $typ:ty) -> Result<i32, LuaError> $code:block)+) => (
        $(
            pub unsafe extern "C" fn $name($larg: *mut ::lua::raw::lua_State) -> ::libc::c_int {
                let mut $larg = &mut ::lua::ExternState::from_lua_State($larg);

                $larg.getglobal("trans");

                if !$larg.islightuserdata(-1) {
                    $larg.errorstr("Corrupted transaction");
                    return 1;
                }

                let trans_ptr = $larg.touserdata(-1);
                let $targ = &mut *(trans_ptr as *mut ProxyTransaction);

                return match inner($targ, &mut $larg) {
                    Ok(i) => i,
                    Err(err) => {
                        err.serialize($larg);
                        1
                    }
                } as ::libc::c_int;

                unsafe fn inner($targ: &mut ProxyTransaction, $larg: &mut $typ) -> Result<i32, LuaError> $code
            }
        )+
    )
}