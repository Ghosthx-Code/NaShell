# NaShell
A shell in `rust` and `lua` 
## Config
```bash

git clone https://github.com/Ghosthx-Code/NsShell
cd NaShell
mv binary/shell .
sudo mv shell /usr/local/bin/shell
chsh -s /usr/local/bin/shell

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

-- &(GIT) to get the git branch
-- &(DIR) to get dir
-- &(RUST) to get rust verion
-- &(TIME) to get time
-- &(HOST) to get local ip
-- &(USER) to get user name
-- &(SYSTEM) to get cpu and memory usage
shell:alias("nv", "nvim .") -- sets a a command

shell:addr("nv", "nvim") -- type nv and it does nvim

```
