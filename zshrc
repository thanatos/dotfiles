# Completion settings:
zstyle ':completion:*' completer _complete _approximate
autoload -Uz compinit
compinit


# As vim user, I prefer emacs mode:
bindkey -e
# Home, end
bindkey -e '\e[H' beginning-of-line
bindkey -e '\e[F' end-of-line
# Ctrl+left/right
bindkey -e '\e[1;5D' backward-word
bindkey -e '\e[1;5C' forward-word
# Delete
bindkey -e '\e[3~' delete-char
# Edit line in $EDITOR:
autoload edit-command-line
zle -N edit-command-line
bindkey -e '^Xe' edit-command-line

# Just kidding:
# Ctrl+left/right
bindkey -v '\e[1;5D' backward-word
bindkey -a '\e[1;5D' backward-word
bindkey -v '\e[1;5C' forward-word
bindkey -a '\e[1;5C' forward-word
# Home, end
bindkey -v '\e[H' beginning-of-line
bindkey -a '\e[H' beginning-of-line
bindkey -v '\e[F' end-of-line
bindkey -a '\e[F' end-of-line
# But bind some of the emacs keys when in insert mode
bindkey -v '^A' beginning-of-line
bindkey -v '^E' end-of-line
bindkey -v '^W' backward-kill-word
bindkey -v '^K' kill-line
bindkey -v '^V' quoted-insert
bindkey -v '^Xe' edit-command-line
bindkey -a '^Xe' edit-command-line
# By default, zle uses the literal-vi backspace behavior. `vim` hasn't
# done this in eons.
bindkey -v '^H' backward-delete-char
bindkey -a '^H' backward-delete-char
bindkey -v '^?' backward-delete-char
bindkey -a '^?' backward-delete-char


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

if [[ "${ZSH_NO_RUST_PROMPT-}" != "" ]]; then
	_prompt_alpha_setup
else
	module_path="$DOTFILES/zsh-prompt-in-rust/target/debug" zmodload libzsh_prompt_in_rust
	PS1='$(_rust-prompt-alpha "$?" "royiv" "vi")'
	eval 'preexec() { _rust-prompt-alpha_pre-exec }'
	eval 'precmd() { _rust-prompt-alpha_pre-cmd }'
fi
