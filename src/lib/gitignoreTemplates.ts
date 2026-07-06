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
};
