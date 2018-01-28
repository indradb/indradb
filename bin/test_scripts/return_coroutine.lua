foo = coroutine.create(function()
end)

return foo
-- err: Lua(FromLuaConversionError { from: "thread", to: "JSON", message: None })
