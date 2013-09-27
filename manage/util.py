import sys

from . import term


def abort_with_error(error):
    sys.stderr.write(
        '{bold}{red}Error:{reset} {}\n'.format(
            error, bold=term.BOLD, reset=term.RESET, red=term.RED))
    sys.exit(1)
