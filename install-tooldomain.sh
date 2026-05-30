#!/bin/bash
set -e

USER_NAME=$(whoami)
HOME_DIR="$HOME"

SCRIPTS_DIR="$HOME_DIR/scripts"
BIN_DIR="$HOME_DIR/bin"

BREW_DIR="$HOME_DIR/brew"
BREW_PREFIX="$HOME_DIR/local"

SCRIPT_NAME="tooldomain.sh"
SCRIPT_DEST="$SCRIPTS_DIR/$SCRIPT_NAME"
DOWNLOAD_URL="https://recursos.colhost.com/tooldomain/tooldomain.data"

echo "== Instalando ToolDomain (brew local, sin contraseña) =="
echo "Usuario: $USER_NAME"

# --------------------------------------------------
# 1. Crear directorios base
# --------------------------------------------------
mkdir -p "$SCRIPTS_DIR" "$BIN_DIR" "$BREW_PREFIX"

# --------------------------------------------------
# 2. Instalar Homebrew local (solo si no existe)
# --------------------------------------------------
if [[ ! -x "$BREW_DIR/bin/brew" ]]; then
  echo "Instalando Homebrew en HOME (modo usuario)..."
  cd "$HOME_DIR"
  git clone https://github.com/Homebrew/brew "$BREW_DIR"
fi

# --------------------------------------------------
# 3. Configurar entorno Brew (solo usuario)
# --------------------------------------------------
export HOMEBREW_PREFIX="$BREW_PREFIX"
export HOMEBREW_CELLAR="$BREW_PREFIX/Cellar"
export HOMEBREW_REPOSITORY="$BREW_DIR"
export PATH="$BREW_DIR/bin:$BREW_PREFIX/bin:$PATH"

# Persistir en zshrc si no existe
if ! grep -q "HOMEBREW_PREFIX=$BREW_PREFIX" "$HOME_DIR/.zshrc" 2>/dev/null; then
  {
    echo ""
    echo "# Homebrew local (sin sudo)"
    echo "export HOMEBREW_PREFIX=\"$BREW_PREFIX\""
    echo "export HOMEBREW_CELLAR=\"$BREW_PREFIX/Cellar\""
    echo "export HOMEBREW_REPOSITORY=\"$BREW_DIR\""
    echo "export PATH=\"$BREW_DIR/bin:$BREW_PREFIX/bin:\$PATH\""
  } >> "$HOME_DIR/.zshrc"
fi

# --------------------------------------------------
# 4. Instalar dependencias necesarias (sin sudo)
# --------------------------------------------------
echo "Instalando dependencias con brew local..."
brew update

brew list whois >/dev/null 2>&1 || brew install whois
brew list bind  >/dev/null 2>&1 || brew install bind
brew list curl  >/dev/null 2>&1 || brew install curl
brew list openssl >/dev/null 2>&1 || brew install openssl

# --------------------------------------------------
# 5. Descargar ToolDomain
# --------------------------------------------------
echo "Descargando ToolDomain..."
curl -fsSL "$DOWNLOAD_URL" -o "$SCRIPT_DEST"
chmod +x "$SCRIPT_DEST"

# --------------------------------------------------
# 6. Crear enlace ejecutable
# --------------------------------------------------
ln -sf "$SCRIPT_DEST" "$BIN_DIR/tooldomain"

# --------------------------------------------------
# 7. Asegurar PATH del bin del usuario
# --------------------------------------------------
if ! grep -q 'export PATH="$HOME/bin:$PATH"' "$HOME_DIR/.zshrc" 2>/dev/null; then
  echo 'export PATH="$HOME/bin:$PATH"' >> "$HOME_DIR/.zshrc"
fi

export PATH="$HOME/bin:$PATH"

# --------------------------------------------------
# 8. Final
# --------------------------------------------------
echo "== Instalación completada correctamente =="
echo ""
echo "Cierra y abre la terminal o ejecuta:"
echo "  source ~/.zshrc"
echo ""
echo "Luego usa:"
echo "  tooldomain dominio.com"
