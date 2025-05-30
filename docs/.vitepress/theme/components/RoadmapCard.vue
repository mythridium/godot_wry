<template>
  <div class="roadmap-card">
    <div class="card-header">
      <div class="card-icon" v-html="icon"></div>
      <h3 class="card-title">{{ title }}</h3>
    </div>
    <div class="card-content">
      <p class="card-description">{{ description }}</p>
      <a v-if="issueLink" :href="issueLink" class="issue-link" target="_blank">
        View issue →
      </a>
      <a
        v-if="version"
        class="version-badge"
        :href="`https://github.com/doceazedo/godot_wry/releases/tag/${version}`"
        target="_blank"
        >{{ version }}</a
      >
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  icon: string;
  title: string;
  description: string;
  issueLink?: string;
  version?: string;
}

defineProps<Props>();
</script>

<style scoped>
.roadmap-card {
  background: var(--vp-c-bg-soft);
  border: 1px solid var(--vp-c-border);
  border-radius: 12px;
  padding: 24px;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 16px;
}

.card-icon {
  font-size: 24px;
  line-height: 1;
}

.card-title {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--vp-c-text-1);
}

.card-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.card-description {
  margin: 0;
  color: var(--vp-c-text-2);
  line-height: 1.6;
  flex: 1;
}

.issue-link {
  color: var(--vp-c-brand-1);
  text-decoration: none;
  font-weight: 500;
  font-size: 14px;
  align-self: flex-start;
}

.issue-link:hover {
  text-decoration: underline;
}

.version-badge {
  background: var(--vp-button-alt-bg);
  color: white;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 500;
  align-self: flex-start;
  transition: background 0.3s ease;
}

.version-badge:hover {
  background: var(--vp-button-alt-hover-bg);
}
</style>
