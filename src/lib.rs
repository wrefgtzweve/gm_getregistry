use gmod::{gmod13_close, gmod13_open, lua_function, lua_string, lua::State, lua::LUA_REGISTRYINDEX};

#[lua_function]
unsafe fn get_registry(lua: State) -> i32 {
    lua.push_value(LUA_REGISTRYINDEX);
    1
}

#[gmod13_open]
unsafe fn gmod13_open(lua: State) -> i32 {
    lua.push_globals();
    lua.new_table();

    lua.push_function(get_registry);
    lua.set_field(-2, lua_string!("Get"));

    lua.set_field(-2, lua_string!("getregistry"));
    lua.pop();

    0
}

#[gmod13_close]
fn gmod13_close(_: State) -> i32 {
    0
}
