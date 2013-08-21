# As vim user, I prefer emacs mode:
bindkey -e
# Home, end
bindkey -e '\eOH' beginning-of-line
bindkey -e '\eOF' end-of-line
# Ctrl+left/right
bindkey -e '\e[1;5D' backward-word
bindkey -e '\e[1;5C' forward-word

# Completion settings:
zstyle ':completion:*' completer _complete _approximate
autoload -Uz compinit
compinit


# History settings:
HISTFILE=~/.histfile
HISTSIZE=5000
SAVEHIST=5000


# Source environment variables:
source "$DOTFILES"/shell/env


# Prompt:
# For more awesome prompts:
setopt PROMPT_SUBST

function _prompt_minimalist() {
	PS1="%B%#%b "
}

function _prompt_bash_like() {
	PS1="%B%F{green}%n@%m%f %F{blue%~ %#%f%b "
}

_prompt_bash_like
