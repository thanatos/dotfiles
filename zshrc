# Completion settings:
zstyle ':completion:*' completer _complete _approximate
autoload -Uz compinit
compinit


# As vim user, I prefer emacs mode:
bindkey -e
# Home, end
bindkey -e '\eOH' beginning-of-line
bindkey -e '\eOF' end-of-line
# Ctrl+left/right
bindkey -e '\e[1;5D' backward-word
bindkey -e '\e[1;5C' forward-word
# Delete
bindkey -e '\e[3~' delete-char
# Edit line in $EDITOR:
autoload edit-command-line
zle -N edit-command-line
bindkey -e '^Xe' edit-command-line


# History settings:
HISTFILE=~/.histfile
HISTSIZE=5000
SAVEHIST=5000


# Source environment variables:
source "$DOTFILES"/shell/env


# Source other stuff:
source "$DOTFILES"/shell/zsh/git
source "$DOTFILES"/shell/zsh/prompt-alpha


# Prompt:
# For more awesome prompts:
setopt PROMPT_SUBST

function _prompt_minimalist() {
	PS1="%B%#%b "
}

function _prompt_bash_like() {
	PS1="%B%F{green}%n@%m%f %F{blue%~ %#%f%b "
}

_prompt_alpha
