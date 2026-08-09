<script setup lang="ts">
import { ref, onMounted } from 'vue'
import AppHeader from '@/components/layout/AppHeader.vue'
import { getMockMagmas } from '@/mock/magmasMock'

interface MagmaCard {
  id: string
  url: string
  title: string
  thumbnail: string
  loading: boolean
  error: boolean
}

const magmas = ref<MagmaCard[]>([])

/**
 * Extrai o id do canvas a partir da URL do magma.
 * Ex: https://magma.com/d/U5Qo6tcUP1 -> "U5Qo6tcUP1"
 */
function extractCanvasId(url: string): string {
  try {
    const { pathname } = new URL(url)
    const segments = pathname.split('/').filter(Boolean)
    return segments[segments.length - 1] ?? ''
  } catch {
    return ''
  }
}

/**
 * Monta a url da thumbnail a partir do id do canvas.
 */
function buildThumbnailUrl(id: string): string {
  return `https://magma.com/${id}.thumbnail.png`
}

/**
 * Remove o sufixo " | Magma" do título da página.
 */
function stripBrandSuffix(rawTitle: string): string {
  return rawTitle.replace(/\s*\|\s*Magma\s*$/i, '').trim()
}

/**
 * Busca o <title> da página do magma.
 *
 * OBS: buscar o HTML de magma.com diretamente do navegador só funciona
 * se o domínio liberar CORS para essa rota. Se não for o caso, troque
 * este fetch por uma chamada ao seu próprio backend, algo como:
 *   GET /api/magma/title?url=<url>
 * que faz o fetch no servidor e devolve só o título já tratado.
 */
async function fetchMagmaTitle(url: string): Promise<string> {
  const response = await fetch(url)
  const html = await response.text()
  const doc = new DOMParser().parseFromString(html, 'text/html')
  const rawTitle = doc.querySelector('title')?.textContent ?? ''
  return stripBrandSuffix(rawTitle)
}

onMounted(async () => {
  const urls = getMockMagmas()

  magmas.value = urls.map((url) => {
    const id = extractCanvasId(url)
    return {
      id,
      url,
      title: 'Carregando…',
      thumbnail: buildThumbnailUrl(id),
      loading: true,
      error: false,
    }
  })

  await Promise.all(
    magmas.value.map(async (magma) => {
      try {
        magma.title = await fetchMagmaTitle(magma.url)
      } catch (err) {
        console.error('Falha ao buscar título do magma', magma.url, err)
        magma.title = 'Sem título'
        magma.error = true
      } finally {
        magma.loading = false
      }
    }),
  )
})
</script>

<template>
  <div class="magmas-page">
    <AppHeader />

    <main class="magmas-page__content">
      <h1 class="magmas-page__title">Meus Magmas</h1>

      <div class="magmas-grid">
        <article
          v-for="magma in magmas"
          :key="magma.id"
          class="magma-card card"
        >
          <a
            :href="magma.url"
            target="_blank"
            rel="noopener noreferrer"
            class="magma-card__title"
            :class="{ 'magma-card__title--loading': magma.loading }"
          >
            {{ magma.title }}
          </a>

          <a
            :href="magma.url"
            target="_blank"
            rel="noopener noreferrer"
            class="magma-card__thumb-wrap"
          >
            <img
              :src="magma.thumbnail"
              :alt="magma.title"
              class="magma-card__thumb"
              loading="lazy"
            />
          </a>
        </article>
      </div>

      <p v-if="!magmas.length" class="magmas-page__empty">
        Nenhum magma encontrado.
      </p>
    </main>
  </div>
</template>

<style scoped>
.magmas-page {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}

.magmas-page__content {
  flex: 1;
  max-width: var(--max-content-width);
  width: 100%;
  margin: 0 auto;
  padding: 28px 32px 64px;
}

.magmas-page__title {
  font-size: 1.75rem;
  margin-bottom: 20px;
}

.magmas-page__empty {
  color: var(--color-text-faint);
  margin-top: 24px;
}

.magmas-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
  gap: 20px;
}

.magma-card {
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding: 16px;
  overflow: hidden;
}

.magma-card__title {
  font-family: var(--font-display);
  font-size: 1rem;
  color: var(--color-text);
  line-height: 1.3;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.magma-card__title:hover {
  color: var(--color-brand);
  text-decoration: underline;
}

.magma-card__title--loading {
  color: var(--color-text-faint);
  font-style: italic;
}

.magma-card__thumb-wrap {
  display: block;
  border-radius: var(--radius-md);
  overflow: hidden;
  background: var(--color-bg-elevated);
  aspect-ratio: 16 / 10;
}

.magma-card__thumb {
  width: 100%;
  height: 100%;
  object-fit: cover;
  display: block;
  transition: transform var(--transition-base);
}

.magma-card__thumb-wrap:hover .magma-card__thumb {
  transform: scale(1.03);
}

@media (max-width: 640px) {
  .magmas-page__content {
    padding-left: 18px;
    padding-right: 18px;
  }

  .magmas-grid {
    grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
    gap: 14px;
  }
}
</style>