# ==============================================================================
# ================== Customized Configurations Below ===========================
# ==============================================================================
# The configurations from this point forward are meticulously tailored to 
# enhance the user experience through customized settings and aliases. These 
# settings are designed to streamline workflow and boost efficiency within 
# the terminal environment.
# ==============================================================================

# Alias for ip command with colorful output
alias ip="ip -c"

# Display system information with fastfetch
fastfetch

# Initialize zoxide for faster directory navigation
eval "$(zoxide init --cmd cd zsh)"

# Source the cargo environment setup script
. "$HOME/.cargo/env" 
