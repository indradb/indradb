foo = coroutine.create(function()
end)

return foo
-- err: FromLuaConversionError { from: "thread", to: "JSON", message: None }