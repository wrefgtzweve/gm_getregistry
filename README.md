# gm_getregistry
A simple Garry's Mod Lua module that exposes the debug.getregistry table to Lua.

The main goal of this module is to provide access to the Lua registry table, which contains internal Lua state and metatables.

## Example usage
Access the registry table from Lua:
```lua
require( "getregistry" )

local registry = getregistry.Get()
print("Registry table:", registry)

-- Access metatables and other internal Lua data
for k, v in pairs(registry) do
    print(k, v)
end
```

## Credits
- [gmod-rs](https://github.com/WilliamVenner/gmod-rs) for the rust gmod bindings
- [goobie-sql](https://github.com/Srlion/goobie-sql/blob/master/.github/workflows/build-mysql.yml) for part of the build workflow
