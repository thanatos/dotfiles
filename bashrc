# Use emacs keybindings.
set -o emacs

source "$DOTFILES"/shell/env
source "$DOTFILES"/shell/prompt-common


function _prompt_default() {
	PS1='\[\e[0;1;32m\]\u@\h\[\e[34m\] \w \$\[\e[0m\] '
}

function _prompt_simple() {
	local TRAILER='\[\e[34m\] \w \$\[\e[0m\] '
	local HOST_SECTION="`_prompt_short_host`"
	PS1='\[\e[0;1;32m\]\u@'"${HOST_SECTION}${TRAILER}"
}
