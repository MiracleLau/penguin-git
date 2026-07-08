export const GITIGNORE_TEMPLATES: Record<string, string> = {
  general: `# OS files
.DS_Store
Thumbs.db
Desktop.ini

# IDE
.idea/
.vscode/
*.swp
*.swo
*~
.project
.classpath
.settings/

# Logs
*.log

# Environment
.env
.env.local
.env.*.local
`,
  node: `# Node.js
node_modules/
npm-debug.log*
yarn-debug.log*
yarn-error.log*
.pnpm-debug.log*
lerna-debug.log*

# Bun
bun.lock

# Build
dist/
build/
*.tsbuildinfo
`,
  rust: `# Rust
target/
Cargo.lock
**/*.rs.bk
`,
  python: `# Python
__pycache__/
*.py[cod]
*$py.class
*.so
.Python
env/
venv/
.venv/
*.egg-info/
*.egg
.eggs/
dist/
build/
.pytest_cache/
.mypy_cache/
.ruff_cache/
`,
  go: `# Go
bin/
*.exe
*.exe~
*.dll
*.so
*.dylib
vendor/
`,
  java: `# Java
*.class
*.jar
*.war
*.nar
target/
build/
.gradle/
gradle-app.setting
!gradle-wrapper.jar
`,
  cpp: `# C/C++
*.o
*.obj
*.exe
*.out
*.app
build/
cmake-build-*/
`,
  php: `# PHP
vendor/
composer.lock
composer.phar
*.log
.phpunit.result.cache
.php_cs.cache
.php-cs-fixer.cache
.php-cs-fixer.php
.php-cs-fixer.dist.php
.php-cs-fixer.cache
.php-cs-fixer.php
.php-cs-fixer.dist.php
.phpunit.cache
phpunit.xml
.phpunit.xml
.phpunit.result.cache
.phpunit.cache
.phpunit.result.cache
.phpunit.cache
.phpunit.result.cache
.env
.env.local
.phpstorm.meta.php
_ide_helper.php
_ide_helper_models.php
.php_cs
.php_cs.dist
.php-cs-fixer
.php-cs-fixer.dist
.php-cs-fixer.cache
.php-cs-fixer.php
.php-cs-fixer.dist.php
.phpunit.cache
.phpunit.result.cache
.phpunit.xml
.phpunit.xml.dist
phpunit.xml
phpunit.xml.dist
.php_cs.cache
.php_cs.dist
`,
};
