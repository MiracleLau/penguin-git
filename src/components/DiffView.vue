<script setup lang="ts">
import type { DiffRow } from "../lib/diffParser";

defineProps<{ rows: DiffRow[] }>();

function rowClass(row: DiffRow): string {
  if (row.type === 'context' && row.oldContent == null) return 'diff-hunk'
  return `diff-row--${row.type}`
}

function isSeparator(row: DiffRow): boolean {
  return row.type === 'context' && row.oldContent == null
}
</script>

<template>
  <div class="diff-view">
    <div v-if="rows.length === 0" class="diff-empty">无差异</div>
    <div v-for="(row, i) in rows" :key="i" class="diff-row" :class="rowClass(row)">
      <template v-if="isSeparator(row)">
        <div class="diff-hunk-line">{{ row.content }}</div>
      </template>
      <template v-else>
        <div class="diff-gutter">{{ row.oldLineNum ?? '' }}</div>
        <pre class="diff-code">{{ row.oldContent }}</pre>
        <div class="diff-gutter">{{ row.newLineNum ?? '' }}</div>
        <pre class="diff-code">{{ row.newContent }}</pre>
      </template>
    </div>
  </div>
</template>

<style scoped>
.diff-view {
  font-family: 'SF Mono', 'Fira Code', 'Cascadia Code', 'JetBrains Mono', Menlo, monospace;
  font-size: 12px;
  line-height: 1.6;
  max-height: 60vh;
  overflow-y: auto;
  overflow-x: auto;
  white-space: pre;
}

.diff-row {
  display: flex;
  min-height: 19.2px;
}

.diff-row--context { background: transparent; }
.diff-row--delete { background: rgba(248, 81, 73, 0.12); }
.diff-row--add { background: rgba(14, 173, 79, 0.12); }
.diff-row--modify .diff-gutter:first-of-type,
.diff-row--modify .diff-code:first-of-type { background: rgba(248, 81, 73, 0.12); }
.diff-row--modify .diff-gutter:last-of-type,
.diff-row--modify .diff-code:last-of-type { background: rgba(14, 173, 79, 0.12); }

.diff-hunk {
  display: flex;
}

.diff-hunk-line {
  padding: 2px 12px;
  opacity: 0.5;
  font-style: italic;
  background: rgba(128, 128, 128, 0.06);
  width: 100%;
}

.diff-gutter {
  width: 52px;
  min-width: 52px;
  text-align: right;
  padding: 0 8px;
  color: rgba(128, 128, 128, 0.6);
  user-select: none;
  box-sizing: border-box;
}

.diff-code {
  margin: 0;
  padding: 0 8px 0 4px;
  white-space: pre;
  overflow: hidden;
  text-overflow: ellipsis;
  min-width: 200px;
  flex: 1;
  box-sizing: border-box;
}

.diff-empty {
  padding: 24px;
  text-align: center;
  opacity: 0.4;
  font-size: 14px;
}
</style>
