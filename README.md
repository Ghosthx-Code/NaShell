# NaShell
A shell in `rust` and `lua` 
## Config
```bash

git clone https://Ghosthx-Code/NsShell
cd NaShell
mv binary/shell .
sudo mv shell /usr/local/bin/shell

cd ..
mkdir ~/.config/NaShell
touch ~/.config/NaShell/config.lua
# go to command for the code to config.lua
# Make it your shell
```

## Code-Lua
- [ ] Lua Commands
```lua
-- shell:set_prompt() set the prompt like
shell:set_prompt("&(white)[&(TIME)] &(green)[&(DIR)] &(cyan)> ") -- sets prompt

-- to get dir
shell:help() -- prints all lua code and what it does

shell:alias("nv", "nvim .") -- sets a a command

shell:addr("nv", "nvim") -- type nv and it does nvim

```
