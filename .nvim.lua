vim.o.title = true
vim.o.titlestring = "Pando"

-- Disable autochdir
vim.o.autochdir = false

-- cd to this lua file location
local file_path = debug.getinfo(1, "S").source:sub(2)
local dir_path = file_path:match("(.*[/\\])") or "./"
vim.cmd("cd " .. dir_path)
