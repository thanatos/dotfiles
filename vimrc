" ─── Basic settings ──────────────────────────────────────────────────────────
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


" ─── Color scheme ────────────────────────────────────────────────────────────
" Colorscheme
if has("gui_running")
	colorscheme darkspectrum
else
	colorscheme slate
endif

" Visibly show where the 80th column is:
"let &colorcolumn=join(range(81,999),",")
let &colorcolumn=81
highlight ColorColumn ctermbg=235 guibg=#402727
" …but don't show it on files that don't make sense:
autocmd FileType man setlocal colorcolumn&
autocmd FileType netrw setlocal colorcolumn&
autocmd FileType nerdtree setlocal colorcolumn&
autocmd FileType conque_term setlocal colorcolumn&

" GUI settings
if has("gui_running")
	if has("gui_win32")
		set guifont=Consolas:h9:cANSI
		set nocompatible
	elseif has("gui_macvim")
		set guifont=DejaVu\ Sans\ Mono\ for\ Powerline:h12
	else
		set guifont=DejaVu\ Sans\ Mono\ for\ Powerline\ 9
	endif
endif


" ─── autocmd ─────────────────────────────────────────────────────────────────
" Python files use 4-space tabs.
autocmd FileType python setlocal expandtab foldmethod=indent
autocmd FileType python normal zR

" YAML files require spaces for indentation:
autocmd FileType yaml setlocal expandtab

" *.rs is Rust.
autocmd BufNewFile,BufRead *.rs setf rust
autocmd FileType rust setlocal expandtab foldmethod=syntax colorcolumn=101
autocmd FileType rust normal zR

" When composing a commit message, help myself with spelling:
autocmd FileType gitcommit setlocal spell

" Use spaces for indentation in ReST:
autocmd FileType rst setlocal expandtab


" ─── Add third-party code to runtimepath ─────────────────────────────────────
function Plugin(name)
	"echom "Loading plugin " . a:name
	let &runtimepath = "~/.vim/bundle/" . a:name . "," . &runtimepath
	"let &runtimepath = &runtimepath . ",~/.vim/bundle/" . a:name
endfunction
command -nargs=1 Plugin call Plugin(<f-args>)

function UpdatePlugin(name)
	execute "!git -C ~/.vim/bundle/" . a:name . " fetch --depth=1"
	execute "!git -C ~/.vim/bundle/" . a:name . " reset --hard @{u}"
endfunction

Plugin vim-airline
Plugin nerdtree
Plugin rust.vim
Plugin vim-toml


" ─── Misc ────────────────────────────────────────────────────────────────────

" Manpages in VIM:
runtime ftplugin/man.vim

" airline works better if we always show the status:
set laststatus=2
" and it looks nicer if we have the patched font:
let g:airline_powerline_fonts = 1
