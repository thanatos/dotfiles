" Line numbers
set number
" Tab width (4 columns, actual tabs)
set tabstop=4
set shiftwidth=4
set noexpandtab
" Smart indent
set smartindent
" Highlight search
set hlsearch
" Syntax highlighting
syntax enable


" Colorscheme
if has("gui_running")
	colorscheme darkspectrum
else
	colorscheme delek
endif


" Visibly show where the 80th column is:
let &colorcolumn=join(range(81,999),",")
highlight ColorColumn ctermbg=235 guibg=#402727
" …but don't show it on files that don't make sense:
autocmd Filetype man setlocal colorcolumn&
autocmd Filetype netrw setlocal colorcolumn&
autocmd Filetype nerdtree setlocal colorcolumn&
autocmd Filetype conque_term setlocal colorcolumn&


" GUI settings
if has("gui_running")
	if has("gui_win32")
		set guifont=Consolas:h9:cANSI
		set nocompatible
	elseif has("gui_macvim")
		set guifont=DejaVu\ Sans\ Mono\ for\ Powerline:h12
	else
		set guifont=Deja\ Vu\ Sans\ Mono\ 9
	endif
endif


" Python files use 4-space tabs.
autocmd Filetype python setlocal expandtab

" YAML files require spaces for indentation:
autocmd Filetype yaml setlocal expandtab


" Manpages in VIM:
runtime ftplugin/man.vim

" NERD tree:
set runtimepath+=~/.vim/bundle/nerdtree
