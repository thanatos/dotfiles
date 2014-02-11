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


" Colorsheme
if has("gui_running")
	colorscheme darkspectrum
else
	colorscheme delek
endif


" GUI settings
if has("gui_running")
	if has("gui_win32")
		set guifont=Consolas:h9:cANSI
		set nocompatible
	else
		set guifont=Deja\ Vu\ Sans\ Mono\ 9
	endif
endif


" Python files use 4-space tabs.
autocmd Filetype python setlocal expandtab
