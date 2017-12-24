foo = coroutine.create(function()
end)

return foo
-- err: error converting Lua thread to JSON
