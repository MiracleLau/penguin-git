export interface DiffRow {
  type: 'context' | 'add' | 'delete' | 'modify'
  oldLineNum?: number
  newLineNum?: number
  oldContent?: string
  newContent?: string
  content?: string
}

export function parseDiff(diff: string): DiffRow[] {
  const lines = diff.split('\n')
  const rows: DiffRow[] = []
  let inHunk = false
  let oldLine = 0
  let newLine = 0
  const dels: { content: string; lineNum: number }[] = []

  for (const raw of lines) {
    const line = raw

    if (!inHunk) {
      const m = line.match(/^@@ -(\d+)(?:,\d+)? \+(\d+)(?:,\d+)? @@/)
      if (m) {
        inHunk = true
        oldLine = parseInt(m[1])
        newLine = parseInt(m[2])
        rows.push({ type: 'context', content: line })
      }
      continue
    }

    if (line.startsWith('@@ ')) {
      flush()
      const m = line.match(/^@@ -(\d+)(?:,\d+)? \+(\d+)(?:,\d+)? @@/)
      if (m) {
        oldLine = parseInt(m[1])
        newLine = parseInt(m[2])
      }
      rows.push({ type: 'context', content: line })
      continue
    }

    if (line.startsWith('-')) {
      dels.push({ content: line.slice(1), lineNum: oldLine++ })
    } else if (line.startsWith('+')) {
      if (dels.length > 0) {
        const d = dels.shift()!
        rows.push({ type: 'modify', oldLineNum: d.lineNum, newLineNum: newLine++, oldContent: d.content, newContent: line.slice(1) })
      } else {
        rows.push({ type: 'add', newLineNum: newLine++, newContent: line.slice(1) })
      }
    } else if (line.startsWith(' ')) {
      flush()
      rows.push({ type: 'context', oldLineNum: oldLine++, newLineNum: newLine++, oldContent: line.slice(1), newContent: line.slice(1) })
    } else if (line.startsWith('\\')) {
      rows.push({ type: 'context', content: line })
    }
  }

  flush()
  return rows

  function flush() {
    for (const d of dels) {
      rows.push({ type: 'delete', oldLineNum: d.lineNum, oldContent: d.content })
    }
    dels.length = 0
  }
}
