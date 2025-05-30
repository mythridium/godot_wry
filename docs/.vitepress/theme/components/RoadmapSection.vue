<template>
  <section class="roadmap-section">
    <header class="section-header">
      <div class="section-icon" v-html="icon"></div>
      <div class="section-info">
        <h2 class="section-title">{{ title }}</h2>
        <p class="section-description">{{ description }}</p>
      </div>
    </header>
    <div class="cards-grid">
      <slot />
    </div>
  </section>
</template>

<script setup lang="ts">
interface Props {
  icon: string;
  title: string;
  description: string;
}

defineProps<Props>();
</script>

<style scoped>
.roadmap-section {
  margin-bottom: 48px;
}

.section-header {
  display: flex;
  align-items: flex-start;
  gap: 16px;
  margin-bottom: 32px;
  padding-bottom: 24px;
  border-bottom: 1px solid var(--vp-c-border);
}

.section-icon {
  font-size: 32px;
  line-height: 1;
  margin-top: 4px;
}

.section-info {
  flex: 1;
}

.section-title {
  margin: 0 0 8px 0;
  font-size: 28px;
  font-weight: 700;
  color: var(--vp-c-text-1);
}

.section-description {
  margin: 0;
  color: var(--vp-c-text-2);
  font-size: 16px;
  line-height: 1.6;
}

.cards-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
  gap: 24px;
}

@media (max-width: 768px) {
  .section-header {
    flex-direction: column;
    gap: 12px;
  }

  .section-icon {
    font-size: 28px;
  }

  .section-title {
    font-size: 24px;
  }

  .cards-grid {
    grid-template-columns: 1fr;
  }
}
</style>
